/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use anyhow::{bail, Context};
use lazy_static::lazy_static;
use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CowStr, Event, LinkType, Options, Parser, Tag};
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;
use std::io::Write;
use std::iter;
use std::process::{Command, Stdio};
use std::str;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

pub struct Pandocs;

impl Pandocs {
    pub fn new() -> Pandocs {
        Pandocs
    }
}

impl Preprocessor for Pandocs {
    fn name(&self) -> &str {
        "pandocs-preproc"
    }

    fn supports_renderer(&self, renderer: &str) -> bool {
        renderer != "not-supported"
    }

    fn run(&self, _: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let mut sections = HashMap::new();
        for item in book.iter() {
            if let BookItem::Chapter(ref chapter) = item {
                self.list_chapter_sections(&mut sections, chapter);
            }
        }

        let mut res = Ok(());

        book.for_each_mut(|item| {
            macro_rules! abort_if_err {
                ($expr:expr) => {
                    match $expr {
                        Err(e) => {
                            res = Err(e);
                            return;
                        }
                        Ok(ret) => ret,
                    }
                };
            }

            if res.is_err() {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = item {
                abort_if_err!(self.process_internal_anchor_links(chapter, &sections));
                abort_if_err!(self.process_bit_descrs(chapter).context(format!("While processing chapter \"{}\"", chapter.name)));

                if chapter.name == "Foreword" {
                    let commit = abort_if_err!(Commit::rev_parse("HEAD"));
                    chapter.content.push_str(&format!(
                        "<small>This document version was produced from git commit [`{}`](https://github.com/gbdev/pandocs/tree/{}) ({}). </small>",
                        commit.short_hash(), commit.hash(), commit.timestamp(),
                    ));
                }
            }
        });

        res.map(|_| book)
    }
}

#[derive(Debug)]
struct Commit {
    hash: String,
    short_hash: String,
    timestamp: String,
}

impl Commit {
    fn rev_parse(what: &str) -> Result<Self, Error> {
        let output = Command::new("git")
            .args(["rev-parse", what])
            .stderr(Stdio::inherit())
            .stdin(Stdio::null())
            .output()
            .expect("Failed to get commit hash");
        if !output.status.success() {
            return Err(Error::msg(format!(
                "Git exited with {} while getting commit hash",
                output.status
            )));
        }
        let hash = String::from_utf8(output.stdout).expect("Commit hash is not valid UTF-8??");

        let output = Command::new("git")
            .args(["rev-parse", "--short", what])
            .stderr(Stdio::inherit())
            .stdin(Stdio::null())
            .output()
            .expect("Failed to get short commit hash");
        if !output.status.success() {
            return Err(Error::msg(format!(
                "Git exited with status {} while getting commit short hash",
                output.status
            )));
        }
        let short_hash =
            String::from_utf8(output.stdout).expect("Commit hash is not valid UTF-8??");

        let output = Command::new("git")
            .args(["show", "-s", "--format=%ci", what])
            .stderr(Stdio::inherit())
            .stdin(Stdio::null())
            .output()
            .expect("Failed to get timestamp");
        if !output.status.success() {
            return Err(Error::msg(format!(
                "Git exited with status {} while getting timestamp",
                output.status
            )));
        }
        let timestamp = String::from_utf8(output.stdout).expect("Commit hash is not valid UTF-8??");

        Ok(Self {
            hash,
            short_hash,
            timestamp,
        })
    }

    fn hash(&self) -> &str {
        self.hash.trim()
    }

    fn short_hash(&self) -> &str {
        self.short_hash.trim()
    }

    fn timestamp(&self) -> &str {
        self.timestamp.trim()
    }
}

impl Pandocs {
    fn list_chapter_sections(
        &self,
        sections: &mut HashMap<String, (String, bool)>,
        chapter: &Chapter,
    ) {
        let mut events = Parser::new(&chapter.content);
        while let Some(event) = events.next() {
            if let Event::Start(Tag::Heading(_)) = event {
                let mut depth = 1;
                let mut name = String::new();

                while depth != 0 {
                    match events.next().expect("Unclosed `Start` tag??") {
                        Event::Start(_) => depth += 1,
                        Event::End(_) => depth -= 1,
                        Event::Text(text) | Event::Code(text) => name.push_str(&text),
                        event => panic!("Unexpected event in header {:?}", event),
                    }
                }

                let chapter_name = name.clone();
                let mut page_name = chapter
                    .path
                    .as_ref()
                    .expect("Chapter without a source file??")
                    .clone()
                    .into_os_string()
                    .into_string()
                    .expect("Chapter file path is not valid UTF-8");
                page_name.truncate(
                    page_name
                        .strip_suffix(".md")
                        .expect("Source file not ending in `.md`??")
                        .len(),
                );

                if sections.insert(name, (page_name, false)).is_some() {
                    // Mark the ambiguity
                    sections.get_mut(&chapter_name).unwrap().1 = true;
                }
            }
        }
    }

