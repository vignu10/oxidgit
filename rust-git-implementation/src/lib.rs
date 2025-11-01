//! oxid - A Git implementation in Rust
//!
//! This library provides the core functionality for a Git-like
//! version control system built from scratch for learning purposes.
//!
//! # Architecture
//!
//! oxid is structured around Git's core concepts:
//!
//! - **Objects**: Blobs, trees, commits, and tags stored in content-addressable database
//! - **Repository**: Manages the .git directory and object database
//! - **Index**: The staging area for preparing commits
//! - **Commands**: High-level operations (init, add, commit, etc.)
//!
//! # Example
//!
//! ```no_run
//! use oxid::Repository;
//!
//! // Initialize a new repository
//! let repo = Repository::init("my-project").unwrap();
//!
//! // Repository is now ready to use!
//! ```

pub mod commands;
pub mod objects;
pub mod repository;
pub mod index;
pub mod utils;

// Re-export commonly used types
pub use repository::Repository;
pub use objects::{GitObject, ObjectType};
// Uncomment as you implement each type
// pub use objects::{Blob, Tree, Commit};

/// Current version of oxid
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
