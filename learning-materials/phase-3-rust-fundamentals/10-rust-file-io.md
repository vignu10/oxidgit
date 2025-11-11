# Lesson 10: Working with Files and Paths in Rust

**Estimated Time**: 2-3 hours
**Prerequisites**: Lessons 08-09

## 🎯 Learning Objectives

By the end of this lesson, you will:

1. Master Rust's file I/O operations
2. Work with paths using `Path` and `PathBuf`
3. Handle filesystem operations safely
4. Read and write binary data
5. Traverse directories
6. Apply these skills to Git implementation

## 📁 Path vs PathBuf

Rust has two types for paths:

- **`Path`**: Borrowed path (like `&str` for strings)
- **`PathBuf`**: Owned path (like `String` for strings)

```rust
use std::path::{Path, PathBuf};

fn main() {
    // Path - borrowed, usually from string literal
    let path: &Path = Path::new("/home/user/.git");

    // PathBuf - owned, can be modified
    let mut path_buf: PathBuf = PathBuf::from("/home/user");
    path_buf.push(".git");
    path_buf.push("objects");

    println!("{}", path_buf.display());
    // /home/user/.git/objects
}
```

### Converting Between Types

```rust
use std::path::{Path, PathBuf};

fn main() {
    // &str → Path
    let path = Path::new("/tmp/file.txt");

    // &str → PathBuf
    let path_buf = PathBuf::from("/tmp/file.txt");

    // PathBuf → &Path
    let path_ref: &Path = &path_buf;

    // Path → PathBuf
    let owned: PathBuf = path.to_path_buf();

    // PathBuf → String (lossy conversion for non-UTF-8)
    let string = path_buf.to_string_lossy();
}
```

## 🛠️ Path Operations

### Building Paths

```rust
use std::path::PathBuf;

fn main() {
    let mut path = PathBuf::from(".git");

    // Add components
    path.push("objects");
    path.push("aa");
    path.push("bbcc...");

    println!("{}", path.display());
    // .git/objects/aa/bbcc...

    // Join (creates new PathBuf)
    let path2 = Path::new(".git").join("refs").join("heads").join("main");
}
```

### Deconstructing Paths

```rust
use std::path::Path;

fn main() {
    let path = Path::new("/home/user/.git/objects/aa/bbccdd");

    // Get parent
    if let Some(parent) = path.parent() {
        println!("Parent: {}", parent.display());
        // /home/user/.git/objects/aa
    }

    // Get filename
    if let Some(name) = path.file_name() {
        println!("Filename: {:?}", name);
        // "bbccdd"
    }

    // Get extension
    let file_path = Path::new("config.json");
    if let Some(ext) = file_path.extension() {
        println!("Extension: {:?}", ext);
        // "json"
    }

    // Components
    for component in path.components() {
        println!("{:?}", component);
    }
}
```

### Path Queries

```rust
use std::path::Path;

fn main() {
    let path = Path::new(".git/objects");

    // Check existence
    if path.exists() {
        println!("Path exists");
    }

    // Check type
    if path.is_dir() {
        println!("Is directory");
    }

    if path.is_file() {
        println!("Is file");
    }

    // Absolute vs relative
    if path.is_absolute() {
        println!("Absolute path");
    } else {
        println!("Relative path");
    }
}
```

### Git Example: Object Path

```rust
use std::path::{Path, PathBuf};

fn get_object_path(git_dir: &Path, hash: &str) -> PathBuf {
    let (dir, file) = hash.split_at(2);
    git_dir.join("objects").join(dir).join(file)
}

fn main() {
    let git_dir = Path::new(".git");
    let hash = "557db03de997c86a4a028e1ebd3a1ceb225be238";

    let path = get_object_path(git_dir, hash);
    println!("{}", path.display());
    // .git/objects/55/7db03de997c86a4a028e1ebd3a1ceb225be238
}
```

## 📖 Reading Files

### Read Entire File

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Read as string
    let content: String = fs::read_to_string("config.txt")?;
    println!("{}", content);

    // Read as bytes
    let bytes: Vec<u8> = fs::read("data.bin")?;
    println!("Read {} bytes", bytes.len());

    Ok(())
}
```

### Read with Better Errors

```rust
use anyhow::{Context, Result};
use std::fs;

fn read_config(path: &str) -> Result<String> {
    fs::read_to_string(path)
        .with_context(|| format!("Failed to read config from {}", path))
}