    fn process_internal_anchor_links(
        &self,
        chapter: &mut Chapter,
        sections: &HashMap<String, (String, bool)>,
    ) -> Result<(), Error> {
        let mut buf = String::with_capacity(chapter.content.len());
        let extensions =
            Options::ENABLE_TABLES | Options::ENABLE_FOOTNOTES | Options::ENABLE_STRIKETHROUGH;

        let events = Parser::new_ext(&chapter.content, extensions).map(|event| match event {
            Event::Start(Tag::Link(link_type, url, title)) if url.starts_with('#') => {
                let (link, ok) = translate_anchor_link(sections, link_type, url, title);
                if !ok {
                    let mut stderr = StandardStream::stderr(ColorChoice::Auto);
                    stderr
                        .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
                        .unwrap();
                    write!(&mut stderr, "warning:").unwrap();
                    stderr.reset().unwrap();

                    if let Tag::Link(_, ref url, _) = link {
                        eprintln!(
                            " {}: Internal anchor link \"{}\" not found, keeping as-is",
                            &chapter.name, url
                        );
                    } else {
                        unreachable!()
                    }
                }
                Event::Start(link)
            }

            Event::End(Tag::Link(link_type, url, title)) if url.starts_with('#') => {
                Event::End(translate_anchor_link(sections, link_type, url, title).0)
            }

            _ => event,
        });

        pulldown_cmark_to_cmark::cmark(events, &mut buf, None)
            .map_err(|err| Error::from(err).context("Markdown serialization failed"))?;
        chapter.content = buf;

        Ok(())
    }
}

fn translate_anchor_link<'a>(
    sections: &HashMap<String, (String, bool)>,
    link_type: LinkType,
    url: CowStr<'a>,
    title: CowStr<'a>,
) -> (Tag<'a>, bool) {
    let (url, ok) = if let Some((chapter, multiple)) = sections.get(&url[1..]) {
        if *multiple {
            let mut stderr = StandardStream::stderr(ColorChoice::Auto);
            stderr
                .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
                .unwrap();
            write!(&mut stderr, "warning:").unwrap();
            stderr.reset().unwrap();
            eprintln!(
                " Referencing multiply-defined section \"{}\" (using chapter \"{}\")",
                &url[1..],
                &chapter
            );
        }
        (
            CowStr::Boxed(format!("{}.html#{}", chapter, id_from_name(&url[1..])).into_boxed_str()),
            true,
        )
    } else {
        (url, false)
    };

    (Tag::Link(link_type, url, title), ok)
}

fn id_from_name(name: &str) -> String {
    let mut content = name.to_string();

    // Skip any tags or html-encoded stuff
    const REPL_SUB: &[&str] = &[
        "<em>",
        "</em>",
        "<code>",
        "</code>",
        "<strong>",
        "</strong>",
        "&lt;",
        "&gt;",
        "&amp;",
        "&#39;",
        "&quot;",
    ];
    for sub in REPL_SUB {
        content = content.replace(sub, "");
    }

    // Remove spaces and hashes indicating a header
    let trimmed = content.trim().trim_start_matches('#').trim();

    // Normalize
    trimmed
        .chars()
        .filter_map(|ch| {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                Some(ch.to_ascii_lowercase())
            } else if ch.is_whitespace() {
                Some('-')
            } else {
                None
            }
        })
        .collect::<String>()
}

impl Pandocs {
    fn process_bit_descrs(&self, chapter: &mut Chapter) -> Result<(), Error> {
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
                replaced.push_str("<table class=\"bit-descrs\"><thead><tr><th></th>");
            } else {
                // Otherwise, add a class to force correct styling of first column
                replaced.push_str("<table class=\"bit-descrs nameless\"><thead><tr>");
            }
            // Start at `start`, and step each time
            for i in iter::successors(Some(start), |i| {
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
                        Some(field) if field.start == pos => (field.len, false, field.name),
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
            replaced.push_str("</tbody></table>");

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
    rows: Vec<(&'input str, Vec<BitDescrField<'input>>)>,
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

            fn parse_name(row_str: &str) -> Option<usize> {
                if !row_str.starts_with('"') {
                    return None;
                }

                row_str[1..] // Skip the leading quote.
                    .find('"')
            }
            let Some(name_len) = parse_name(row_str) else {
                bail!("Expected row to begin by its name (did you forget to put quotes around it?)");
            };
            let name = &row_str[1..(name_len + 1)];
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
                let name = &cap.get(3).unwrap().as_str();

                // Perform some sanity checks.
                let Some((mut start, len)) = if increasing {
                    right.checked_sub(left)
                } else {
                    left.checked_sub(right)
                }
                .map(|len| (left, len + 1)) else {
                    bail!(
                        "Field must end after it started ({}-{})",
                        left, right
                    )};

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
    name: &'a str,
}

impl BitDescrField<'_> {
    fn end(&self) -> usize {
        self.start + self.len
    }
}
