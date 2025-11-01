# Lesson 12: Project Setup and Architecture

**Estimated Time**: 2-3 hours
**Prerequisites**: Phases 1-3 complete

## 🎯 Learning Objectives

By the end of this lesson, you will:

1. Set up the Rust project structure for our Git implementation
2. Understand the modular architecture we'll build
3. Configure dependencies
4. Create the foundational types and modules
5. Have a working project skeleton

## 🏗️ Project Architecture

We'll build **oxid** (Oxidized Git Implementation) with this structure:

```
oxid/
├── Cargo.toml                # Project manifest
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── commands/            # Git commands (init, add, commit, etc.)
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── hash_object.rs
│   │   ├── cat_file.rs
│   │   └── ...
│   ├── objects/             # Git objects (blob, tree, commit, tag)
│   │   ├── mod.rs
│   │   ├── blob.rs
│   │   ├── tree.rs
│   │   ├── commit.rs
│   │   └── object.rs
│   ├── repository.rs        # Repository operations
│   ├── index.rs             # Staging area
│   └── utils.rs             # Helper functions (hashing, compression)
├── tests/                   # Integration tests
│   └── basic_workflow.rs
└── README.md
```

## 📦 Initialize the Project

```bash
cd /home/matrix/oxidgit/rust-git-implementation

# Create new binary + library project
cargo new --lib oxid
cd oxid

# Project structure created!
```

## ⚙️ Configure Dependencies

Edit `Cargo.toml`:

```toml
[package]
name = "oxid"
version = "0.1.0"
edition = "2021"

[[bin]]
name = "oxid"
path = "src/main.rs"

[lib]
name = "oxid"
path = "src/lib.rs"

[dependencies]
# SHA-1 hashing
sha1 = "0.10"

# Compression
flate2 = "1.0"

# Command-line argument parsing
clap = { version = "4.0", features = ["derive"] }

# Error handling
anyhow = "1.0"

# Serialization (for index)
bincode = "1.3"
serde = { version = "1.0", features = ["derive"] }

# File system operations
walkdir = "2.3"

# Time handling
chrono = "0.4"

[dev-dependencies]
# Testing
tempfile = "3.3"
```

## 🏛️ Core Types and Modules

### Create Module Structure

```bash
cd src
mkdir commands objects
touch commands/mod.rs objects/mod.rs
touch repository.rs index.rs utils.rs
```

### src/lib.rs - Library Root

```rust
//! oxid - A Git implementation in Rust
//!
//! This library provides the core functionality for a Git-like
//! version control system.

pub mod commands;
pub mod objects;
pub mod repository;
pub mod index;
pub mod utils;

// Re-export commonly used types
pub use repository::Repository;
pub use objects::{Blob, Tree, Commit, GitObject};
```

### src/main.rs - CLI Entry Point

```rust
use clap::{Parser, Subcommand};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "oxid")]
#[command(about = "A Git implementation in Rust", long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Initialize a new repository
    Init {
        #[arg(default_value = ".")]
        path: String,
    },

    /// Compute object ID and optionally create a blob
    HashObject {
        #[arg(short = 'w')]
        write: bool,

        file: String,
    },

    /// Provide content for repository objects
    CatFile {
        #[arg(short = 't')]
        show_type: bool,

        #[arg(short = 'p')]
        pretty_print: bool,

        object: String,
    },

    /// Create a tree object from the current index
    WriteTree,

    /// Create a new commit object
    Commit {
        #[arg(short = 'm')]
        message: String,
    },
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { path } => {
            println!("Initializing repository in: {}", path);
            oxid::commands::init::run(&path)?;
        }
        Commands::HashObject { write, file } => {
            oxid::commands::hash_object::run(&file, write)?;
        }
        Commands::CatFile {
            show_type,
            pretty_print,
            object,
        } => {
            oxid::commands::cat_file::run(&object, show_type, pretty_print)?;
        }
        Commands::WriteTree => {
            oxid::commands::write_tree::run()?;
        }
        Commands::Commit { message } => {
            oxid::commands::commit::run(&message)?;
        }
    }

    Ok(())
}
```

### src/utils.rs - Helper Functions

```rust
use anyhow::Result;
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::io::{Read, Write};

/// Compute SHA-1 hash of data
pub fn hash_data(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

/// Compress data using zlib
pub fn compress(data: &[u8]) -> Result<Vec<u8>> {
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(data)?;
    Ok(encoder.finish()?)
}

/// Decompress data using zlib
pub fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    let mut decoder = ZlibDecoder::new(data);
    let mut result = Vec::new();
    decoder.read_to_end(&mut result)?;
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_data() {
        let data = b"Hello World";
        let hash = hash_data(data);
        assert_eq!(hash.len(), 40); // SHA-1 produces 40 hex chars
    }

    #[test]
    fn test_compress_decompress() {
        let original = b"Hello World";
        let compressed = compress(original).unwrap();
        let decompressed = decompress(&compressed).unwrap();
        assert_eq!(original.to_vec(), decompressed);
    }
}
```

### src/repository.rs - Repository Structure

