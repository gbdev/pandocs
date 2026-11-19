use std::path::Path;

use anyhow::{Context, bail};
use mdbook_preprocessor::{Preprocessor, PreprocessorContext, book::Book, errors::Result};
use regex::Regex;

mod admonitions;
mod anchors;
mod bit_descrs;
mod git;
#[cfg(not(doctest))]
pub(crate) mod replacing;
#[cfg(doctest)]
pub mod replacing;

pub struct Pandocs {
    block_regex: Regex,
}

impl Pandocs {
    /// For parsing.
    const WHITESPACE: [char; 2] = [' ', '\t'];
    const WHITESPACE_AND_NEWLINES: [char; 4] = [' ', '\t', '\r', '\n'];

    #[allow(clippy::new_without_default)]
    pub fn new() -> Self {
        // Note that putting the string inline within `Regex::new` allows its syntax to be checked by Clippy.
        let block_regex = Regex::new(
            r"(?xm-u) # Verbose syntax; `^` and `$` can match BOL/EOL; ASCII char classes
^([\ \t]*)(?: # Blocks must always open their lines
  \{\{ [\ \t]*        # Opening braces, with optional whitespace
    \#(\w+)\b         # Tag name
    [\ \t\r\n]* ((?u) # Contents (may be empty)
      (?:[^{}\\]|\\[{}\\])* # All backslashes and braces within contents must be backslash-escaped
      [^}\\][\ \t\r\n]      # Must not end in a closing brace, backslash, or whitespace
    )? [\ \t\r\n]*          # (...guarantees this matches all trailing whitespace)
  \}\}            # Closing braces
  |
  ::: (\w+)     # Admonition opener, with kind
    (?u:[\ \t]+(.*[^\ \t\r\n]))? # Optional title (cannot be empty or end with whitespace)
    [\ \t]*$                     # End of the line
  ((?su) .*?)   # Contents, which can be multi-line
  ^([\ \t]*)::: # End of admonition
)[\ \t]*$
|
(?:\{\{|:::) # Block opener not recognised by the full matchers: signals a syntax error",
        )
        .unwrap();
        Self { block_regex }
    }

    fn process_chapter(
        &self,
        name: &str,
        contents: &mut String,
        link_map: &anchors::InternalLinkMap,
        is_out_of_repo: bool,
    ) -> Result<()> {
        // Process blocks.
        let mut replacer = replacing::Replacer::new(contents);
        for captures in self.block_regex.captures_iter(contents) {
            // TODO: report location of errors
            let dest = replacer.replace(captures.get_match().range());
            let opt_match = |idx| captures.get(idx).map(|m| m.as_str());
            /* Capture map:
             * 1 = indentation
             * 2 = tag name
             * 3 = tag contents (will be missing if empty)
             * 4 = admonition type
             * 5 = admonition title (optional)
             * 6 = admonition contents (may be empty)
             * 7 = admonition closing
             */
            let indentation = captures
                .get(1)
                .with_context(|| format!("bad block start at {:?}", captures.get_match()))?
                .as_str();
            // TODO: `debug_assert` the matches..?
            if let Some(tag_name) = captures.get(2) {
                match tag_name.as_str() {
                    "bits" => bit_descrs::render_bit_descr(dest, indentation, opt_match(3))
                        .with_context(|| {
                            format!("Invalid {{#bits}} block at {:?}", captures.get_match())
                        })?,
                    unknown => bail!("unknown tag name \"{unknown}\""),
                }
            } else if let Some(admonition_type) = captures.get(4) {
                admonitions::render_admonition(
                    dest,
                    indentation,
                    &captures[7],
                    admonition_type.as_str(),
                    opt_match(5),
                    &captures[6],
                )
                .with_context(|| format!("Bad admonition at {:?}", captures.get_match()))?;
            } else {
                unreachable!(); // One of the regex's two arms must have matched.
            }
        }
        if let Some(new_contents) = replacer.finish() {
            *contents = new_contents;
        }

        // Process internal anchor links.
        // We do this in a separate pass from the above, since they may introduce extra
        link_map
            .rewrite_internal_links(contents, name)
            .context("link map error")?;

        if name == "Foreword" {
            use std::fmt::Write;

            // If the `.git` directory exists, we're very likely on a dev machine,
            // so it's safe to assume the command is installed.
            if Path::new(".git").exists() {
                let commit = crate::git::Commit::rev_parse("HEAD")
                    .context("unable to obtain commit info")?;
                write!(
                    contents,
                    "<small>This document version was produced from git commit [`{}`](https://github.com/gbdev/pandocs/tree/{}) ({}).</small>",
                    commit.short_hash(),
                    commit.hash(),
                    commit.timestamp(),
                ).unwrap();
            } else if is_out_of_repo {
                // OK, just don't add anything.
            } else {
                bail!(
                    "\
Git metadata is missing, but out-of-repo builds are not enabled!
\tYou can enable them by setting `preprocessor.pandocs.out-of-repo` to `true`.
\t(Consider using an environment variable for this:
\t https://rust-lang.github.io/mdBook/format/configuration/environment-variables.html)"
                );
            }
        }

        Ok(())
    }

    const MARKDOWN_OPTIONS: pulldown_cmark::Options = pulldown_cmark::Options::ENABLE_TABLES
        .union(pulldown_cmark::Options::ENABLE_FOOTNOTES)
        .union(pulldown_cmark::Options::ENABLE_STRIKETHROUGH);
}

impl Preprocessor for Pandocs {
    fn name(&self) -> &'static str {
        "pandocs-preproc"
    }

    fn run(&self, ctx: &PreprocessorContext, mut book: Book) -> Result<Book> {
        let is_out_of_repo = match ctx.config.get::<bool>("preprocessor.pandocs.out-of-repo") {
            Ok(Some(boolean)) => boolean,
            Err(_) | Ok(None) => false, // Assume in-repo unless otherwise specified.
        };

        let link_map = anchors::InternalLinkMap::new(&book);

        let mut res = Ok(());
        // Note that this skips draft chapters!
        book.for_each_chapter_mut(|chapter| {
            if res.is_ok() {
                res = self
                    .process_chapter(
                        &chapter.name,
                        &mut chapter.content,
                        &link_map,
                        is_out_of_repo,
                    )
                    .with_context(|| format!("while processing chapter \"{}\"", chapter.name));
            }
        });
        res.map(|_| book)
    }
}

/// Test utilities.
#[cfg(test)]
impl Pandocs {
    fn test_processing_chapter(contents: &mut String) -> Result<()> {
        const EMPTY_LINK_MAP: anchors::InternalLinkMap = anchors::InternalLinkMap::empty();

        let preproc = Pandocs::new();
        preproc.process_chapter("Test Chapter", contents, &EMPTY_LINK_MAP, true)
    }
}
