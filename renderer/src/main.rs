/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use anyhow::Context;
use globwalk::{FileType, GlobWalkerBuilder};
use lazy_static::lazy_static;
use mdbook::book::BookItem;
use mdbook::errors::{Error, Result};
use mdbook::renderer::{HtmlHandlebars, RenderContext, Renderer};
use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Write};
use std::io::{BufRead, BufReader, BufWriter};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use url::Url;

fn main() -> Result<()> {
    let mut stdin = io::stdin();
    let ctx = RenderContext::from_json(&mut stdin).unwrap();

    let renderer = Pandocs;

    if ctx.version != mdbook::MDBOOK_VERSION {
        // We should probably use the `semver` crate to check compatibility
        // here...
        let mut stderr = StandardStream::stderr(ColorChoice::Auto);
        stderr
            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
            .unwrap();
        write!(&mut stderr, "warning:").unwrap();
        stderr.reset().unwrap();
        eprintln!(
            " The {} renderer was built against version {} of mdbook, \
             but we're being called from version {}",
            renderer.name(),
            mdbook::MDBOOK_VERSION,
            ctx.version
        );
    }

    renderer.render(&ctx)
}

struct Pandocs;

impl Renderer for Pandocs {
    fn name(&self) -> &'static str {
        "pandocs"
    }

    fn render(&self, ctx: &RenderContext) -> Result<()> {
        // First, render things using the HTML renderer
        let renderer = HtmlHandlebars;
        renderer.render(ctx)?;

        // Now, post-process the pages in-place to inject the boxes
        let mut path = ctx.destination.join(self.name());
        for (i, item) in ctx.book.iter().enumerate() {
            match item {
                BookItem::Chapter(chapter) if !chapter.is_draft_chapter() => {
                    let page_name = chapter.path.as_ref().unwrap();
                    path.set_file_name(page_name);
                    path.set_extension("html");
                    render(&mut path, &chapter.name, i)
                        .with_context(|| format!("Failed to render {}", &chapter.name))?;
                }

                _ => (),
            }
        }
        // Post-process the print page as well
        path.set_file_name("print.html");
        render(&mut path, "<print>", usize::MAX).context("Failed to render print page")?;

        // Generate the single-page version
        let base_url = Url::parse("http://localhost/").unwrap();
        gen_single_page(&mut path, &base_url).context("Failed to render single-page version")?;

        // Generate the graphs in `imgs/src/` by shelling out to Python
        let working_dir = ctx.destination.join("imgs");
        let src_dir = working_dir.join("src");
        let python = if cfg!(windows) { "python" } else { "python3" };
        let gen_graph = |file_name, title| {
            let mut file_name = PathBuf::from_str(file_name).unwrap(); // Can't fail
            let output = File::create(working_dir.join(&file_name))?;

            file_name.set_extension("csv");
            let status = Command::new(python)
                .current_dir(&src_dir)
                .arg("graph_render.py")
                .arg(&file_name)
                .arg(title)
                .stdout(output)
                .status()
                .with_context(|| format!("Failed to generate \"{}\"", file_name.display()))?;

            if status.success() {
                Ok(())
            } else {
                Err(Error::msg(format!(
                    "Generating \"{}\" failed with {}",
                    file_name.display(),
                    status,
                )))
            }
        };
        gen_graph("MBC5_Rumble_Mild.svg", "Mild Rumble")?;
        gen_graph("MBC5_Rumble_Strong.svg", "Strong Rumble")?;
        fs::remove_dir_all(&src_dir).context(format!("Failed to remove {}", src_dir.display()))?;

        // Scrub off files that need not be published
        for path in GlobWalkerBuilder::from_patterns(&ctx.destination, &[".gitignore", "*.graphml"])
            .file_type(FileType::FILE)
            .build()?
        {
            let path = path?;
            let path = path.path();
            fs::remove_file(path)
                .with_context(|| format!("Failed to remove {}", path.display()))?;
        }

        Ok(())
    }
}

