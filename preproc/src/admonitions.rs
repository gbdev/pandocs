/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use std::{iter::Peekable, matches};

use anyhow::Error;
use mdbook::book::Chapter;
use pulldown_cmark::{Event, Options, Parser, Tag};

use crate::Pandocs;

impl Pandocs {
    pub fn process_admonitions(&self, chapter: &mut Chapter) -> Result<(), Error> {
        let mut buf = String::with_capacity(chapter.content.len());
        let extensions =
            Options::ENABLE_TABLES | Options::ENABLE_FOOTNOTES | Options::ENABLE_STRIKETHROUGH;

        let events = AdmonitionsGenerator::new(Parser::new_ext(&chapter.content, extensions));

        pulldown_cmark_to_cmark::cmark(events, &mut buf, None)
            .map_err(|err| Error::from(err).context("Markdown serialization failed"))?;
        chapter.content = buf;

        Ok(())
    }
}

struct AdmonitionsGenerator<'a, Iter: Iterator<Item = Event<'a>>> {
    iter: Peekable<Iter>,
    nesting_level: usize,
    at_paragraph_start: bool,
}

impl<'a, Iter: Iterator<Item = Event<'a>>> AdmonitionsGenerator<'a, Iter> {
    const KINDS: [&'static str; 3] = ["tip", "warning", "danger"];

    fn new(iter: Iter) -> Self {
        Self {
            iter: iter.peekable(),
            nesting_level: 0,
            at_paragraph_start: false,
        }
    }
}

impl<'a, Iter: Iterator<Item = Event<'a>>> Iterator for AdmonitionsGenerator<'a, Iter> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut evt = self.iter.next()?;

        match evt {
            Event::Text(ref text) if self.at_paragraph_start => {
                if let Some(params) = text.strip_prefix(":::") {
                    // Check that there is no more text in the paragraph; if there isn't, we'll consume the entire paragraph.
                    // Note that this intentionally rejects any formatting within the paragraph—serialisation would be too complex.
                    if matches!(self.iter.peek(), Some(Event::End(Tag::Paragraph))) {
                        if params.is_empty() {
                            if self.nesting_level != 0 {
                                // Ending an admonition.
                                self.nesting_level -= 1;

                                evt = Event::Html("</div>".into());
                            }
                        } else {
                            let (kind, title) =
                                match params.split_once(|c: char| c.is_ascii_whitespace()) {
                                    Some((kind, title)) => (kind, title.trim()),
                                    None => (params, ""),
                                };
                            if Self::KINDS.contains(&kind) {
                                // Beginning an admonition.
                                self.nesting_level += 1;

                                evt = Event::Html(
                                    if title.is_empty() {
                                        format!("<div class=\"box {kind}\">")
                                    } else {
                                        format!("<div class=\"box {kind}\"><p class=\"box-title\">{title}</p>")
                                    }
                                    .into(),
                                );
                            }
                        }
                    }
                }
            }
            _ => {}
        }

        self.at_paragraph_start = matches!(evt, Event::Start(Tag::Paragraph));

        Some(evt)
    }
}
