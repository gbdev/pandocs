/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use std::borrow::Cow;

use anyhow::{bail, Context, Error};
use lazy_static::lazy_static;
use mdbook::book::Chapter;
use regex::Regex;

use crate::Pandocs;

impl Pandocs {
    pub fn process_bit_descrs(&self, chapter: &mut Chapter) -> Result<(), Error> {
        // When replacing one thing in a string by something with a different length,
        // the indices after that will not correspond,
        // we therefore have to store the difference to correct this
        let mut previous_end_index = 0;
        let mut replaced = String::with_capacity(chapter.content.len());

        for result in find_bit_descrs(&chapter.content) {
            let (cap_start, cap_end, attrs) = result?;

            replaced.push_str(&chapter.content[previous_end_index..cap_start]);

            let (start, end) = if attrs.increasing {
                (0, attrs.width - 1)
            } else {
                (attrs.width - 1, 0)
            };

            // Generate the table head
            if !attrs.rows[0].0.is_empty() {
                // If names are present, add an empty cell for the name column
                replaced.push_str(
                    "<div class=\"table-wrapper\"><table class=\"bit-descrs\"><thead><tr><th></th>",
                );
            } else {
                // Otherwise, add a class to force correct styling of first column
                replaced.push_str(
                    "<div class=\"table-wrapper\"><table class=\"bit-descrs nameless\"><thead><tr>",
                );
            }
            // Start at `start`, and step each time
            for i in std::iter::successors(Some(start), |i| {
                (*i != end).then(|| if attrs.increasing { i + 1 } else { i - 1 })
            }) {
                replaced.push_str(&format!("<th>{}</th>", i));
            }
            replaced.push_str("</tr></thead><tbody>");

            for (name, row) in &attrs.rows {
                replaced.push_str("<tr>");
                // If a name is present, add it
                if !name.is_empty() {
                    replaced.push_str(&format!("<td><strong>{}</strong></td>", name));
                }
                let mut pos = 0;
                let mut fields = row.iter().peekable();
                while pos < attrs.width {
                    let (len, is_unused, name) = match fields.peek() {
                        // If we are at the edge of a "used" field, use it
                        Some(field) if field.start == pos => {
                            (field.len, false, field.name.as_ref())
                        }
                        // If in an unused field, end at the next field, or the width if none such
                        res => (res.map_or(attrs.width, |field| field.start) - pos, true, ""),
                    };

                    replaced.push_str(&format!(
                        "<td colspan=\"{}\"{}>{}</td>",
                        len,
                        if is_unused {
                            " class=\"unused-field\""
                        } else {
                            ""
                        },
                        name
                    ));

                    if !is_unused {
                        fields.next();
                    }
                    pos += len;
                }
                replaced.push_str("</tr>");
            }
            replaced.push_str("</tbody></table></div>");

            previous_end_index = cap_end;
        }

        replaced.push_str(&chapter.content[previous_end_index..]);

        chapter.content = replaced;
        Ok(())
    }
}

fn find_bit_descrs(
    contents: &str,
) -> impl Iterator<Item = Result<(usize, usize, BitDescrAttrs<'_>), Error>> {
    lazy_static! {
        static ref RE: Regex = Regex::new(
            r"(?x)             # Allow comments in the regex
            \\\{\{\#.*\}\}     # Escaped tag (will be ignored)
            |                  # ...or...
            \{\{\s*\#bits\s+   # tag opening braces, whitespace, type, and separating whitespace
            ([^}]+)            # tag contents
            \}\}               # closing braces"
        )
        .unwrap();
    }
    RE.captures_iter(contents)
        .filter(|caps| caps.len() != 1)
        .map(|caps| {
            // Must use `.get()`, as indexing ties the returned value's lifetime to `caps`'s.
            let contents = caps.get(1).unwrap().as_str();
            BitDescrAttrs::from_str(contents)
                .map(|attrs| {
                    let all = caps.get(0).unwrap(); // There is always a 0th capture.
                    (all.start(), all.end(), attrs)
                })
                .context(format!("Failed to parse \"{contents}\""))
        })
}

#[derive(Debug)]
struct BitDescrAttrs<'input> {
    width: usize,
    rows: Vec<(Cow<'input, str>, Vec<BitDescrField<'input>>)>,
    increasing: bool,
}

