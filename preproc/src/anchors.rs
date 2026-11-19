/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use std::ops::Deref;

use anyhow::Context;
use mdbook_preprocessor::{book::Book, errors::Error};
use pulldown_cmark::{Event, Parser, Tag, TagEnd};
use rustc_hash::{FxBuildHasher, FxHashMap};
use tracing::warn;

use crate::Pandocs;

#[derive(Debug)]
pub struct InternalLinkMap {
    headings: FxHashMap<String, HeadingInfo>,
}
#[derive(Debug)]
struct HeadingInfo {
    /// Sans extension.
    file: Box<str>,
    has_homonyms: bool,
    /// Custom ID if one was provided and parsed as such.
    id: Option<Box<str>>,
}

impl InternalLinkMap {
    pub fn new(book: &Book) -> Self {
        let mut headings = FxHashMap::with_hasher(FxBuildHasher);

        for chapter in book.chapters() {
            let Some(ref path) = chapter.source_path else {
                continue; // Draft chapters. (We do not generate any “synthetic” chapters.)
            };
            let file = path
                .file_name() // mdBook will automatically translate the `.md` extension.
                .unwrap()
                .to_str()
                .expect("Non-UTF8 file name!?");

            let mut events = Parser::new_ext(&chapter.content, Pandocs::MARKDOWN_OPTIONS);
            while let Some(event) = events.next() {
                if let Event::Start(Tag::Heading { id, .. }) = event {
                    let mut name = String::new();
                    for sub_event in events
                        .by_ref()
                        .take_while(|event| !matches!(event, Event::End(TagEnd::Heading(_level))))
                    {
                        match sub_event {
                            Event::Start(_) | Event::End(_) => {} // Inline formatting markers
                            Event::Text(text) | Event::Code(text) => name.push_str(&text),
                            event => panic!("Unexpected event in header {:?}", event),
                        }
                    }

                    use std::collections::hash_map::Entry;
                    match headings.entry(name) {
                        Entry::Vacant(vacant_entry) => {
                            vacant_entry.insert(HeadingInfo {
                                file: file.into(),
                                has_homonyms: false,
                                id: id.map(|s| s.deref().into()),
                            });
                        }
                        Entry::Occupied(occupied_entry) => {
                            occupied_entry.into_mut().has_homonyms = true
                        }
                    }
                }
            }
        }

        Self { headings }
    }

    pub fn rewrite_internal_links(
        &self,
        content: &mut String,
        chapter_name: &str,
    ) -> Result<(), Error> {
        let events = Parser::new_ext(content, Pandocs::MARKDOWN_OPTIONS).map(|mut event| {
            if let Event::Start(Tag::Link {
                ref mut dest_url,
                ..
            }) = event
                && let Some(anchor) = dest_url.strip_prefix('#')
            {
                match self.headings.get(anchor) {
                    None => warn!("{chapter_name}: Found no heading called \"{anchor}\", keeping internal link as-is"),
                    Some(info) => {
                        if info.has_homonyms {
                            warn!("Referencing multiply-defined section \"{anchor}\" (using chapter \"{chapter_name}\")");
                        }

                        let target_fragment = match info.id.as_deref() {
                            Some(id) => id,
                            None => &id_from_name(anchor),
                        };
                        *dest_url = format!("{}#{target_fragment}", info.file).into();
                    }
                }
            }
            event
        });

        let mut new_content = String::with_capacity(content.len());
        pulldown_cmark_to_cmark::cmark(events, &mut new_content, Pandocs::MARKDOWN_OPTIONS)
            .context("serialisation failed")?;
        *content = new_content;

        Ok(())
    }

    #[cfg(test)]
    pub(crate) const fn empty() -> Self {
        Self {
            headings: FxHashMap::with_hasher(FxBuildHasher),
        }
    }
}

fn id_from_name(name: &str) -> Box<str> {
    // Normalize like mdBook
    name.chars()
        .filter_map(|ch| {
            if ch.is_alphanumeric() || ch == '_' || ch == '-' {
                Some(ch.to_ascii_lowercase())
            } else if ch.is_whitespace() {
                Some('-')
            } else {
                None
            }
        })
        .collect()
}