fn main() -> Result<()> {
    let config = read_config("config.txt")?;
    println!("{}", config);
    Ok(())
}
```

### Buffered Reading (for large files)

```rust
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() -> std::io::Result<()> {
    let file = File::open("large.txt")?;
    let reader = BufReader::new(file);

    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
    }

    Ok(())
}
```

### Git Example: Read Object

```rust
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

fn read_object(git_dir: &Path, hash: &str) -> Result<Vec<u8>> {
    let (dir, file) = hash.split_at(2);
    let path = git_dir.join("objects").join(dir).join(file);

    fs::read(&path)
        .with_context(|| format!("Failed to read object {}", hash))
}

fn main() -> Result<()> {
    let git_dir = Path::new(".git");
    let data = read_object(git_dir, "557db03de997c86a4a028e1ebd3a1ceb225be238")?;
    println!("Read {} bytes", data.len());
    Ok(())
}
```

## ✍️ Writing Files

### Write Entire File

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Write string
    fs::write("output.txt", "Hello, World!")?;

    // Write bytes
    let data: Vec<u8> = vec![0, 1, 2, 3];
    fs::write("output.bin", &data)?;

    Ok(())
}
```

### Buffered Writing

```rust
use std::fs::File;
use std::io::{BufWriter, Write};

fn main() -> std::io::Result<()> {
    let file = File::create("output.txt")?;
    let mut writer = BufWriter::new(file);

    writer.write_all(b"Line 1\n")?;
    writer.write_all(b"Line 2\n")?;

    // Flush ensures data is written
    writer.flush()?;

    Ok(())
}
```

### Git Example: Write Object

```rust
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

fn write_object(git_dir: &Path, hash: &str, data: &[u8]) -> Result<()> {
    let (dir, file) = hash.split_at(2);
    let dir_path = git_dir.join("objects").join(dir);

    // Create directory if it doesn't exist
    fs::create_dir_all(&dir_path)
        .context("Failed to create objects directory")?;

    // Write file
    let file_path = dir_path.join(file);
    fs::write(&file_path, data)
        .with_context(|| format!("Failed to write object {}", hash))?;

    Ok(())
}

fn main() -> Result<()> {
    let git_dir = Path::new(".git");
    let hash = "557db03de997c86a4a028e1ebd3a1ceb225be238";
    let data = b"compressed object data...";

    write_object(git_dir, hash, data)?;
    println!("Object written successfully");
    Ok(())
}
```

## 📂 Directory Operations

### Create Directories

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    // Create single directory
    fs::create_dir("new_dir")?;

    // Create directory with parents
    fs::create_dir_all("path/to/nested/dir")?;

    Ok(())
}
```

### Read Directory

```rust
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    for entry in fs::read_dir(".")? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            println!("File: {}", path.display());
        } else if path.is_dir() {
            println!("Dir:  {}", path.display());
        }
    }

    Ok(())
}
```

### Recursive Directory Walking

```rust
use std::fs;
use std::path::Path;

fn walk_dir(path: &Path) -> std::io::Result<()> {
    if path.is_dir() {
        for entry in fs::read_dir(path)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                walk_dir(&path)?;  // Recurse
            } else {
                println!("{}", path.display());
            }
        }
    }
    Ok(())
}

fn main() -> std::io::Result<()> {
    walk_dir(Path::new(".git/objects"))?;
    Ok(())
}
```

### Using walkdir Crate (Better!)

```rust
use walkdir::WalkDir;

fn main() {
    for entry in WalkDir::new(".git/objects") {
        let entry = entry.unwrap();
        if entry.file_type().is_file() {
            println!("{}", entry.path().display());
        }
    }
}
```

### Git Example: Find All Objects

```rust
use anyhow::Result;
use std::fs;
use std::path::Path;

fn find_all_objects(git_dir: &Path) -> Result<Vec<String>> {
    let objects_dir = git_dir.join("objects");
    let mut hashes = Vec::new();

    for entry in fs::read_dir(&objects_dir)? {
        let entry = entry?;
        let dir_name = entry.file_name();

        // Skip pack and info directories
        if dir_name == "pack" || dir_name == "info" {
            continue;
        }

        let dir_path = entry.path();
        if dir_path.is_dir() {
            for file_entry in fs::read_dir(&dir_path)? {
                let file_entry = file_entry?;
                let file_name = file_entry.file_name();

                // Construct full hash
                let hash = format!(
                    "{}{}",
                    dir_name.to_string_lossy(),
                    file_name.to_string_lossy()
                );
                hashes.push(hash);
            }
        }
    }

    Ok(hashes)
}

