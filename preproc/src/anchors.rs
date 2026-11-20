/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use std::ops::Deref;

use mdbook_preprocessor::book::Book;
use pulldown_cmark::{CowStr, Event, Parser, Tag, TagEnd};
use rustc_hash::{FxBuildHasher, FxHashMap};

use crate::{Pandocs, bail, diagnostics::ChapterError, ensure};

#[derive(Debug)]
pub struct InternalLinkMap {
    headings: FxHashMap<String, HeadingInfo>,
}
#[derive(Debug)]
struct HeadingInfo {
    file_name: Option<Box<str>>,
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
            let file_name = path
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
                                file_name: Some(file_name.into()),
                                id: id.map(|s| s.deref().into()),
                            });
                        }
                        Entry::Occupied(occupied_entry) => {
                            occupied_entry.into_mut().file_name = None
                        }
                    }
                }
            }
        }

        Self { headings }
    }

    pub fn rewrite_internal_links(&self, content: &str) -> (Vec<ChapterError>, String) {
        let rewrite_link = |dest_url: &mut CowStr<'_>, src_range| {
            let Some(anchor) = dest_url.strip_prefix('#') else {
                return Ok(());
            };

            let Some(info) = self.headings.get(anchor) else {
                bail!(
                    "No such heading",
                    src_range,
                    "not found anywhere in the book"
                );
            };
            let Some(ref file_name) = info.file_name else {
                bail!(
                    "This heading exists in multiple places",
                    src_range,
                    "please disambiguate (TODO)"
                );
            };

            // `unwrap_or_else` would not extend the lifetime of the temporary returned by `id_from_name`!
            let target_fragment = match info.id {
                Some(ref id) => id,
                None => &id_from_name(anchor),
            };
            *dest_url = format!("{file_name}#{target_fragment}").into();

            Ok(())
        };

        let mut errors = vec![];

        let events = Parser::new_ext(content, Pandocs::MARKDOWN_OPTIONS)
            .into_offset_iter()
            .map(|(mut event, src_range)| {
                #[allow(clippy::collapsible_if)] // They serve different semantic purposes.
                if let Event::Start(Tag::Link {
                    ref mut dest_url, ..
                }) = event
                {
                    // TODO: if the link uses a ref, we should point to it...
                    if let Err(error) = rewrite_link(dest_url, src_range) {
                        errors.push(error);
                    }
                }
                event
            });

        let mut new_content = String::with_capacity(content.len());
        pulldown_cmark_to_cmark::cmark(events, &mut new_content, Pandocs::MARKDOWN_OPTIONS)
            .expect("Markdown serialisation failed!?");

        (errors, new_content)
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
