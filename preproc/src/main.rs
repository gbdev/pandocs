/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use std::collections::HashMap;
use std::io;
use std::path::Path;
use std::process;

use anyhow::{anyhow, Context};
use mdbook_preprocessor::{
    book::{Book, BookItem},
    errors::Error,
    Preprocessor, PreprocessorContext,
};

mod admonitions;
mod anchors;
mod bit_descrs;
mod git;
use git::Commit;

fn main() -> Result<(), Error> {
    let mut args = std::env::args();
    let _ = args.next(); // Program name.

    let preprocessor = Pandocs::new();

    if args.next().is_some_and(|arg| arg == "supports") {
        let renderer_name = args
            .next()
            .expect("Missing argument: name of renderer to check for compat");
        assert!(args.next().is_none(), "Unexpected extra args");
        handle_supports(&preprocessor, &renderer_name);
    } else {
        handle_preprocessing(&preprocessor)
    }
}

fn handle_preprocessing(pre: &dyn Preprocessor) -> Result<(), Error> {
    let (ctx, book) = mdbook_preprocessor::parse_input(io::stdin())?;

    if ctx.mdbook_version != mdbook_preprocessor::MDBOOK_VERSION {
        // We should probably use the `semver` crate to check compatibility
        // here...
        eprintln!(
            "Warning: The {} plugin was built against version {} of mdbook, \
             but we're being called from version {}",
            pre.name(),
            mdbook_preprocessor::MDBOOK_VERSION,
            ctx.mdbook_version
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &dyn Preprocessor, renderer: &str) -> ! {
    let supported = pre.supports_renderer(renderer);

    // Signal whether the renderer is supported by exiting with 1 or 0.
    if supported.unwrap() {
        process::exit(0);
    } else {
        process::exit(1);
    }
}

struct Pandocs;

impl Pandocs {
    fn new() -> Pandocs {
        Pandocs
    }
}

impl Preprocessor for Pandocs {
    fn name(&self) -> &str {
        "pandocs-preproc"
    }

    fn supports_renderer(&self, renderer: &str) -> Result<bool, anyhow::Error> {
        Ok(renderer != "not-supported")
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book, Error> {
        let out_of_repo = match ctx.config.get::<bool>("preprocessor.pandocs.out-of-repo") {
            Ok(boolean) => match boolean {
                Some(b) => b,
                None => false,
            },
            Err(_) => false,
        };

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
                abort_if_err!(self.process_admonitions(chapter));

                if chapter.name == "Foreword" {
                    // If the `.git` directory exists, we're very likely on a dev machine,
                    // so it's safe to assume the command is installed.
                    if Path::new(".git").exists() {
                        let commit = abort_if_err!(Commit::rev_parse("HEAD"));
                        chapter.content.push_str(&format!(
                            "<small>This document version was produced from git commit [`{}`](https://github.com/gbdev/pandocs/tree/{}) ({}).</small>",
                            commit.short_hash(), commit.hash(), commit.timestamp(),
                        ));
                    } else if out_of_repo {
                        // OK, just don't add anything.
                    } else {
                        res = Err(anyhow!("Git metadata is missing, but out-of-repo builds are not enabled!\n\tYou can enable them by setting `preprocessor.pandocs.out-of-repo` to `true`.\n\t(Consider using an environment variable for this:\n\t https://rust-lang.github.io/mdBook/format/configuration/environment-variables.html)"));
                        return;
                    }
                }
            }
        });

        res.map(|_| book)
    }
}