fn render(path: &mut PathBuf, name: &str, index: usize) -> Result<()> {
    // Since we are about to edit the file in-place, we must buffer it into memory
    let html = fs::read_to_string(&path)?;
    // Open the output file, and possibly the output "index.html" file
    let mut output = File::create(&path)?;
    // The index is generated from the first chapter
    let index_file = if index == 0 {
        path.set_file_name("index.html");
        Some(File::create(&path).context(format!("Failed to create {}", path.display()))?)
    } else {
        None
    };
    macro_rules! output {
        ($string:expr) => {
            output
                .write_all($string.as_bytes())
                .context("Failed to write to output file")?;
            index_file
                .as_ref()
                .map(|mut f| f.write_all($string.as_bytes()))
                .transpose()
                .context(format!("Failed to write to index file"))?;
        };
    }

    let mut in_box = false;
    for (i, line) in html.lines().enumerate() {
        let line_no = i + 1;
        // Yes, this relies on how the HTML renderer outputs paragraphs, i.e.
        // that tags are flush with the content.
        // Yes, this sucks, and yes, I hate it.
        // If you have a better idea, please tell us! x_x

        if let Some(line) = line.strip_prefix("<p>:::") {
            if let Some(line) = line.strip_suffix("</p>") {
                let line = line.trim();

                if let Some(box_type) = line.split_whitespace().next() {
                    // This is a box start marker
                    if in_box {
                        return Err(Error::msg(format!(
                            "{}:{}: Attempting to open box inside of one",
                            path.display(),
                            line_no
                        )));
                    }
                    in_box = true;

                    let box_type = if ["tip", "warning", "danger"].contains(&box_type) {
                        box_type
                    } else {
                        let mut stderr = StandardStream::stderr(ColorChoice::Auto);
                        stderr
                            .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
                            .unwrap();
                        write!(&mut stderr, "warning").unwrap();
                        stderr.reset().unwrap();
                        eprintln!(
                            " ({}): unknown box type \"{}\", defaulting to \"tip\"",
                            name, box_type
                        );
                        "tip"
                    };
                    output!(format!("<div class=\"box {}\">\n", box_type));

                    let title = &line[box_type.len()..].trim_start();
                    if !title.is_empty() {
                        output!(format!("<p class=\"box-title\">{}</p>", title));
                    }
                } else {
                    // This is a box ending marker
                    if !in_box {
                        return Err(Error::msg(format!(
                            "{}:{}: Attempting to close box outside of one",
                            path.display(),
                            line_no
                        )));
                    }
                    in_box = false;

                    output!("</div>\n");
                }

                // Prevent normal output
                continue;
            } else {
                let mut stderr = StandardStream::stderr(ColorChoice::Auto);
                stderr
                    .set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true))
                    .unwrap();
                write!(&mut stderr, "warning").unwrap();
                stderr.reset().unwrap();
                eprintln!(" ({}): ignoring \":::{}\"; box start/end tags must be alone in their paragraph", name, line);
            }
        }
        output!(line);
        output!("\n");
    }

    if in_box {
        return Err(Error::msg(format!("{}: Unclosed box", path.display())));
    }

    Ok(())
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
    lazy_static! {
        static ref LINK_RE: Regex =
            Regex::new(r#"<a(?:\s+(?:href="([^"]*)"|\w+="[^"]*"))*\s*>"#).unwrap();
    }

    // HACK: this assumes all link tags span a single line
    let mut lines = print_page.lines();
    while let Some(line) = lines.next().transpose()? {
        let mut i = 0;
        for url_match in LINK_RE.captures_iter(&line).filter_map(|caps| caps.get(1)) {
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
                single_page.write_all(line[i..url_match.start()].as_bytes())?;
                // Write the replaced match
                single_page.write_all("#".as_bytes())?;
                single_page.write_all(frag.as_bytes())?;
                // Start copying after the match
                i = url_match.end();
            }
        }

        // Write rest of line
        single_page.write_all(line[i..].as_bytes())?;
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

    single_page.flush()?;
    Ok(())
}
