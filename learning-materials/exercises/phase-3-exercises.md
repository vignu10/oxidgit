# Phase 3 Exercises: Rust Fundamentals

**Prerequisites**: Complete Phase 3 (Lessons 08-11)

These exercises will solidify your Rust skills in preparation for building Git.

## 🎯 Exercise Set 1: Rust Basics

### Exercise 1.1: SHA-1 Hash Function

Write a Rust function that computes the SHA-1 hash of a byte slice.

```rust
use sha1::{Sha1, Digest};

fn hash_bytes(data: &[u8]) -> String {
    // TODO: Implement
}

fn main() {
    let data = b"Hello World";
    let hash = hash_bytes(data);
    println!("{}", hash);
    // Should print: 0a4d55a8d778e5022fab701977c5d840bbc486d0
}
```

<details>
<summary>Solution</summary>

```rust
use sha1::{Sha1, Digest};

fn hash_bytes(data: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}
```
</details>

### Exercise 1.2: Blob Object Creator

Create a function that takes file content and returns Git blob format.

```rust
fn create_blob(content: &[u8]) -> Vec<u8> {
    // TODO: Create "blob [size]\0[content]"
}

#[test]
fn test_blob() {
    let content = b"Hello";
    let blob = create_blob(content);
    assert_eq!(&blob[..7], b"blob 5\0");
    assert_eq!(&blob[7..], b"Hello");
}
```

### Exercise 1.3: Ownership Exercise

Fix the ownership errors in this code:

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;
    println!("{}", s1);  // Error! Why?

    let v1 = vec![1, 2, 3];
    print_vec(v1);
    println!("{:?}", v1);  // Error! Why?
}

fn print_vec(v: Vec<i32>) {
    println!("{:?}", v);
}
```

<details>
<summary>Solution</summary>

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1.clone();  // Or use &s1 and &s2
    println!("{}", s1);

    let v1 = vec![1, 2, 3];
    print_vec(&v1);  // Borrow instead of moving
    println!("{:?}", v1);
}

fn print_vec(v: &Vec<i32>) {  // Take reference
    println!("{:?}", v);
}
```
</details>

## 🎯 Exercise Set 2: Error Handling

### Exercise 2.1: Safe File Reader

Implement a function that reads a file with proper error handling:

```rust
use anyhow::{Context, Result};
use std::fs;

fn read_git_file(path: &str) -> Result<String> {
    // TODO: Read file with helpful error message
}

fn main() -> Result<()> {
    match read_git_file(".git/HEAD") {
        Ok(content) => println!("HEAD: {}", content),
        Err(e) => eprintln!("Error: {}", e),
    }
    Ok(())
}
```

<details>
<summary>Solution</summary>

```rust
use anyhow::{Context, Result};
use std::fs;

fn read_git_file(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read {}", path))
}
```
</details>

### Exercise 2.2: Custom Error Type

Create a custom error type for Git operations:

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    // TODO: Add variants for:
    // - Object not found
    // - Invalid hash
    // - IO error
    // - Parse error
}

fn find_object(hash: &str) -> Result<Vec<u8>, GitError> {
    if hash.len() != 40 {
        // Return InvalidHash error
    }
    // ...
}
```

<details>
<summary>Solution</summary>

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Object not found: {0}")]
    ObjectNotFound(String),

    #[error("Invalid hash: {0}")]
    InvalidHash(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Parse error: {0}")]
    Parse(String),
}

fn find_object(hash: &str) -> Result<Vec<u8>, GitError> {
    if hash.len() != 40 {
        return Err(GitError::InvalidHash(hash.to_string()));
    }
    // ...
    Ok(vec![])
}
```
</details>

### Exercise 2.3: Error Chain

Create a function chain with proper error context:

```rust
use anyhow::{Context, Result};

fn read_commit(hash: &str) -> Result<String> {
    let data = read_object(hash)
        .context("Failed to read object")?;

    let decompressed = decompress(&data)
        .context("Failed to decompress")?;

    parse_commit(&decompressed)
        .context("Failed to parse commit")
}

// Implement the helper functions
fn read_object(hash: &str) -> Result<Vec<u8>> {
    // TODO
}

fn decompress(data: &[u8]) -> Result<Vec<u8>> {
    // TODO
}

fn parse_commit(data: &[u8]) -> Result<String> {
    // TODO
}
```

## 🎯 Exercise Set 3: File I/O

### Exercise 3.1: Object Path Builder

Implement a function that builds the correct path for a Git object:

```rust
use std::path::{Path, PathBuf};

fn get_object_path(git_dir: &Path, hash: &str) -> PathBuf {
    // TODO: Build path like .git/objects/ab/cdef...
}

#[test]
fn test_object_path() {
    let path = get_object_path(Path::new(".git"), "abcdef1234567890");
    assert_eq!(
        path,
        PathBuf::from(".git/objects/ab/cdef1234567890")
    );
}
```

### Exercise 3.2: Object Writer

Write a complete object writer:

