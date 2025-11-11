# Lesson 09: Rust Error Handling

**Estimated Time**: 2-3 hours
**Prerequisites**: Lesson 08 (Rust Basics)

## 🎯 Learning Objectives

By the end of this lesson, you will:

1. Understand Rust's error handling philosophy
2. Master `Result<T, E>` and `Option<T>`
3. Use the `?` operator effectively
4. Create custom error types
5. Use `anyhow` and `thiserror` crates
6. Handle errors in Git implementation scenarios

## 🚫 No Exceptions in Rust

Unlike many languages, Rust doesn't have exceptions. Instead:

**Other languages**:
```python
try:
    file = open("config.txt")
except FileNotFoundError:
    print("File not found!")
```

**Rust**:
```rust
match std::fs::read_to_string("config.txt") {
    Ok(content) => println!("{}", content),
    Err(e) => println!("Error: {}", e),
}
```

### Why No Exceptions?

1. **Explicit**: You must handle errors or explicitly ignore them
2. **Type-safe**: Errors are part of the type system
3. **No hidden control flow**: No invisible `throw` jumping out
4. **Composable**: Easy to chain operations that might fail

## 📦 Option<T>: Handling Absence

`Option<T>` represents a value that might not exist:

```rust
enum Option<T> {
    Some(T),    // Value exists
    None,       // Value doesn't exist
}
```

### Basic Usage

```rust
fn find_user(id: u32) -> Option<String> {
    if id == 1 {
        Some(String::from("Alice"))
    } else {
        None
    }
}

fn main() {
    // Pattern matching
    match find_user(1) {
        Some(name) => println!("Found: {}", name),
        None => println!("User not found"),
    }

    // Using if let
    if let Some(name) = find_user(1) {
        println!("Found: {}", name);
    }

    // Unwrap (panics if None!)
    let name = find_user(1).unwrap();

    // Unwrap with default
    let name = find_user(999).unwrap_or(String::from("Guest"));

    // Unwrap with fallback function
    let name = find_user(999).unwrap_or_else(|| String::from("Guest"));
}
```

### Git Example: Finding a Ref

```rust
use std::fs;
use std::path::Path;

fn read_ref(ref_name: &str) -> Option<String> {
    let path = format!(".git/refs/heads/{}", ref_name);
    fs::read_to_string(path).ok()  // Convert Result to Option
}

fn main() {
    match read_ref("main") {
        Some(hash) => println!("main points to: {}", hash.trim()),
        None => println!("Branch 'main' not found"),
    }
}
```

## ✅ Result<T, E>: Handling Errors

`Result<T, E>` represents an operation that can succeed or fail:

```rust
enum Result<T, E> {
    Ok(T),      // Success with value T
    Err(E),     // Failure with error E
}
```

### Basic Usage

```rust
use std::fs::File;
use std::io::Read;

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;  // ? operator (we'll cover this soon!)
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

fn main() {
    match read_file("hello.txt") {
        Ok(content) => println!("File content: {}", content),
        Err(e) => println!("Error reading file: {}", e),
    }
}
```

## 🎯 The ? Operator: Error Propagation

The `?` operator is Rust's secret weapon for error handling.

### What ? Does

```rust
// Without ?
fn read_config() -> Result<String, std::io::Error> {
    let file = match File::open("config.txt") {
        Ok(f) => f,
        Err(e) => return Err(e),  // Propagate error
    };

    let mut content = String::new();
    match file.read_to_string(&mut content) {
        Ok(_) => {},
        Err(e) => return Err(e),  // Propagate error
    }

    Ok(content)
}

// With ? (equivalent!)
fn read_config() -> Result<String, std::io::Error> {
    let mut file = File::open("config.txt")?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
```

**The `?` operator**:
1. If `Ok(value)` → unwraps and returns `value`
2. If `Err(e)` → returns `Err(e)` from the function immediately

### Requirements for ?

The function must return `Result` or `Option`:

