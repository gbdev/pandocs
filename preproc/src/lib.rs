use std::{collections::HashMap, path::Path};

use anyhow::Context;
use mdbook_preprocessor::{
    Preprocessor, PreprocessorContext,
    book::{Book, BookItem},
    errors::{Error, Result},
};

mod admonitions;
mod anchors;
mod bit_descrs;
mod git;

pub struct Pandocs;

impl Pandocs {
    const MARKDOWN_OPTIONS: pulldown_cmark::Options = pulldown_cmark::Options::ENABLE_TABLES
        .union(pulldown_cmark::Options::ENABLE_FOOTNOTES)
        .union(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
    const MARKDOWN_OPTIONS_RENDER: pulldown_cmark::Options = pulldown_cmark::Options::ENABLE_TABLES
        .union(pulldown_cmark::Options::ENABLE_OLD_FOOTNOTES) // Workaround for Byron/pulldown-cmark-to-cmark#118
        .union(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
}

impl Preprocessor for Pandocs {
    fn name(&self) -> &'static str {
        "pandocs-preproc"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let out_of_repo = match ctx.config.get::<bool>("preprocessor.pandocs.out-of-repo") {
            Ok(Some(boolean)) => boolean,
            Err(_) | Ok(None) => false,
        };

        let mut sections = HashMap::new();
        for item in book.iter() {
            if let BookItem::Chapter(chapter) = item {
                self.list_chapter_sections(&mut sections, chapter);
            }
        }

        let mut res = Ok(());

        // Note that this skips draft chapters!
        book.for_each_chapter_mut(|chapter| {
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

            // Keep short-circuiting if a previous chapter encountered a fatal error.
            if res.is_err() {
                return;
            }

            // Bit descrs should be first, since it does not like the HTML entity serialisation done by the later steps.
            abort_if_err!(self.process_bit_descrs(chapter).with_context(|| format!("While processing chapter \"{}\"", chapter.name)));
            abort_if_err!(self.process_internal_anchor_links(chapter, &sections));
            abort_if_err!(self.process_admonitions(chapter));

            if chapter.name == "Foreword" {
                // If the `.git` directory exists, we're very likely on a dev machine,
                // so it's safe to assume the command is installed.
                if Path::new(".git").exists() {
                    let commit = abort_if_err!(crate::git::Commit::rev_parse("HEAD"));
                    chapter.content.push_str(&format!(
                        "<small>This document version was produced from git commit [`{}`](https://github.com/gbdev/pandocs/tree/{}) ({}).</small>",
                        commit.short_hash(), commit.hash(), commit.timestamp(),
                    ));
                } else if out_of_repo {
                    // OK, just don't add anything.
                } else {
                    res = Err(Error::msg("\
Git metadata is missing, but out-of-repo builds are not enabled!
\tYou can enable them by setting `preprocessor.pandocs.out-of-repo` to `true`.
\t(Consider using an environment variable for this:
\t https://rust-lang.github.io/mdBook/format/configuration/environment-variables.html)"));
                }
            }
        });

        res.map(|_| book)
    }
}