```rust
use anyhow::Result;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use std::fs;
use std::io::Write;
use std::path::Path;

fn write_object(git_dir: &Path, hash: &str, data: &[u8]) -> Result<()> {
    // TODO:
    // 1. Compress data
    // 2. Create parent directory
    // 3. Write compressed data to correct path
}
```

### Exercise 3.3: Directory Scanner

Scan all objects in a Git repository:

```rust
use std::fs;
use std::path::Path;
use anyhow::Result;

fn scan_objects(git_dir: &Path) -> Result<Vec<String>> {
    let mut hashes = Vec::new();
    // TODO: Walk .git/objects and collect all hashes
    Ok(hashes)
}

fn main() -> Result<()> {
    let objects = scan_objects(Path::new(".git"))?;
    println!("Found {} objects", objects.len());
    Ok(())
}
```

## 🎯 Exercise Set 4: Parsing and Serialization

### Exercise 4.1: Header Parser

Parse Git object headers:

```rust
use anyhow::{Context, Result};

fn parse_header(data: &[u8]) -> Result<(String, usize, usize)> {
    // TODO: Parse "type size\0" and return (type, size, content_offset)
}

#[test]
fn test_parse_header() {
    let data = b"blob 11\0Hello World";
    let (obj_type, size, offset) = parse_header(data).unwrap();
    assert_eq!(obj_type, "blob");
    assert_eq!(size, 11);
    assert_eq!(offset, 8);
}
```

<details>
<summary>Solution</summary>

```rust
use anyhow::{Context, Result, bail};

fn parse_header(data: &[u8]) -> Result<(String, usize, usize)> {
    let null_pos = data.iter()
        .position(|&b| b == 0)
        .context("No null byte in header")?;

    let header = std::str::from_utf8(&data[..null_pos])?;
    let parts: Vec<&str> = header.split(' ').collect();

    if parts.len() != 2 {
        bail!("Invalid header format");
    }

    let obj_type = parts[0].to_string();
    let size: usize = parts[1].parse()?;
    let offset = null_pos + 1;

    Ok((obj_type, size, offset))
}
```
</details>

### Exercise 4.2: Commit Parser

Parse a commit object:

```rust
#[derive(Debug)]
struct Commit {
    tree: String,
    parents: Vec<String>,
    author: String,
    committer: String,
    message: String,
}

fn parse_commit(data: &[u8]) -> Result<Commit> {
    // TODO: Parse commit format
}

#[test]
fn test_parse_commit() {
    let data = b"tree abc123\nauthor John <john@example.com> 123 +0000\ncommitter John <john@example.com> 123 +0000\n\nTest commit\n";
    let commit = parse_commit(data).unwrap();
    assert_eq!(commit.tree, "abc123");
    assert_eq!(commit.message, "Test commit");
}
```

### Exercise 4.3: Tree Entry Parser

Parse tree entries:

```rust
#[derive(Debug)]
struct TreeEntry {
    mode: String,
    name: String,
    hash: [u8; 20],
}

fn parse_tree_entry(data: &[u8]) -> Result<(TreeEntry, usize)> {
    // TODO: Parse "mode name\0[20-byte hash]"
    // Return entry and bytes consumed
}
```

### Exercise 4.4: Object Serializer

Create a trait for serializable Git objects:

```rust
trait GitObject {
    fn object_type(&self) -> &str;
    fn serialize_content(&self) -> Vec<u8>;

    fn serialize(&self) -> Vec<u8> {
        let content = self.serialize_content();
        let header = format!("{} {}\0", self.object_type(), content.len());

        let mut data = header.into_bytes();
        data.extend_from_slice(&content);
        data
    }
}

struct Blob {
    content: Vec<u8>,
}

impl GitObject for Blob {
    fn object_type(&self) -> &str {
        "blob"
    }

    fn serialize_content(&self) -> Vec<u8> {
        self.content.clone()
    }
}

// TODO: Implement for Tree and Commit
```

## 🎯 Exercise Set 5: Integration Exercises

### Exercise 5.1: Complete Object Reader

Build a complete object reader that:
1. Takes a hash
2. Reads from `.git/objects/`
3. Decompresses
4. Parses header
5. Returns content

```rust
use anyhow::Result;
use std::path::Path;

fn read_object(git_dir: &Path, hash: &str) -> Result<(String, Vec<u8>)> {
    // TODO: Complete implementation
}

fn main() -> Result<()> {
    let (obj_type, content) = read_object(
        Path::new(".git"),
        "557db03de997c86a4a028e1ebd3a1ceb225be238"
    )?;

    println!("Type: {}", obj_type);
    println!("Content: {}", String::from_utf8_lossy(&content));
    Ok(())
}
```

### Exercise 5.2: Complete Object Writer

Build a complete object writer that:
1. Takes object type and content
2. Creates object format
3. Computes hash
4. Compresses
5. Writes to `.git/objects/`

```rust
use anyhow::Result;
use std::path::Path;

fn write_object(
    git_dir: &Path,
    obj_type: &str,
    content: &[u8]
) -> Result<String> {
    // TODO: Complete implementation
    // Return the hash
}

fn main() -> Result<()> {
    let hash = write_object(
        Path::new(".git"),
        "blob",
        b"Hello World"
    )?;

    println!("Created object: {}", hash);
    Ok(())
}
```

