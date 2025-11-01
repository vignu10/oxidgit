//! The Git index (staging area)
//!
//! The index is a binary file that tracks which files are staged for the next commit.
//! This is a placeholder implementation - you'll build this in later lessons!

use anyhow::Result;

/// Represents the Git index (staging area)
pub struct Index {
    // TODO: Implement index entries
}

impl Index {
    /// Create a new empty index
    pub fn new() -> Self {
        Index {}
    }

    /// Read index from file
    pub fn read(_path: &str) -> Result<Self> {
        // TODO: Implement index reading
        Ok(Index::new())
    }

    /// Write index to file
    pub fn write(&self, _path: &str) -> Result<()> {
        // TODO: Implement index writing
        Ok(())
    }
}

impl Default for Index {
    fn default() -> Self {
        Self::new()
    }
}
