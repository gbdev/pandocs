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

        // Generate the single-page version
        let base_url = Url::parse("http://localhost/").unwrap();
        let mut path = ctx.destination.join(self.name());
        path.set_file_name("print.html");
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
