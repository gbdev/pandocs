/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use anyhow::ensure;
use mdbook_preprocessor::errors::Result;

use crate::Pandocs;

pub fn render_admonition(
    dest: &mut String,
    indentation: &str,
    end_indentation: &str,
    kind: &str,
    title: Option<&str>,
    contents: &str,
) -> Result<()> {
    ensure!(
        matches!(kind, "tip" | "warning" | "danger"),
        "Unknown admonition type \"{kind}\" (must be \"tip\", \"warning\", or \"danger\"",
    );
    ensure!(
        end_indentation == indentation,
        "Admonition's end marker not indented like its start"
    );
    let real_contents = contents
        .strip_circumfix('\n', '\n')
        // Important to maintain blank lines as expected
        .expect("Regex doesn't capture newlines around admonition content!?");
    ensure!(
        real_contents
            .lines()
            .all(|line| line.starts_with(indentation)
                || !line.contains(|ch| !Pandocs::WHITESPACE.contains(&ch))),
        "Admonition's body is less indented than its opening marker",
    );

    use std::fmt::Write; // Note that `write!()`ing to a `String` never fails, so `unwrap()` is fine.

    // Ensure there is a blank line before the start of the admonition, so the `<div>` is not part of a previous paragraph.
    write!(dest, "\n{indentation}<div class=\"box {kind}-box\">").unwrap();
    if let Some(title) = title {
        write!(dest, "<p class=\"box-title\">{title}</p>").unwrap();
    }
    writeln!(dest).unwrap();
    // Since `contents` begins and ends with a newline, this leaves a blank line between it and the HTML lines;
    // this is important so that Markdown processing is not suppressed.
    writeln!(dest, "{contents}").unwrap();
    writeln!(dest, "{indentation}</div>").unwrap();

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::Pandocs;

    use super::*;

    #[test]
    fn it_works() {
        let mut content = "# Test chapter

This one is easy, just serving as a smoke test; trickier cases will come later,
so that if this test fails we know not to look at more complex ones.

:::tip

Admonition!!

:::

And there we go :)"
            .into();
        Pandocs::test_processing_chapter(&mut content).unwrap();
        assert_eq!(
            content,
            r#"# Test chapter

This one is easy, just serving as a smoke test; trickier cases will come later,
so that if this test fails we know not to look at more complex ones.


<div class="box tip-box">


Admonition!!


</div>


And there we go :)"#
        )
    }

    // TODO: more tests
}