```rust
use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents a Git repository
pub struct Repository {
    /// Path to the .git directory
    pub git_dir: PathBuf,
    /// Path to the working directory
    pub work_tree: PathBuf,
}

impl Repository {
    /// Create a new Repository instance
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let work_tree = path.as_ref().canonicalize()?;
        let git_dir = work_tree.join(".git");

        if !git_dir.exists() {
            anyhow::bail!("Not an oxid repository: {}", work_tree.display());
        }

        Ok(Repository {
            git_dir,
            work_tree,
        })
    }

    /// Initialize a new repository
    pub fn init(path: impl AsRef<Path>) -> Result<Self> {
        let work_tree = path.as_ref();
        let git_dir = work_tree.join(".git");

        // Create .git directory structure
        fs::create_dir_all(&git_dir)?;
        fs::create_dir_all(git_dir.join("objects"))?;
        fs::create_dir_all(git_dir.join("refs/heads"))?;
        fs::create_dir_all(git_dir.join("refs/tags"))?;

        // Create HEAD pointing to main
        fs::write(git_dir.join("HEAD"), b"ref: refs/heads/main\n")?;

        // Create config file
        let config = r#"[core]
    repositoryformatversion = 0
    filemode = false
    bare = false
"#;
        fs::write(git_dir.join("config"), config)?;

        // Create description
        fs::write(
            git_dir.join("description"),
            b"Unnamed oxid repository.\n",
        )?;

        Ok(Repository {
            git_dir: git_dir.canonicalize()?,
            work_tree: work_tree.to_path_buf(),
        })
    }

    /// Get path to object file for given hash
    pub fn object_path(&self, hash: &str) -> PathBuf {
        let (dir, file) = hash.split_at(2);
        self.git_dir.join("objects").join(dir).join(file)
    }

    /// Read an object from the database
    pub fn read_object(&self, hash: &str) -> Result<Vec<u8>> {
        let path = self.object_path(hash);
        let compressed = fs::read(&path)
            .with_context(|| format!("Failed to read object {}", hash))?;
        crate::utils::decompress(&compressed)
    }

    /// Write an object to the database
    pub fn write_object(&self, hash: &str, data: &[u8]) -> Result<()> {
        let path = self.object_path(hash);

        // Create parent directory
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Compress and write
        let compressed = crate::utils::compress(data)?;
        fs::write(&path, compressed)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init() {
        let dir = tempdir().unwrap();
        let repo = Repository::init(dir.path()).unwrap();

        assert!(repo.git_dir.join("objects").exists());
        assert!(repo.git_dir.join("refs/heads").exists());
        assert!(repo.git_dir.join("HEAD").exists());
    }
}
```

### src/objects/mod.rs - Object Module

```rust
pub mod blob;
pub mod tree;
pub mod commit;
pub mod object;

pub use blob::Blob;
pub use tree::Tree;
pub use commit::Commit;
pub use object::GitObject;

/// Object types in Git
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
    Tag,
}

impl ObjectType {
    pub fn as_str(&self) -> &str {
        match self {
            ObjectType::Blob => "blob",
            ObjectType::Tree => "tree",
            ObjectType::Commit => "commit",
            ObjectType::Tag => "tag",
        }
    }
}
```

### src/objects/object.rs - Common Object Trait

```rust
use crate::objects::ObjectType;
use crate::utils;
use anyhow::Result;

/// Trait for all Git objects
pub trait GitObject {
    /// Get the object type
    fn object_type(&self) -> ObjectType;

    /// Serialize to bytes (without header)
    fn serialize(&self) -> Result<Vec<u8>>;

    /// Compute the object's hash
    fn hash(&self) -> Result<String> {
        let data = self.to_bytes()?;
        Ok(utils::hash_data(&data))
    }

    /// Convert to bytes with Git object format: [type] [size]\0[content]
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let content = self.serialize()?;
        let header = format!("{} {}\0", self.object_type().as_str(), content.len());

        let mut data = header.as_bytes().to_vec();
        data.extend_from_slice(&content);

        Ok(data)
    }
}
```

## ✅ Test the Setup

```bash
# Build the project
cargo build

# Run tests
cargo test

# Try the CLI
cargo run -- --help
```

You should see:

```
A Git implementation in Rust

Usage: oxid <COMMAND>

Commands:
  init         Initialize a new repository
  hash-object  Compute object ID and optionally create a blob
  cat-file     Provide content for repository objects
  write-tree   Create a tree object from the current index
  commit       Create a new commit object
  help         Print this message or the help of the given subcommand(s)
```

## 🎓 Key Takeaways

1. **Modular architecture**: Separate concerns (commands, objects, repository)
2. **Library + binary**: Core logic in library, CLI in binary
3. **Strong types**: Use enums for object types, structs for entities
4. **Error handling**: Use `Result<T>` and `anyhow` for errors
5. **Testing**: Write tests alongside code

## 🔜 Next Steps

Now we have the foundation! Next, we'll implement our first Git command: `hash-object`.

**→ [Lesson 13: Implementing hash-object](13-implementing-hash-object.md)**

---

**Phase**: 4 - Building Git
**Lesson**: 12 of 20