```rust
// ✅ Works - returns Result
fn process() -> Result<i32, std::io::Error> {
    let content = std::fs::read_to_string("num.txt")?;
    Ok(content.parse().unwrap())
}

// ❌ Doesn't work - main returns ()
fn main() {
    let content = std::fs::read_to_string("num.txt")?;  // Error!
}

// ✅ Works - main returns Result
fn main() -> Result<(), std::io::Error> {
    let content = std::fs::read_to_string("num.txt")?;
    println!("{}", content);
    Ok(())
}
```

### Git Example: Reading Object

```rust
use std::fs;
use std::io;

fn read_object(hash: &str) -> Result<Vec<u8>, io::Error> {
    let (dir, file) = hash.split_at(2);
    let path = format!(".git/objects/{}/{}", dir, file);
    let compressed = fs::read(path)?;  // Propagate error

    // Decompress (simplified)
    Ok(compressed)
}

fn main() -> Result<(), io::Error> {
    let data = read_object("557db03de997c86a4a028e1ebd3a1ceb225be238")?;
    println!("Read {} bytes", data.len());
    Ok(())
}
```

## 🔧 Custom Error Types

For complex applications, create custom error types:

### Basic Custom Error

```rust
use std::fmt;

#[derive(Debug)]
enum GitError {
    ObjectNotFound(String),
    InvalidHash(String),
    CorruptedObject(String),
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GitError::ObjectNotFound(hash) => {
                write!(f, "Object not found: {}", hash)
            }
            GitError::InvalidHash(hash) => {
                write!(f, "Invalid hash: {}", hash)
            }
            GitError::CorruptedObject(hash) => {
                write!(f, "Corrupted object: {}", hash)
            }
        }
    }
}

impl std::error::Error for GitError {}

// Usage
fn find_object(hash: &str) -> Result<Vec<u8>, GitError> {
    if hash.len() != 40 {
        return Err(GitError::InvalidHash(hash.to_string()));
    }

    // Try to read object...
    Err(GitError::ObjectNotFound(hash.to_string()))
}
```

## 📦 The anyhow Crate

For applications (not libraries), `anyhow` makes error handling easy:

### Setup

Add to `Cargo.toml`:
```toml
[dependencies]
anyhow = "1.0"
```

### Basic Usage

```rust
use anyhow::{Result, Context};
use std::fs;

fn read_config(path: &str) -> Result<String> {
    // Result<T> = Result<T, anyhow::Error>
    let content = fs::read_to_string(path)
        .context("Failed to read config file")?;

    Ok(content)
}

fn parse_config(content: &str) -> Result<Config> {
    // Can return different error types!
    let config: Config = serde_json::from_str(content)
        .context("Failed to parse JSON")?;

    Ok(config)
}

fn main() -> Result<()> {
    let content = read_config("config.json")?;
    let config = parse_config(&content)?;
    println!("Loaded config: {:?}", config);
    Ok(())
}
```

### Git Example with anyhow

```rust
use anyhow::{Context, Result, bail, ensure};
use std::fs;

fn read_object(hash: &str) -> Result<Vec<u8>> {
    // Validate hash length
    ensure!(hash.len() == 40, "Invalid hash length: expected 40, got {}", hash.len());

    // Read object
    let (dir, file) = hash.split_at(2);
    let path = format!(".git/objects/{}/{}", dir, file);

    let compressed = fs::read(&path)
        .with_context(|| format!("Failed to read object {}", hash))?;

    if compressed.is_empty() {
        bail!("Object {} is empty", hash);
    }

    Ok(compressed)
}

fn main() -> Result<()> {
    let data = read_object("557db03de997c86a4a028e1ebd3a1ceb225be238")?;
    println!("Success: {} bytes", data.len());
    Ok(())
}
```

**anyhow features**:
- `Result<T>` is an alias for `Result<T, anyhow::Error>`
- `.context()` adds context to errors
- `bail!()` returns an error immediately
- `ensure!()` like `assert!()` but returns error instead of panicking

## 🎨 The thiserror Crate

