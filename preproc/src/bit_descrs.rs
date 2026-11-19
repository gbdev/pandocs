/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use std::{num::NonZeroU8, ops::Range};

use crate::{
    Pandocs, bail,
    diagnostics::{ChapterError, ChapterResult},
    ensure,
};

pub fn render_bit_descr(dest: &mut String, indentation: &str, body: &str) -> ChapterResult {
    use std::fmt::Write;

    debug_assert!(!body.starts_with(Pandocs::WHITESPACE_AND_NEWLINES));
    debug_assert!(!body.ends_with(Pandocs::WHITESPACE_AND_NEWLINES));
    let substr_range = |substr: &str| body.as_bytes().subslice_range(substr.as_bytes()).unwrap();

    let mut header = body.splitn(3, Pandocs::WHITESPACE_AND_NEWLINES);
    // Parsing.
    let width_str = header.next().expect("body should not be just whitespace!!");
    let width: NonZeroU8 = width_str
        .parse()
        .map_err(|err| ("Invalid table width", substr_range(width_str), err))?;
    let order_str = header.next().ok_or((
        "missing bit order",
        substr_range(width_str),
        "expected `<` or `>` after this",
    ))?;
    let order_is_ascending = match order_str {
        "<" => true,
        ">" => false,
        _ => bail!(
            "Invalid bit order",
            substr_range(order_str),
            "This must be `<` or `>`"
        ),
    };
    let rows = header
        .next()
        .ok_or((
            "Rows are missing",
            substr_range(order_str),
            "expected after this",
        ))?
        .trim_matches(Pandocs::WHITESPACE_AND_NEWLINES);
    debug_assert_eq!(header.next(), None); // Should be guaranteed by `splitn()`.

    // Rendering.
    dest.push_str(indentation);
    let have_names = !rows.starts_with("\"\"");
    dest.push_str(if have_names {
        // If names are present, add an empty cell for the name column.
        r#"<div class="table-wrapper"><table class="bit-descrs"><thead><tr><th></th>"#
    } else {
        // Otherwise, add a class to correct the styling of the first column.
        r#"<div class="table-wrapper"><table class="bit-descrs nameless"><thead><tr>"#
    });
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
        let (row_name, after_name) = require_prefix_string(
            row.trim_matches(Pandocs::WHITESPACE_AND_NEWLINES),
            |trimmed_row| {
                (
                    "Missing row name",
                    substr_range(trimmed_row),
                    "expected a double-quoted name before this",
                )
            },
        )?;
        check_unsupported_escaping(row_name, body)?;
        let mut to_parse = after_name.trim_start_matches(Pandocs::WHITESPACE_AND_NEWLINES);

        let have_name = !row_name.is_empty();
        ensure!(
            have_name == have_names,
            (
                "All rows must have empty names, or none may; but not both",
                encompass_quotes(substr_range(row_name)),
                if have_name {
                    "this is the first named row"
                } else {
                    "this is the first anonymous row"
                }
            )
        );
        if have_name {
            write!(dest, "<td><strong>{row_name}</strong></td>").unwrap();
        }

        // The row proper.
        let mut next_idx = 0;
        let mut advance_bit_idx = |start_idx: u8, new_next_idx, dest: &mut String| {
            let nb_skipped_bits =
                start_idx.checked_sub(std::mem::replace(&mut next_idx, new_next_idx))?;
            if nb_skipped_bits != 0 {
                write!(
                    dest,
                    r#"<td colspan={nb_skipped_bits} class="unused-field"></td>"#
                )
                .unwrap();
            }
            Some(())
        };
        while !to_parse.is_empty() {
            // Parsing.
            let (key, after_key) = to_parse.split_once(':').ok_or((
                "Rows' fields must be `<key>:\"<name>\"`",
                substr_range(to_parse),
                "no colon found in this",
            ))?;
            let (name, rest) = require_prefix_string(after_key, |_after_key| {
                (
                    "Missing name for this field",
                    substr_range(after_key),
                    "expected a double-quoted string here",
                )
            })?;
            check_unsupported_escaping(name, body)?;
            let parse_bit = |string: &str, parse_err_msg| {
                match string.parse() {
                    Ok(bit) => {
                        ensure!(
                            bit < width.get(),
                            (
                                "Out-of-bounds bit index",
                                substr_range(string),
                                format!("this is over {width}")
                            )
                        );
                        // Flip bit numbers into indices, to keep the rest of the logic much, much simpler.
                        Ok(if order_is_ascending {
                            bit
                        } else {
                            width.get() - 1 - bit
                        })
                    }
                    Err(error) => bail!(parse_err_msg, substr_range(string), error),
                }
            };
            let (start_idx, nb_bits) = match key.split_once('-') {
                None => (parse_bit(key, "Invalid bit index")?, 1),
                Some((start_str, end_str)) => {
                    let start_idx = parse_bit(start_str, "Invalid starting bit index")?;
                    let end_idx = parse_bit(end_str, "Invalid end bit index")?;
                    ensure!(
                        start_idx <= end_idx,
                        (
                            "Bit range is backwards",
                            substr_range(key),
                            if order_is_ascending {
                                "this is not ascending"
                            } else {
                                "this is not descending"
                            }
                        )
                    );
                    (start_idx, end_idx - start_idx + 1)
                }
            };
            to_parse = rest.trim_start_matches(Pandocs::WHITESPACE_AND_NEWLINES);

            // Rendering.
            if advance_bit_idx(start_idx, start_idx + nb_bits, dest).is_none() {
                bail!(
                    "Incorrect ordering of fields",
                    substr_range(key),
                    if order_is_ascending {
                        "this must be larger than the previous field's end"
                    } else {
                        "this must be smaller than the previous field's end"
                    }
                );
            }
            write!(dest, "<td colspan={nb_bits}>{name}</td>").unwrap(); // TODO: parse `name` for Markdown and render that?
        }

        // Rendering.
        advance_bit_idx(width.get(), u8::MAX, dest).unwrap();

        dest.push_str("</tr>");
    }

    dest.push_str("</tbody></table></div>");

    Ok(())
}

fn extract_prefix_string(haystack: &str) -> Option<ChapterResult<(&str, &str)>> {
    haystack.strip_prefix('"').map(|after_first_quote| {
        after_first_quote
            .split_once('"')
            .ok_or_else(|| ("Unterminated string", 0..1, "no matching double quote").into())
    })
}
fn require_prefix_string<T: Into<ChapterError>>(
    haystack: &str,
    on_missing: impl FnOnce(&str) -> T,
) -> ChapterResult<(&str, &str)> {
    extract_prefix_string(haystack).unwrap_or_else(|| Err(on_missing(haystack).into()))
}
/// Adds in the double quotes around a diagnostic's report.
fn encompass_quotes(label_byte_range: impl Into<Range<usize>>) -> Range<usize> {
    let mut range = label_byte_range.into();
    range.start -= 1;
    range.end += 1;
    range
}

fn check_unsupported_escaping(extracted_string: &str, body: &str) -> ChapterResult {
    if extracted_string.ends_with('\\') {
        let mut error = ChapterError::new(
            "Escaping double quotes is not supported here",
            extracted_string.len() - 1..extracted_string.len() + 1, // Include the closing quote.
            "use &quot; instead",
        );
        error.substr_adjust(body, extracted_string);
        Err(error)
    } else {
        Ok(())
    }
}