### Exercise 5.3: Repository Structure

Create a `Repository` struct:

```rust
use anyhow::Result;
use std::path::{Path, PathBuf};

struct Repository {
    git_dir: PathBuf,
    work_tree: PathBuf,
}

impl Repository {
    fn open(path: impl AsRef<Path>) -> Result<Self> {
        // TODO: Validate .git exists
    }

    fn read_object(&self, hash: &str) -> Result<(String, Vec<u8>)> {
        // TODO: Read object
    }

    fn write_object(&self, obj_type: &str, content: &[u8]) -> Result<String> {
        // TODO: Write object
    }

    fn read_ref(&self, ref_name: &str) -> Result<String> {
        // TODO: Read reference
    }
}
```

## 🎯 Challenge Exercises

### Challenge 1: Cat-File Implementation

Implement `git cat-file -p`:

```rust
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short = 'p')]
    pretty_print: bool,

    object: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // TODO: Implement cat-file
    Ok(())
}
```

### Challenge 2: Hash-Object Implementation

Implement `git hash-object -w`:

```rust
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
struct Args {
    #[arg(short = 'w')]
    write: bool,

    file: String,
}

fn main() -> Result<()> {
    let args = Args::parse();
    // TODO: Implement hash-object
    Ok(())
}
```

### Challenge 3: Object Verifier

Build a tool that verifies object integrity:

```rust
use anyhow::Result;

fn verify_object(hash: &str) -> Result<bool> {
    // TODO:
    // 1. Read object
    // 2. Decompress
    // 3. Compute hash of decompressed data
    // 4. Compare with expected hash
}

fn main() -> Result<()> {
    let objects = scan_all_objects()?;
    let mut corrupted = 0;

    for hash in objects {
        if !verify_object(&hash)? {
            println!("Corrupted: {}", hash);
            corrupted += 1;
        }
    }

    println!("Verified {} objects, {} corrupted", objects.len(), corrupted);
    Ok(())
}
```

## 📊 Project: Git Library

Build a reusable Git library in Rust:

### Core Modules:

```rust
// src/lib.rs
pub mod objects;
pub mod repository;
pub mod refs;
pub mod utils;

// src/objects.rs
pub trait GitObject { /* ... */ }
pub struct Blob { /* ... */ }
pub struct Tree { /* ... */ }
pub struct Commit { /* ... */ }

// src/repository.rs
pub struct Repository { /* ... */ }

// src/refs.rs
pub fn read_ref(path: &Path) -> Result<String> { /* ... */ }
pub fn write_ref(path: &Path, hash: &str) -> Result<()> { /* ... */ }

// src/utils.rs
pub fn hash_data(data: &[u8]) -> String { /* ... */ }
pub fn compress(data: &[u8]) -> Result<Vec<u8>> { /* ... */ }
pub fn decompress(data: &[u8]) -> Result<Vec<u8>> { /* ... */ }
```

### Tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_write_read_blob() {
        let dir = tempdir().unwrap();
        let repo = Repository::init(dir.path()).unwrap();

        let hash = repo.write_object("blob", b"test").unwrap();
        let (obj_type, content) = repo.read_object(&hash).unwrap();

        assert_eq!(obj_type, "blob");
        assert_eq!(content, b"test");
    }
}
```

## ✅ Completion Checklist

**Rust Basics**
- [ ] Exercise 1.1: SHA-1 Hash Function
- [ ] Exercise 1.2: Blob Object Creator
- [ ] Exercise 1.3: Ownership Exercise

**Error Handling**
- [ ] Exercise 2.1: Safe File Reader
- [ ] Exercise 2.2: Custom Error Type
- [ ] Exercise 2.3: Error Chain

**File I/O**
- [ ] Exercise 3.1: Object Path Builder
- [ ] Exercise 3.2: Object Writer
- [ ] Exercise 3.3: Directory Scanner

**Parsing and Serialization**
- [ ] Exercise 4.1: Header Parser
- [ ] Exercise 4.2: Commit Parser
- [ ] Exercise 4.3: Tree Entry Parser
- [ ] Exercise 4.4: Object Serializer

**Integration**
- [ ] Exercise 5.1: Complete Object Reader
- [ ] Exercise 5.2: Complete Object Writer
- [ ] Exercise 5.3: Repository Structure

**Challenges**
- [ ] Challenge 1: Cat-File Implementation
- [ ] Challenge 2: Hash-Object Implementation
- [ ] Challenge 3: Object Verifier

**Project**
- [ ] Git Library

## 🎓 Learning Outcomes

After completing these exercises, you should be able to:

✅ Write idiomatic Rust code
✅ Handle errors properly with Result and anyhow
✅ Work with files and binary data
✅ Parse complex data structures
✅ Build a reusable library

## 🔜 Next Steps

Once you've completed Phase 3 exercises, you're ready to build Git!

**→ [Phase 4: Building Git](../phase-4-building-git/12-project-setup-and-architecture.md)**

---

**Phase**: 3 - Rust Fundamentals
**Difficulty**: Intermediate
**Estimated Time**: 10-15 hours