fn main() -> Result<()> {
    let git_dir = Path::new(".git");
    let objects = find_all_objects(git_dir)?;

    println!("Found {} objects:", objects.len());
    for hash in objects {
        println!("  {}", hash);
    }

    Ok(())
}
```

## 🔄 File Metadata

```rust
use std::fs;
use std::time::SystemTime;

fn main() -> std::io::Result<()> {
    let metadata = fs::metadata("file.txt")?;

    println!("Size: {} bytes", metadata.len());
    println!("Is file: {}", metadata.is_file());
    println!("Is dir: {}", metadata.is_dir());
    println!("Read-only: {}", metadata.permissions().readonly());

    if let Ok(modified) = metadata.modified() {
        println!("Modified: {:?}", modified);
    }

    Ok(())
}
```

### Git Example: Check Object Metadata

```rust
use anyhow::{Context, Result};
use std::fs;
use std::path::Path;

fn get_object_size(git_dir: &Path, hash: &str) -> Result<u64> {
    let (dir, file) = hash.split_at(2);
    let path = git_dir.join("objects").join(dir).join(file);

    let metadata = fs::metadata(&path)
        .with_context(|| format!("Failed to get metadata for {}", hash))?;

    Ok(metadata.len())
}

fn main() -> Result<()> {
    let git_dir = Path::new(".git");
    let size = get_object_size(git_dir, "557db03de997c86a4a028e1ebd3a1ceb225be238")?;
    println!("Object size: {} bytes (compressed)", size);
    Ok(())
}
```

## 🔐 Binary Data Operations

### Reading Binary Data

```rust
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

fn main() -> std::io::Result<()> {
    let mut file = File::open("data.bin")?;

    // Read specific number of bytes
    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;
    println!("First 4 bytes: {:?}", buffer);

    // Seek to position
    file.seek(SeekFrom::Start(10))?;

    // Read from new position
    let mut buffer = [0u8; 4];
    file.read_exact(&mut buffer)?;
    println!("Bytes 10-13: {:?}", buffer);

    Ok(())
}
```

### Writing Binary Data

```rust
use std::fs::File;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let mut file = File::create("output.bin")?;

    // Write bytes
    file.write_all(&[0x00, 0x01, 0x02, 0x03])?;

    // Write integers
    let number: u32 = 12345;
    file.write_all(&number.to_be_bytes())?;  // Big-endian
    file.write_all(&number.to_le_bytes())?;  // Little-endian

    Ok(())
}
```

### Git Example: Parse Object Header

```rust
use anyhow::{bail, Result};

fn parse_object_header(data: &[u8]) -> Result<(&str, usize, &[u8])> {
    // Find null byte separating header from content
    let null_pos = data.iter()
        .position(|&b| b == 0)
        .ok_or_else(|| anyhow::anyhow!("No null byte in object"))?;

    // Parse header: "type size"
    let header = std::str::from_utf8(&data[..null_pos])?;
    let parts: Vec<&str> = header.split(' ').collect();

    if parts.len() != 2 {
        bail!("Invalid object header: {}", header);
    }

    let object_type = parts[0];
    let size: usize = parts[1].parse()?;
    let content = &data[null_pos + 1..];

    Ok((object_type, size, content))
}

fn main() -> Result<()> {
    let data = b"blob 11\0Hello World";
    let (obj_type, size, content) = parse_object_header(data)?;

    println!("Type: {}", obj_type);
    println!("Size: {}", size);
    println!("Content: {}", std::str::from_utf8(content)?);

    Ok(())
}
```

## 🎯 Practical Git Examples

### Example 1: Complete Object Reader

```rust
use anyhow::{Context, Result};
use flate2::read::ZlibDecoder;
use std::fs;
use std::io::Read;
use std::path::Path;

