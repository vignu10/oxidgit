//! Git object types and traits
//!
//! This module defines the four Git object types:
//! - Blob: File content
//! - Tree: Directory structure
//! - Commit: Snapshot with metadata
//! - Tag: Named reference to a commit

pub mod object;
// Uncomment as you implement each type
// pub mod blob;
// pub mod tree;
// pub mod commit;

pub use object::{GitObject, ObjectType};
// pub use blob::Blob;
// pub use tree::Tree;
// pub use commit::Commit;
