/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use std::{
    fs::{self, File},
    io::{self, BufRead, BufReader, BufWriter, Write},
    path::PathBuf,
};

use anyhow::Context;
use mdbook_renderer::{errors::Result, RenderContext, Renderer};
use regex::Regex;
use tracing::warn;
use url::Url;
use walkdir::{DirEntry, WalkDir};

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr()))
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();

    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    let renderer = Pandocs;

    // We should probably use the `semver` crate to check compatibility here...
    if ctx.version != mdbook_renderer::MDBOOK_VERSION {
        warn!(
            "The {} renderer was built against version {} of mdbook, but we're being called from version {}",
            renderer.name(),
            mdbook_renderer::MDBOOK_VERSION,
            ctx.version,
        );
    }

    renderer.render(&ctx)
}

fn entry_should_be_scrubbed(entry: &DirEntry) -> bool {
    entry.file_name() == ".gitignore" || entry.file_name().as_encoded_bytes().ends_with(b".graphml")
}

struct Pandocs;

impl Renderer for Pandocs {
    fn name(&self) -> &'static str {
        "pandocs"
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        // Copy the HTML renderer's output, except for files we do not seek to publish.
        fs::remove_dir_all(&ctx.destination) // Make sure to avoid any stale files, though!
            .with_context(|| format!("Failed to empty dir {}", ctx.destination.display()))?;
        let html_output_dir = ctx.destination.parent().unwrap().join("html");
        for entry_result in WalkDir::new(&html_output_dir)
            .into_iter()
            .filter_entry(|entry| !entry_should_be_scrubbed(entry))
        {
            let entry = entry_result.with_context(|| {
                format!("Error while iterating on {}", html_output_dir.display())
            })?;

            let dest_path = ctx
                .destination
                .join(entry.path().strip_prefix(&html_output_dir).unwrap());
            if entry.file_type().is_dir() {
                fs::create_dir(&dest_path) // The directory shouldn't already exist, since we start from scratch each time!
                    .with_context(|| format!("Failed to create dir {}", dest_path.display()))?
            } else {
                fs::copy(entry.path(), dest_path)
                    .with_context(|| format!("Failed to copy file {}", entry.path().display()))?;
            }
        }

        // Generate the single-page version
        let base_url = Url::parse("http://localhost/").unwrap();
        let mut path = ctx.destination.join(self.name());
        path.set_file_name("print.html");
        gen_single_page(&mut path, &base_url).context("Failed to render single-page version")?;

        Ok(())
    }
}

/// This generates `single.html` from `print.html`.
/// This does not properly parse HTML, instead relying on crude assumptions about mdBook's output.
/// This is for the sake of performance, as we can afford to update this from time to time.
/// Such assumptions are marked by `HACK:` comments in the function, to at least ease tracability.
fn gen_single_page(path: &mut PathBuf, base_url: &Url) -> Result<()> {
    let print_page = BufReader::new(
        File::open(&path)
            .with_context(|| format!("Failed to open print page \"{}\"", path.display()))?,
    );
    path.set_file_name("single.html");
    let mut single_page = BufWriter::new(File::create(path)?);
    // HACK: this almost certainly forgets a bunch of HTML edge cases
    let link_regex = Regex::new(r#"<a(?:\s+(?:href="([^"]*)"|\w+="[^"]*"))*\s*>"#).unwrap();

    // HACK: this assumes all link tags span a single line
    let mut lines = print_page.lines().peekable();
    let mut removed_placeholder = false;
    while let Some(result) = lines.next() {
        let mut line = result?;
        // Remove the placeholder "Single-page version" chapter (`src/single.md`),
        // which would otherwise be included at the end of the page.
        // HACK: this assumes that each chapter starts with a page break `<div>` line,
        if line
            .trim_start()
            .starts_with(r#"<div style="break-before: page;"#)
            // immediately followed by the chapter's `<h1>` line.
            && lines
                .next_if(|next_line| next_line.is_ok_and(|contents| {
                    contents
                        .trim_start()
                        .starts_with(r#"<h1 id="single-page-version">"#),
                })
                .is_some()
        {
            // Discard lines until the end of the chapter, which is either the next chapter's
            // page break, or the end of the page's content. That line is kept.
            line = lines
                .by_ref()
                .find(|res| match res {
                    Ok(line) => {
                        let line = line.trim_start();
                        line == "</main>"
                            || line.starts_with(r#"<div style="break-before: page;"#)
                    }
                    Err(_) => true,
                })
                .context("Couldn't find the end of the placeholder single-page chapter")??;
            removed_placeholder = true;
        }

        let mut i = 0;
        for url_match in link_regex
            .captures_iter(&line)
            .filter_map(|caps| caps.get(1))
        {
            let url = &line[url_match.range()];

            match Url::parse(url) {
                Ok(_) => continue, // If not a relative URL, skip
                Err(url::ParseError::RelativeUrlWithoutBase) => (),
                Err(e) => return Err(e).with_context(|| format!("Bad link URL \"{}\"", url)), // Return other errors
            }

            let url = base_url
                .join(url)
                .with_context(|| format!("Bad total URL \"{}\"", url))?;
            if let Some(frag) = url.fragment() {
                // Write everything up to the match
                single_page.write_all(&line.as_bytes()[i..url_match.start()])?;
                // Write the replaced match
                single_page.write_all("#".as_bytes())?;
                single_page.write_all(frag.as_bytes())?;
                // Start copying after the match
                i = url_match.end();
            }
        }

        // Write rest of line
        single_page.write_all(&line.as_bytes()[i..])?;
        single_page.write_all("\n".as_bytes())?;

        // Remove the automatic print trigger code.
        // HACK: this assumes the location of the script, the comment's format,
        // and that there is no content between this comment and the script that cannot be
        // passed through.
        if line.trim_start() == "<!-- Custom JS scripts -->" {
            // Pass extra scripts through unchanged
            // HACK: we filter scripts by assuming that mdBook's "additional JS" refs are one-line
            // and the printer script tag isn't.
            lines
                .by_ref()
                .take_while(|line| {
                    line.as_ref().map_or(true, |line| {
                        // HACK: this relies on the exact formatting of the auto-printer's script tag
                        line.trim_start() != "<script>"
                    })
                })
                .try_for_each(|line| {
                    single_page.write_all(line?.as_bytes())?;
                    single_page.write_all("\n".as_bytes())
                })?;

            // Discard lines until the end of the script.
            // Also, check if this does discard lines; if this discards none, we have a problem.
            let auto_printer_script_lines = lines.by_ref().take_while(|line| {
                line.as_ref().map_or(true, |line| {
                    // HACK: this relies on the exact formatting of the auto-printer's script end tag
                    line.trim_start() != "</script>"
                })
            });
            if auto_printer_script_lines.count() == 0 {
                panic!("Warning: unterminated auto-printer script tag??");
            }
        }
    }

    anyhow::ensure!(removed_placeholder, "Did not find the placeholder single-page chapter to remove");
    single_page.flush()?;
    Ok(())
}