impl<'input> BitDescrAttrs<'input> {
    fn from_str(contents: &'input str) -> Result<Self, Error> {
        // First, parse the width.
        let contents = contents.trim();
        let width_len = contents
            .find(|c: char| c.is_ascii_whitespace())
            .ok_or_else(|| Error::msg("{{#bits}} descriptions must describe at least one thing"))?;
        let width_str = &contents[..width_len];
        let width = width_str.parse().context(format!(
            "Expected bits description to start with width, got \"{}\"",
            width_str
        ))?;
        let s = contents[width_len..].trim_start();

        // Then, parse the direction
        let mut chars = s.chars();
        // Angle brackets have a tendency to get escaped, so account for that
        let (base_len, next) = match chars.next() {
            Some('\\') => ('\\'.len_utf8(), chars.next()),
            c => (0, c),
        };
        let increasing = match next {
            Some('<') => true,
            Some('>') => false,
            c => {
                bail!(
                    "Expected width to be followed by '<' or '>' for direction, found {}",
                    c.map_or(Cow::from("nothing"), |c| format!(
                        "'{}'",
                        &s[..base_len + c.len_utf8()]
                    )
                    .into()),
                );
            }
        };
        debug_assert_eq!('<'.len_utf8(), 1);
        debug_assert_eq!('>'.len_utf8(), 1);
        let s = s[base_len + 1..].trim_start();

        // Next, parse the rows!
        let mut rows = Vec::new();
        let mut name_type = None;
        for row_str in s.split_terminator(';') {
            let row_str = row_str.trim();

            fn undo_escapes(escaped: &str) -> Cow<'_, str> {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r#"\\(\[|\])"#).unwrap();
                }
                RE.replace_all(escaped, "$1")
            }

            fn parse_name(row_str: &str) -> Option<usize> {
                if !row_str.starts_with('"') {
                    return None;
                }

                row_str[1..] // Skip the leading quote.
                    .find('"')
            }
            let Some(name_len) = parse_name(row_str) else {
                bail!(
                    "Expected row to begin by its name (did you forget to put quotes around it?)"
                );
            };
            let name = undo_escapes(&row_str[1..(name_len + 1)]);
            let mut row_str = row_str[(name_len + 2)..].trim_start(); // The end is already trimmed.

            // Then, the fields!
            let mut fields: Vec<BitDescrField> = Vec::new();
            while !row_str.is_empty() {
                lazy_static! {
                    // Since mdBook has "smart quotes", be lenient about them.
                    static ref RE: Regex =
                        Regex::new(r#"^(\d+)(?:\s*-\s*(\d+))?\s*:\s*"([^"]*)""#).unwrap();
                }

                let Some(cap) = RE.captures(row_str) else {
                    bail!("Failed to parse field for \"{}\"", row_str);
                };
                let left: usize = cap[1].parse().unwrap();
                let right = cap
                    .get(2)
                    .map_or(left, |end_match| end_match.as_str().parse().unwrap());
                let name = undo_escapes(&cap.get(3).unwrap().as_str());

                // Perform some sanity checks.
                let Some((mut start, len)) = if increasing {
                    right.checked_sub(left)
                } else {
                    left.checked_sub(right)
                }
                .map(|len| (left, len + 1)) else {
                    bail!("Field must end after it started ({}-{})", left, right)
                };

                if let Some(field) = fields.last() {
                    if !increasing {
                        // Cancel the massaging to get back what was input
                        let prev_end = width - field.end();

                        if prev_end < start {
                            bail!(
                                "Field must start after previous ended (expected {} > {})",
                                prev_end - 1,
                                start
                            );
                        }
                    } else if field.end() > start {
                        bail!(
                            "Field must start after previous ended (expected {} < {})",
                            field.end() - 1,
                            start
                        );
                    }
                }

                // If in decreasing order, still store positions in increasing order to simplify processing.
                if !increasing {
                    start = width - 1 - start;
                }

                fields.push(BitDescrField { start, len, name });

                // Advance by the match's length, plus any whitespace after it.
                row_str = row_str[cap[0].len()..].trim_start();
            }

            // Check the name type.
            let new_name_type = name.is_empty();
            if let Some(name_type) = name_type {
                if new_name_type != name_type {
                    return Err(Error::msg("Row names must all be omitted, or none may be"));
                }
            } else {
                name_type = Some(new_name_type);
            }

            rows.push((name, fields));
        }

        Ok(BitDescrAttrs {
            width,
            rows,
            increasing,
        })
    }
}

#[derive(Debug)]
struct BitDescrField<'a> {
    start: usize,
    len: usize,
    name: Cow<'a, str>,
}

impl BitDescrField<'_> {
    fn end(&self) -> usize {
        self.start + self.len
    }
}