fn read_and_parse_object(git_dir: &Path, hash: &str) -> Result<(String, Vec<u8>)> {
    // Read compressed data
    let (dir, file) = hash.split_at(2);
    let path = git_dir.join("objects").join(dir).join(file);
    let compressed = fs::read(&path)
        .with_context(|| format!("Failed to read object {}", hash))?;

    // Decompress
    let mut decoder = ZlibDecoder::new(&compressed[..]);
    let mut decompressed = Vec::new();
    decoder.read_to_end(&mut decompressed)?;

    // Parse header
    let null_pos = decompressed.iter()
        .position(|&b| b == 0)
        .context("Invalid object format")?;

    let header = std::str::from_utf8(&decompressed[..null_pos])?;
    let parts: Vec<&str> = header.split(' ').collect();

    let object_type = parts[0].to_string();
    let content = decompressed[null_pos + 1..].to_vec();

    Ok((object_type, content))
}

fn main() -> Result<()> {
    let git_dir = Path::new(".git");
    let hash = "557db03de997c86a4a028e1ebd3a1ceb225be238";

    let (obj_type, content) = read_and_parse_object(git_dir, hash)?;

    println!("Type: {}", obj_type);
    println!("Content: {}", String::from_utf8_lossy(&content));

    Ok(())
}
```

### Example 2: Complete Object Writer

```rust
use anyhow::Result;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::fs;
use std::io::Write;
use std::path::Path;

fn write_blob(git_dir: &Path, content: &[u8]) -> Result<String> {
    // Create header
    let header = format!("blob {}\0", content.len());

    // Combine header and content
    let mut data = header.into_bytes();
    data.extend_from_slice(content);

    // Compute hash
    let mut hasher = Sha1::new();
    hasher.update(&data);
    let hash = format!("{:x}", hasher.finalize());

    // Compress
    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
    encoder.write_all(&data)?;
    let compressed = encoder.finish()?;

    // Write to file
    let (dir, file) = hash.split_at(2);
    let dir_path = git_dir.join("objects").join(dir);
    fs::create_dir_all(&dir_path)?;

    let file_path = dir_path.join(file);
    fs::write(&file_path, compressed)?;

    Ok(hash)
}

fn main() -> Result<()> {
    let git_dir = Path::new(".git");
    let content = b"Hello World";

    let hash = write_blob(git_dir, content)?;
    println!("Created blob: {}", hash);

    Ok(())
}
```

## 🎓 Key Takeaways

1. **Path vs PathBuf**: Like `&str` vs `String`
2. **join()**: Build paths safely across platforms
3. **fs::read/write**: Simple operations for entire files
4. **BufReader/BufWriter**: Efficient for large files
5. **create_dir_all()**: Create nested directories
6. **walkdir**: Best for recursive directory traversal
7. **Always use error context**: `.with_context()` for better errors

## 🧩 Common Patterns

### Reading with fallback

```rust
fn read_config(path: &Path) -> String {
    fs::read_to_string(path).unwrap_or_else(|_| String::from("default"))
}
```

### Atomic write (write to temp, then rename)

```rust
fn atomic_write(path: &Path, content: &[u8]) -> Result<()> {
    let temp = path.with_extension("tmp");
    fs::write(&temp, content)?;
    fs::rename(&temp, path)?;
    Ok(())
}
```

### Check and create

```rust
fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}
```

## ✅ Checkpoint Quiz

1. What's the difference between `Path` and `PathBuf`?
2. How do you build a path from multiple components?
3. What's the safest way to create nested directories?
4. How do you read a file as bytes vs as a string?
5. When should you use `BufReader`?

<details>
<summary>Click to see answers</summary>

1. `Path` is borrowed (like `&str`), `PathBuf` is owned (like `String`)
2. Use `.join()`: `Path::new("a").join("b").join("c")`
3. `fs::create_dir_all(path)` - creates all missing parents
4. Bytes: `fs::read()`, String: `fs::read_to_string()`
5. When reading large files or reading line-by-line
</details>

## 🧪 Hands-On Exercises

### Exercise 1: Object Path Builder

Write a function that takes a git directory and hash, and returns the full object path.

### Exercise 2: Safe File Reader

Write a function that reads a file and returns a helpful error message if it fails.

### Exercise 3: Directory Scanner

Write a function that lists all files in `.git/objects/` and counts them.

## 🔜 Next Lesson

Now you can work with files and paths! Next, we'll learn about parsing and serialization for Git objects.

**→ [Lesson 11: Parsing and Serialization in Rust](11-rust-parsing.md)**

---

**Phase**: 3 - Rust Fundamentals
**Lesson**: 10 of 20
