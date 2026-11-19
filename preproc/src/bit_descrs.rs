/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use std::num::NonZeroU8;

use anyhow::{Context, bail, ensure};
use mdbook_preprocessor::errors::{Error, Result};

use crate::Pandocs;

pub fn render_bit_descr(
    dest: &mut String,
    indentation: &str,
    opt_body: Option<&str>,
) -> Result<()> {
    use std::fmt::Write;

    let body = opt_body.context("body cannot be empty")?;
    debug_assert!(!body.starts_with(Pandocs::WHITESPACE_AND_NEWLINES));
    debug_assert!(!body.ends_with(Pandocs::WHITESPACE_AND_NEWLINES));

    let mut header = body.splitn(3, Pandocs::WHITESPACE_AND_NEWLINES);
    // Parsing.
    let width: NonZeroU8 = header
        .next()
        .expect("body should not be just whitespace!!")
        .parse()
        .context("bad width")?;
    let order_is_ascending = match header.next().context("missing bit order")? {
        "<" => true,
        ">" => false,
        bad => bail!("Bit order should be `<` or `>`, not \"{bad}\""),
    };
    let rows = header
        .next()
        .context("missing rows after bit order")?
        .trim_matches(Pandocs::WHITESPACE_AND_NEWLINES);
    debug_assert_eq!(header.next(), None); // Should be guaranteed by `splitn()`.

    // Rendering.
    dest.push_str(indentation);
    let have_names = !rows.starts_with(r#""""#);
    if have_names {
        // If names are present, add an empty cell for the name column.
        dest.push_str(
            r#"<div class="table-wrapper"><table class="bit-descrs"><thead><tr><th></th>"#,
        );
    } else {
        // Otherwise, add a class to correct the styling of the first column.
        dest.push_str(
            r#"<div class="table-wrapper"><table class="bit-descrs nameless"><thead><tr>"#,
        );
    };
    if order_is_ascending {
        for i in 0..width.get() {
            write!(dest, "<th>{i}</th>").unwrap();
        }
    } else {
        for i in (0..width.get()).rev() {
            write!(dest, "<th>{i}</th>").unwrap();
        }
    }
    dest.push_str("</tr></thead><tbody>");

    // The table's body, now...

    for row in rows.split_terminator(';') {
        dest.push_str("<tr>");

        // The row's name.
        let (row_name, after_name) =
            extract_prefix_string(row.trim_matches(Pandocs::WHITESPACE_AND_NEWLINES))
                .map_err(|_| Error::msg("missing row name"))??;
        check_unsupported_escaping(row_name)?;
        let mut to_parse = after_name.trim_start_matches(Pandocs::WHITESPACE_AND_NEWLINES);

        let have_name = !row_name.is_empty();
        ensure!(
            have_name == have_names,
            "All rows must have empty names, or none may; but not both",
        );
        if have_name {
            write!(dest, "<td><strong>{row_name}</strong></td>").unwrap();
        }

        // The row proper.
        let mut next_idx = 0;
        let mut nb_skipped_bits = |start_idx, new_next_idx| {
            NonZeroU8::new(start_idx - std::mem::replace(&mut next_idx, new_next_idx))
        };
        while !to_parse.is_empty() {
            // Parsing.
            let (key, after_key) = to_parse
                .split_once(':')
                .context("Rows' items must be `<key>:\"<name>\"`")?;
            let (name, rest) = extract_prefix_string(after_key).map_err(|_| {
                Error::msg("Rows' items must have names surrounded by double quotes")
            })??;
            check_unsupported_escaping(name)?;
            let parse_bit = |string: &str| {
                string.parse().map(|bit| -> Result<u8> {
                    ensure!(
                        bit < width.get(),
                        "Bit index specified for \"{name}\" is over {width}",
                    );
                    // Flip bit numbers into indices, to keep the rest of the logic much, much simpler.
                    Ok(if order_is_ascending {
                        bit
                    } else {
                        width.get() - 1 - bit
                    })
                })
            };
            let (start_idx, nb_bits) = match key.split_once('-') {
                None => (
                    parse_bit(key).with_context(|| format!("Bad bit index \"{key}\""))??,
                    1,
                ),
                Some((start_str, end_str)) => {
                    let start_idx = parse_bit(start_str).context("Bad starting bit index")??;
                    let end_idx = parse_bit(end_str).context("Bad end bit index")??;
                    ensure!(
                        start_idx <= end_idx,
                        "Bit range of \"{name}\" ({start_str}, {end_str}) is backwards",
                    );
                    (start_idx, end_idx - start_idx + 1)
                }
            };
            to_parse = rest.trim_start_matches(Pandocs::WHITESPACE_AND_NEWLINES);

            // Rendering.
            if let Some(empty_field_width) = nb_skipped_bits(start_idx, start_idx + nb_bits) {
                write!(
                    dest,
                    r#"<td colspan={empty_field_width} class="unused-field"></td>"#
                )
                .unwrap();
            }
            write!(dest, "<td colspan={nb_bits}>{name}</td>").unwrap();
        }

        // Rendering.
        if let Some(empty_field_width) = nb_skipped_bits(width.get(), u8::MAX) {
            write!(
                dest,
                r#"<td colspan={empty_field_width} class="unused-field"></td>"#
            )
            .unwrap();
        }

        dest.push_str("</tr>");
    }

    dest.push_str("</tbody></table></div>");

    Ok(())
}

fn extract_prefix_string(haystack: &str) -> Result<Result<(&str, &str)>, &str> {
    match haystack.strip_prefix('"') {
        Some(after_first_quote) => Ok(after_first_quote
            .split_once('"')
            .context("Unterminated double-quoted string")),
        None => Err(haystack),
    }
}
fn check_unsupported_escaping(extracted_string: &str) -> Result<()> {
    ensure!(
        !extracted_string.ends_with('\\'),
        "Escaping double quotes is not supported; use `&quot;` instead"
    );
    Ok(())
}
