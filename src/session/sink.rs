//! Session artifact storage and progress reporting.

use anyhow::{Context, Result};
use serde::Serialize;

/// Destination for session artifacts and progress messages.
pub trait Sink: Send + Sync {
    /// Saves `contents` under artifact `name` as soon as it is available.
    fn artifact(&self, name: &str, contents: &[u8]) -> Result<()>;
    /// Receives progress messages during a solve, including diagnostics before a failure.
    fn progress(&self, message: &str);
    /// Whether this sink stores artifacts. If false, expensive artifact serializations can be skipped.
    fn wants_artifacts(&self) -> bool {
        true
    }
}

pub(crate) struct Discard;

impl Sink for Discard {
    fn artifact(&self, _: &str, _: &[u8]) -> Result<()> {
        Ok(())
    }

    fn progress(&self, _: &str) {}

    fn wants_artifacts(&self) -> bool {
        false
    }
}

impl dyn Sink + '_ {
    /// Saves `value` as pretty-printed JSON under artifact `name`.
    pub(crate) fn json(&self, name: &str, value: &impl Serialize) -> Result<()> {
        if !self.wants_artifacts() {
            return Ok(());
        }
        let json =
            serde_json::to_vec_pretty(value).with_context(|| format!("serializing {name}"))?;
        self.artifact(name, &json)
    }
}
