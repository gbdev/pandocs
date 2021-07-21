/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use anyhow::Context;
use mdbook::book::BookItem;
use mdbook::errors::{Error, Result};
use mdbook::renderer::{HtmlHandlebars, RenderContext, Renderer};
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::str::FromStr;

use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};

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
                        .context(format!("Failed to render {}", &chapter.name))?;
                }

                _ => (),
            }
        }
        // Post-process the print page as well
        path.set_file_name("print.html");
        render(&mut path, "<print>", usize::MAX).context("Failed to render print page")?;

        // Generate the graphs in `imgs/src/` by shelling out to Python
        let working_dir = ctx.destination.join("imgs");
        let src_dir = working_dir.join("src");
        let python = if cfg!(windows) { "py3" } else { "python3" };
        let render = |file_name, title| {
            let mut file_name = PathBuf::from_str(file_name).unwrap();
            let output = File::create(working_dir.join(&file_name))?;

            file_name.set_extension("csv");
            let status = Command::new(python)
                .current_dir(&src_dir)
                .arg("graph_render.py")
                .arg(&file_name)
                .arg(title)
                .stdout(output)
                .status()
                .unwrap_or_else(|_| panic!("Failed to generate \"{}\"", file_name.display()));
            if !status.success() {
                return Err(Error::msg(format!(
                    "Generating \"{}\" failed with {}",
                    file_name.display(),
                    status,
                )));
            }
            Ok(())
        };
        render("MBC5_Rumble_Mild.svg", "Mild Rumble")?;
        render("MBC5_Rumble_Strong.svg", "Strong Rumble")?;
        // Delete the source files
        fs::remove_dir_all(&src_dir).context(format!("Failed to remove {}", src_dir.display()))?;

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

                    let box_type = if ["tip", "warning"].contains(&box_type) {
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
