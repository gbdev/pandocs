/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use mdbook_preprocessor::{
    Preprocessor,
    errors::{Error, Result},
};
use pandocs_preproc::Pandocs;
use tracing::warn;

fn main() -> Result<()> {
    tracing_subscriber::fmt()
        .without_time()
        .with_ansi(std::io::IsTerminal::is_terminal(&std::io::stderr()))
        .with_writer(std::io::stderr)
        .with_target(false)
        .init();

    let mut args = std::env::args();
    let _ = args.next(); // Program name.

    let preprocessor = Pandocs::new();

    match args.next().as_deref() {
        None => handle_preprocessing(&preprocessor),

        Some("supports") => {
            let renderer_name = args.next().ok_or_else(|| {
                Error::msg("Missing argument: name of renderer to check for compat")
            })?;
            if !args.next().is_none() {
                return Err(Error::msg(
                    "Unexpected extra CLI args after the renderer's name",
                ));
            }

            handle_supports(&preprocessor, &renderer_name)
        }

        Some(subcmd) => Err(Error::msg(format!("Unknown subcommand `{subcmd}`"))),
    }
}

fn handle_preprocessing(pre: &Pandocs) -> Result<()> {
    let (ctx, book) = mdbook_preprocessor::parse_input(std::io::stdin())?;

    // We should probably use the `semver` crate to check compatibility here...
    if ctx.mdbook_version != mdbook_preprocessor::MDBOOK_VERSION {
        warn!(
            "The Pan Docs preproc was built against version {} of mdbook, but we're being called from version {}",
            mdbook_preprocessor::MDBOOK_VERSION,
            ctx.mdbook_version,
        );
    }

    let processed_book = pre.run(&ctx, book)?;
    serde_json::to_writer(std::io::stdout(), &processed_book)?;

    Ok(())
}

fn handle_supports(pre: &Pandocs, renderer: &str) -> Result<()> {
    // Signal whether the renderer is supported by exiting with 1 or 0.
    std::process::exit(if pre.supports_renderer(renderer)? {
        0
    } else {
        1
    })
}
