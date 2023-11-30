/*
 * This Source Code Form is subject to the
 * terms of the Mozilla Public License, v.
 * 2.0. If a copy of the MPL was not
 * distributed with this file, You can
 * obtain one at
 * http://mozilla.org/MPL/2.0/.
 */

use std::process::{Command, Stdio};

use anyhow::Error;

#[derive(Debug)]
pub struct Commit {
    info: String,
    splits: [usize; 2],
}

impl Commit {
    pub fn rev_parse(what: &str) -> Result<Self, Error> {
        let output = Command::new("git")
            .args(["show", "-s", "--format=%H%x00%h%x00%ci", what])
            .stderr(Stdio::inherit())
            .stdin(Stdio::null())
            .output()
            .expect("Failed to get commit info");
        if !output.status.success() {
            return Err(Error::msg(format!(
                "Git exited with status {} while getting commit info",
                output.status
            )));
        }
        let info = String::from_utf8(output.stdout).expect("Commit info is not valid UTF-8??");

        let first_split = info
            .find('\0')
            .expect("Failed to split hash and short hash");
        let second_split = info[first_split + 1..]
            .find('\0')
            .expect("Failed to split short hash and timestamp")
            + first_split;

        Ok(Self {
            info,
            splits: [first_split, second_split],
        })
    }

    pub fn hash(&self) -> &str {
        &self.info[..self.splits[0]]
    }

    pub fn short_hash(&self) -> &str {
        &self.info[self.splits[0] + 1..self.splits[1]]
    }

    pub fn timestamp(&self) -> &str {
        &self.info[self.splits[1] + 1..]
    }
}
