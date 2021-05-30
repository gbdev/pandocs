/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use mdbook::book::{Book, BookItem, Chapter};
use mdbook::errors::Error;
use mdbook::preprocess::{Preprocessor, PreprocessorContext};
use pulldown_cmark::{CowStr, Event, LinkType, Options, Parser, Tag};
use std::collections::HashMap;
use std::io::Write;
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
        let output = Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .stderr(Stdio::inherit())
            .stdin(Stdio::null())
            .output()
            .expect("Failed to get commit hash");
        if !output.status.success() {
            return Err(Error::msg(format!(
                "Git exited with status {} while getting commit hash",
                output.status
            )));
        }
        let commit_hash = str::from_utf8(&output.stdout)
            .expect("Commit hash is not valid UTF-8??")
            .trim();

        let output = Command::new("git")
            .args(&["rev-parse", "--short", "HEAD"])
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
        let commit_short_hash = str::from_utf8(&output.stdout)
            .expect("Commit hash is not valid UTF-8??")
            .trim();

        let output = Command::new("git")
            .args(&["show", "-s", "--format=%ci", "HEAD"])
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
        let commit_timestamp = str::from_utf8(&output.stdout)
            .expect("Commit hash is not valid UTF-8??")
            .trim();

        let mut sections = HashMap::new();
        for item in book.iter() {
            if let BookItem::Chapter(ref chapter) = item {
                self.list_chapter_sections(&mut sections, &chapter);
            }
        }

        let mut res = Ok(());

        book.for_each_mut(|item| {
            if res.is_err() {
                return;
            }

            if let BookItem::Chapter(ref mut chapter) = item {
                if let Err(e) = self.process_chapter(chapter, &sections) {
                    res = Err(e);
                }

                if chapter.name == "Foreword" {
                    chapter.content.push_str(&format!("<small>This document version was produced from git commit [`{}`](https://github.com/gbdev/pandocs/tree/{}) ({}). </small>", commit_short_hash, commit_hash,commit_timestamp));
                }
            }
        });

        res.map(|_| book)
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

    fn process_chapter(
        &self,
        chapter: &mut Chapter,
        sections: &HashMap<String, (String, bool)>,
    ) -> Result<(), Error> {
        let mut buf = String::with_capacity(chapter.content.len());
        let extensions = Options::ENABLE_TABLES
            | Options::ENABLE_FOOTNOTES
            | Options::ENABLE_STRIKETHROUGH
            | Options::ENABLE_SMART_PUNCTUATION;

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