For libraries, use `thiserror` to create ergonomic error types:

### Setup

Add to `Cargo.toml`:
```toml
[dependencies]
thiserror = "1.0"
```

### Usage

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GitError {
    #[error("Object not found: {0}")]
    ObjectNotFound(String),

    #[error("Invalid hash: {0}")]
    InvalidHash(String),

    #[error("IO error")]
    Io(#[from] std::io::Error),

    #[error("Decompression failed")]
    Decompression(#[from] DecompressionError),
}

fn read_object(hash: &str) -> Result<Vec<u8>, GitError> {
    if hash.len() != 40 {
        return Err(GitError::InvalidHash(hash.to_string()));
    }

    let (dir, file) = hash.split_at(2);
    let path = format!(".git/objects/{}/{}", dir, file);

    // std::io::Error automatically converts to GitError::Io
    let data = std::fs::read(path)?;

    if data.is_empty() {
        return Err(GitError::ObjectNotFound(hash.to_string()));
    }

    Ok(data)
}
```

**thiserror features**:
- `#[error("...")]` generates `Display` implementation
- `#[from]` generates automatic conversions from other error types
- Implements `std::error::Error` automatically

## 🔄 Converting Between Error Types

### From Result to Option

```rust
let result: Result<i32, _> = "42".parse();
let option: Option<i32> = result.ok();  // Ok(42) → Some(42), Err(_) → None
```

### From Option to Result

```rust
let option: Option<i32> = Some(42);
let result: Result<i32, &str> = option.ok_or("No value");  // Some(42) → Ok(42)

let option: Option<i32> = None;
let result: Result<i32, String> = option.ok_or_else(|| format!("Value not found"));
```

### Chaining Operations

```rust
use std::fs;

fn get_config_value(key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let content = fs::read_to_string("config.json")?;
    let json: serde_json::Value = serde_json::from_str(&content)?;

    json.get(key)
        .and_then(|v| v.as_str())  // Option<&str>
        .map(|s| s.to_string())     // Option<String>
        .ok_or_else(|| format!("Key '{}' not found", key).into())  // Result
}
```

## 🧪 Practical Git Examples

### Example 1: Read and Parse Blob

```rust
use anyhow::{Context, Result};
use std::fs;

fn read_blob(hash: &str) -> Result<Vec<u8>> {
    // Read compressed object
    let (dir, file) = hash.split_at(2);
    let path = format!(".git/objects/{}/{}", dir, file);

    let compressed = fs::read(&path)
        .with_context(|| format!("Failed to read blob {}", hash))?;

    // Decompress
    let data = decompress(&compressed)
        .context("Failed to decompress blob")?;

    // Parse header
    let (header, content) = parse_object(&data)
        .context("Failed to parse blob")?;

    if header.object_type != "blob" {
        anyhow::bail!("Expected blob, got {}", header.object_type);
    }

    Ok(content)
}
```

### Example 2: Write Object with Validation

```rust
use anyhow::{Result, ensure};
use std::fs;

fn write_object(object_type: &str, content: &[u8]) -> Result<String> {
    // Validate
    ensure!(
        matches!(object_type, "blob" | "tree" | "commit" | "tag"),
        "Invalid object type: {}",
        object_type
    );

    // Create object data
    let header = format!("{} {}\0", object_type, content.len());
    let mut data = header.into_bytes();
    data.extend_from_slice(content);

    // Hash
    let hash = compute_hash(&data);

    // Compress and write
    let compressed = compress(&data)?;
    let (dir, file) = hash.split_at(2);

    fs::create_dir_all(format!(".git/objects/{}", dir))
        .context("Failed to create objects directory")?;

    fs::write(format!(".git/objects/{}/{}", dir, file), compressed)
        .context("Failed to write object")?;

    Ok(hash)
}
```

### Example 3: Repository Operations

```rust
use anyhow::{Context, Result};
use std::path::{Path, PathBuf};

struct Repository {
    git_dir: PathBuf,
}

impl Repository {
    fn open(path: impl AsRef<Path>) -> Result<Self> {
        let git_dir = path.as_ref().join(".git");

        anyhow::ensure!(
            git_dir.exists(),
            "Not a git repository: {}",
            path.as_ref().display()
        );

        Ok(Repository { git_dir })
    }

    fn read_head(&self) -> Result<String> {
        let head_path = self.git_dir.join("HEAD");

        let content = std::fs::read_to_string(&head_path)
            .context("Failed to read HEAD")?;

        if let Some(ref_path) = content.strip_prefix("ref: ") {
            let ref_path = self.git_dir.join(ref_path.trim());
            std::fs::read_to_string(&ref_path)
                .with_context(|| format!("Failed to read ref: {}", ref_path.display()))
        } else {
            Ok(content.trim().to_string())
        }
    }
}

fn main() -> Result<()> {
    let repo = Repository::open(".")
        .context("Failed to open repository")?;

    let head = repo.read_head()
        .context("Failed to get HEAD")?;

    println!("HEAD: {}", head);
    Ok(())
}
```

## 🎓 Key Takeaways

1. **No exceptions**: Rust uses `Result` and `Option` for error handling
2. **? operator**: Simplifies error propagation in functions returning `Result`/`Option`
3. **anyhow**: Perfect for applications, provides easy error handling
4. **thiserror**: Perfect for libraries, creates ergonomic custom errors
5. **Explicit errors**: All potential failures are visible in type signatures
6. **Composable**: Easy to chain operations that might fail

## 📋 Error Handling Patterns

### When to Use What

| Situation | Use |
|-----------|-----|
| Application code | `anyhow::Result<T>` |
| Library code | `thiserror` + custom errors |
| Value might not exist | `Option<T>` |
| Operation might fail | `Result<T, E>` |
| Want to add context | `.context()` |
| Need to return error immediately | `bail!()` or `return Err()` |
| Assertion with error | `ensure!()` |

### Pattern: Error Context Chain

```rust
use anyhow::{Context, Result};

fn process_commit() -> Result<()> {
    read_repo()
        .context("Failed to read repository")?;

    parse_commit()
        .context("Failed to parse commit")?;

    validate_signature()
        .context("Failed to validate signature")?;

    Ok(())
}

// Error output shows full context:
// Error: Failed to read repository
// Caused by:
//     Failed to open .git directory
// Caused by:
//     Permission denied (os error 13)
```

## ✅ Checkpoint Quiz

1. What's the difference between `Option<T>` and `Result<T, E>`?
2. What does the `?` operator do?
3. When should you use `anyhow` vs `thiserror`?
4. How do you add context to an error?
5. What's the difference between `unwrap()` and `?`?

<details>
<summary>Click to see answers</summary>

1. `Option` represents presence/absence; `Result` represents success/failure with error info
2. Unwraps `Ok`/`Some` or returns `Err`/`None` early from the function
3. `anyhow` for applications (end product), `thiserror` for libraries (reusable code)
4. Use `.context("description")` or `.with_context(|| format!("..."))`
5. `unwrap()` panics on error, `?` returns the error from the function
</details>

## 🧪 Hands-On Exercises

### Exercise 1: Safe File Reader

Write a function that:
1. Reads a file
2. Returns `Result<String, std::io::Error>`
3. Uses the `?` operator

### Exercise 2: Git Object Validator

Create a function that:
1. Takes a hash string
2. Validates it's 40 characters
3. Checks if the object exists
4. Returns appropriate errors using `anyhow`

### Exercise 3: Custom Error Type

Create a `GitError` enum using `thiserror` with variants for:
- ObjectNotFound
- InvalidFormat
- IoError (with `#[from]`)

## 🔜 Next Lesson

Now you can handle errors properly! Next, we'll learn about file I/O and path handling in Rust.

**→ [Lesson 10: Working with Files and Paths in Rust](10-rust-file-io.md)**

---

**Phase**: 3 - Rust Fundamentals
**Lesson**: 09 of 20
