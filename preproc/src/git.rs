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
    hash: String,
    short_hash: String,
    timestamp: String,
}

impl Commit {
    pub fn rev_parse(what: &str) -> Result<Self, Error> {
        let output = Command::new("git")
            .args(["rev-parse", what])
            .stderr(Stdio::inherit())
            .stdin(Stdio::null())
            .output()
            .expect("Failed to get commit hash");
        if !output.status.success() {
            return Err(Error::msg(format!(
                "Git exited with {} while getting commit hash",
                output.status
            )));
        }
        let hash = String::from_utf8(output.stdout).expect("Commit hash is not valid UTF-8??");

        let output = Command::new("git")
            .args(["rev-parse", "--short", what])
            .stderr(Stdio::inherit())
            .stdin(Stdio::null())
            .output()
            .expect("Failed to get short commit hash");
        if !output.status.success() {
            return Err(Error::msg(format!(
                "Git exited with status {} while getting commit short hash",
                output.status
            )));
        }
        let short_hash =
            String::from_utf8(output.stdout).expect("Commit hash is not valid UTF-8??");

        let output = Command::new("git")
            .args(["show", "-s", "--format=%ci", what])
            .stderr(Stdio::inherit())
            .stdin(Stdio::null())
            .output()
            .expect("Failed to get timestamp");
        if !output.status.success() {
            return Err(Error::msg(format!(
                "Git exited with status {} while getting timestamp",
                output.status
            )));
        }
        let timestamp = String::from_utf8(output.stdout).expect("Commit hash is not valid UTF-8??");

        Ok(Self {
            hash,
            short_hash,
            timestamp,
        })
    }

    pub fn hash(&self) -> &str {
        self.hash.trim()
    }

    pub fn short_hash(&self) -> &str {
        self.short_hash.trim()
    }

    pub fn timestamp(&self) -> &str {
        self.timestamp.trim()
    }
}
