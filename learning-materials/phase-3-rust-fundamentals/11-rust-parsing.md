# Lesson 11: Parsing and Serialization in Rust

**Estimated Time**: 2-3 hours
**Prerequisites**: Lessons 08-10

## 🎯 Learning Objectives

By the end of this lesson, you will:

1. Parse binary data structures
2. Serialize Rust types to bytes
3. Work with byte slices efficiently
4. Parse Git object formats
5. Build robust parsers with error handling
6. Understand serialization patterns

## 🔢 Working with Bytes

### Byte Arrays and Slices

```rust
fn main() {
    // Fixed-size array
    let array: [u8; 4] = [1, 2, 3, 4];

    // Dynamic vector
    let vec: Vec<u8> = vec![1, 2, 3, 4];

    // Slice (borrowed view)
    let slice: &[u8] = &vec[1..3];  // [2, 3]

    println!("{:?}", slice);
}
```

### Byte Literals

```rust
fn main() {
    // Byte literal (u8)
    let byte: u8 = b'A';  // 65

    // Byte string literal (&[u8])
    let bytes: &[u8] = b"Hello";  // [72, 101, 108, 108, 111]

    // Convert string to bytes
    let string = "Hello";
    let bytes: &[u8] = string.as_bytes();
}
```

### Hexadecimal Notation

```rust
fn main() {
    // Hex literals
    let byte = 0xFF;  // 255
    let bytes = vec![0x00, 0x01, 0x02, 0xFF];

    // Format as hex
    println!("{:02x}", byte);  // ff
    println!("{:02X}", byte);  // FF

    // Print all bytes as hex
    for b in &bytes {
        print!("{:02x} ", b);
    }
    // 00 01 02 ff
}
```

## 🔍 Finding Data in Bytes

### Finding a Byte

```rust
fn main() {
    let data = b"blob 11\0Hello World";

    // Find null byte
    if let Some(pos) = data.iter().position(|&b| b == 0) {
        println!("Null byte at position {}", pos);  // 7
    }

    // Split at null byte
    let (header, content) = data.split_at(8);
    println!("Header: {:?}", header);
    println!("Content: {:?}", content);
}
```

### Finding a Pattern

```rust
fn main() {
    let data = b"tree 100\0...content...";

    // Check if starts with pattern
    if data.starts_with(b"tree ") {
        println!("This is a tree object");
    }

    // Find substring
    let haystack = b"Hello World";
    let needle = b"World";

    if let Some(pos) = haystack.windows(needle.len())
        .position(|window| window == needle)
    {
        println!("Found at position {}", pos);  // 6
    }
}
```

## 📝 Parsing Text from Bytes

### Converting Bytes to String

```rust
use std::str;

fn main() {
    let bytes = b"Hello World";

    // Safe conversion (returns Result)
    match str::from_utf8(bytes) {
        Ok(s) => println!("{}", s),
        Err(e) => println!("Invalid UTF-8: {}", e),
    }

    // Lossy conversion (replaces invalid bytes)
    let string = String::from_utf8_lossy(bytes);
    println!("{}", string);

    // Panic if invalid (unsafe!)
    let string = str::from_utf8(bytes).unwrap();
    println!("{}", string);
}
```

### Parsing Numbers

```rust
fn main() {
    let text = b"12345";

    // Convert to string, then parse
    let string = std::str::from_utf8(text).unwrap();
    let number: u32 = string.parse().unwrap();
    println!("{}", number);  // 12345

    // Or use from_utf8_lossy
    let number: u32 = String::from_utf8_lossy(text).parse().unwrap();
    println!("{}", number);
}
```

## 🎯 Parsing Git Objects

### Example 1: Parse Object Header

```rust
use anyhow::{Context, Result, bail};

fn parse_header(data: &[u8]) -> Result<(&str, usize, &[u8])> {
    // Find null byte
    let null_pos = data.iter()
        .position(|&b| b == 0)
        .context("No null byte found in object")?;

    // Get header (before null)
    let header_bytes = &data[..null_pos];
    let header = std::str::from_utf8(header_bytes)?;

    // Split "type size"
    let parts: Vec<&str> = header.split(' ').collect();
    if parts.len() != 2 {
        bail!("Invalid header format: {}", header);
    }

    let object_type = parts[0];
    let size: usize = parts[1].parse()
        .context("Invalid size in header")?;

    // Get content (after null)
    let content = &data[null_pos + 1..];

    Ok((object_type, size, content))
}

fn main() -> Result<()> {
    let data = b"blob 11\0Hello World";
    let (obj_type, size, content) = parse_header(data)?;

    println!("Type: {}", obj_type);
    println!("Size: {}", size);
    println!("Content: {}", std::str::from_utf8(content)?);

    Ok(())
}
```

### Example 2: Parse Commit Object

```rust
use anyhow::{Context, Result};

#[derive(Debug)]
struct Commit {
    tree: String,
    parents: Vec<String>,
    author: String,
    committer: String,
    message: String,
}

fn parse_commit(data: &[u8]) -> Result<Commit> {
    let content = std::str::from_utf8(data)?;
    let mut lines = content.lines();

    let mut commit = Commit {
        tree: String::new(),
        parents: Vec::new(),
        author: String::new(),
        committer: String::new(),
        message: String::new(),
    };

    // Parse header lines
    for line in lines.by_ref() {
        if line.is_empty() {
            break;  // Empty line separates headers from message
        }

        if let Some(tree) = line.strip_prefix("tree ") {
            commit.tree = tree.to_string();
        } else if let Some(parent) = line.strip_prefix("parent ") {
            commit.parents.push(parent.to_string());
        } else if let Some(author) = line.strip_prefix("author ") {
            commit.author = author.to_string();
        } else if let Some(committer) = line.strip_prefix("committer ") {
            commit.committer = committer.to_string();
        }
    }

    // Rest is commit message
    commit.message = lines.collect::<Vec<_>>().join("\n");

    Ok(commit)
}

fn main() -> Result<()> {
    let data = b"tree abc123\nparent def456\nauthor John Doe <john@example.com> 1234567890 +0000\ncommitter John Doe <john@example.com> 1234567890 +0000\n\nInitial commit\n";

    let commit = parse_commit(data)?;
    println!("{:#?}", commit);

    Ok(())
}
```

### Example 3: Parse Tree Entry

```rust
use anyhow::{Context, Result};

#[derive(Debug)]
struct TreeEntry {
    mode: String,
    name: String,
    hash: [u8; 20],  // SHA-1 is 20 bytes
}

fn parse_tree_entry(data: &[u8]) -> Result<(TreeEntry, usize)> {
    // Format: [mode] [name]\0[20-byte hash]

    // Find space
    let space_pos = data.iter()
        .position(|&b| b == b' ')
        .context("No space in tree entry")?;

    let mode = std::str::from_utf8(&data[..space_pos])?.to_string();

    // Find null
    let null_pos = data[space_pos + 1..]
        .iter()
        .position(|&b| b == 0)
        .context("No null byte in tree entry")?;

    let name_start = space_pos + 1;
    let name = std::str::from_utf8(&data[name_start..name_start + null_pos])?
        .to_string();

    // Read 20-byte hash
    let hash_start = name_start + null_pos + 1;
    if data.len() < hash_start + 20 {
        anyhow::bail!("Not enough bytes for hash");
    }

    let mut hash = [0u8; 20];
    hash.copy_from_slice(&data[hash_start..hash_start + 20]);

    let entry = TreeEntry { mode, name, hash };
    let consumed = hash_start + 20;

    Ok((entry, consumed))
}

fn main() -> Result<()> {
    // Example tree entry
    let mut data = Vec::new();
    data.extend_from_slice(b"100644 file.txt\0");
    data.extend_from_slice(&[0x55, 0x7d, 0xb0, 0x3d, 0xe9, 0x97, 0xc8, 0x6a, 0x4a, 0x02, 0x8e, 0x1e, 0xbd, 0x3a, 0x1c, 0xeb, 0x22, 0x5b, 0xe2, 0x38]);

    let (entry, consumed) = parse_tree_entry(&data)?;
    println!("{:#?}", entry);
    println!("Consumed {} bytes", consumed);

    Ok(())
}
```

## 🔨 Building Objects (Serialization)

### Example 1: Serialize Blob

```rust
fn serialize_blob(content: &[u8]) -> Vec<u8> {
    let header = format!("blob {}\0", content.len());

    let mut data = header.into_bytes();
    data.extend_from_slice(content);

    data
}

fn main() {
    let content = b"Hello World";
    let blob = serialize_blob(content);

    println!("{:?}", blob);
    // [98, 108, 111, 98, 32, 49, 49, 0, 72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]
    // b   l   o   b       1   1   \0  H   e   l   l   o       W   o   r   l   d
}
```

### Example 2: Serialize Commit

```rust
struct Commit {
    tree: String,
    parents: Vec<String>,
    author: String,
    committer: String,
    message: String,
}

impl Commit {
    fn serialize(&self) -> Vec<u8> {
        let mut lines = Vec::new();

        // Tree
        lines.push(format!("tree {}", self.tree));

        // Parents
        for parent in &self.parents {
            lines.push(format!("parent {}", parent));
        }

        // Author and committer
        lines.push(format!("author {}", self.author));
        lines.push(format!("committer {}", self.committer));

        // Empty line, then message
        lines.push(String::new());
        lines.push(self.message.clone());

        let content = lines.join("\n");

        // Add header
        let header = format!("commit {}\0", content.len());
        let mut data = header.into_bytes();
        data.extend_from_slice(content.as_bytes());

        data
    }
}

fn main() {
    let commit = Commit {
        tree: "abc123".to_string(),
        parents: vec!["def456".to_string()],
        author: "John Doe <john@example.com> 1234567890 +0000".to_string(),
        committer: "John Doe <john@example.com> 1234567890 +0000".to_string(),
        message: "Initial commit".to_string(),
    };

    let data = commit.serialize();
    println!("{}", String::from_utf8_lossy(&data));
}
```

### Example 3: Serialize Tree

```rust
struct TreeEntry {
    mode: String,
    name: String,
    hash: [u8; 20],
}

fn serialize_tree(entries: &[TreeEntry]) -> Vec<u8> {
    let mut content = Vec::new();

    for entry in entries {
        // Format: [mode] [name]\0[20-byte hash]
        content.extend_from_slice(entry.mode.as_bytes());
        content.push(b' ');
        content.extend_from_slice(entry.name.as_bytes());
        content.push(0);
        content.extend_from_slice(&entry.hash);
    }

    // Add header
    let header = format!("tree {}\0", content.len());
    let mut data = header.into_bytes();
    data.extend_from_slice(&content);

    data
}

fn main() {
    let entries = vec![
        TreeEntry {
            mode: "100644".to_string(),
            name: "file.txt".to_string(),
            hash: [0x55, 0x7d, 0xb0, 0x3d, 0xe9, 0x97, 0xc8, 0x6a, 0x4a, 0x02, 0x8e, 0x1e, 0xbd, 0x3a, 0x1c, 0xeb, 0x22, 0x5b, 0xe2, 0x38],
        },
    ];

    let data = serialize_tree(&entries);
    println!("Serialized {} bytes", data.len());
}
```

## 🔧 Byte Manipulation Utilities

### Converting Integers to Bytes

```rust
fn main() {
    let number: u32 = 12345;

    // Big-endian (network byte order)
    let be_bytes = number.to_be_bytes();
    println!("{:?}", be_bytes);  // [0, 0, 48, 57]

    // Little-endian
    let le_bytes = number.to_le_bytes();
    println!("{:?}", le_bytes);  // [57, 48, 0, 0]

    // Convert back
    let restored = u32::from_be_bytes(be_bytes);
    println!("{}", restored);  // 12345
}
```

### Hex Encoding/Decoding

```rust
fn bytes_to_hex(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect()
}

fn hex_to_bytes(hex: &str) -> Result<Vec<u8>, std::num::ParseIntError> {
    (0..hex.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&hex[i..i + 2], 16))
        .collect()
}

fn main() {
    let bytes = vec![0x55, 0x7d, 0xb0, 0x3d];

    let hex = bytes_to_hex(&bytes);
    println!("Hex: {}", hex);  // 557db03d

    let decoded = hex_to_bytes(&hex).unwrap();
    println!("Decoded: {:?}", decoded);  // [85, 125, 176, 61]
}
```

### Using the hex Crate

```toml
# Cargo.toml
[dependencies]
hex = "0.4"
```

```rust
use hex;

fn main() {
    let bytes = vec![0x55, 0x7d, 0xb0, 0x3d];

    // Encode
    let hex_string = hex::encode(&bytes);
    println!("{}", hex_string);  // 557db03d

    // Decode
    let decoded = hex::decode(hex_string).unwrap();
    println!("{:?}", decoded);  // [85, 125, 176, 61]
}
```

## 🎯 Complete Example: Git Object Parser

```rust
use anyhow::{Context, Result, bail};
use std::str;

#[derive(Debug)]
enum GitObject {
    Blob { content: Vec<u8> },
    Tree { entries: Vec<TreeEntry> },
    Commit { commit: CommitData },
}

#[derive(Debug)]
struct TreeEntry {
    mode: String,
    name: String,
    hash: [u8; 20],
}

#[derive(Debug)]
struct CommitData {
    tree: String,
    parents: Vec<String>,
    author: String,
    committer: String,
    message: String,
}

fn parse_object(data: &[u8]) -> Result<GitObject> {
    // Parse header
    let null_pos = data.iter()
        .position(|&b| b == 0)
        .context("No null byte in object")?;

    let header = str::from_utf8(&data[..null_pos])?;
    let parts: Vec<&str> = header.split(' ').collect();

    if parts.len() != 2 {
        bail!("Invalid header: {}", header);
    }

    let object_type = parts[0];
    let content = &data[null_pos + 1..];

    match object_type {
        "blob" => Ok(GitObject::Blob {
            content: content.to_vec(),
        }),
        "tree" => {
            let entries = parse_tree_entries(content)?;
            Ok(GitObject::Tree { entries })
        }
        "commit" => {
            let commit = parse_commit_data(content)?;
            Ok(GitObject::Commit { commit })
        }
        _ => bail!("Unknown object type: {}", object_type),
    }
}

fn parse_tree_entries(data: &[u8]) -> Result<Vec<TreeEntry>> {
    let mut entries = Vec::new();
    let mut offset = 0;

    while offset < data.len() {
        let remaining = &data[offset..];

        // Find space
        let space_pos = remaining.iter()
            .position(|&b| b == b' ')
            .context("No space in tree entry")?;

        let mode = str::from_utf8(&remaining[..space_pos])?.to_string();

        // Find null
        let null_pos = remaining[space_pos + 1..]
            .iter()
            .position(|&b| b == 0)
            .context("No null in tree entry")?;

        let name_start = space_pos + 1;
        let name = str::from_utf8(&remaining[name_start..name_start + null_pos])?
            .to_string();

        // Read hash
        let hash_start = name_start + null_pos + 1;
        if remaining.len() < hash_start + 20 {
            bail!("Not enough bytes for hash");
        }

        let mut hash = [0u8; 20];
        hash.copy_from_slice(&remaining[hash_start..hash_start + 20]);

        entries.push(TreeEntry { mode, name, hash });
        offset += hash_start + 20;
    }

    Ok(entries)
}

fn parse_commit_data(data: &[u8]) -> Result<CommitData> {
    let content = str::from_utf8(data)?;
    let mut lines = content.lines();

    let mut tree = String::new();
    let mut parents = Vec::new();
    let mut author = String::new();
    let mut committer = String::new();

    for line in lines.by_ref() {
        if line.is_empty() {
            break;
        }

        if let Some(t) = line.strip_prefix("tree ") {
            tree = t.to_string();
        } else if let Some(p) = line.strip_prefix("parent ") {
            parents.push(p.to_string());
        } else if let Some(a) = line.strip_prefix("author ") {
            author = a.to_string();
        } else if let Some(c) = line.strip_prefix("committer ") {
            committer = c.to_string();
        }
    }

    let message = lines.collect::<Vec<_>>().join("\n");

    Ok(CommitData {
        tree,
        parents,
        author,
        committer,
        message,
    })
}

fn main() -> Result<()> {
    // Test blob
    let blob_data = b"blob 11\0Hello World";
    let blob = parse_object(blob_data)?;
    println!("{:#?}", blob);

    Ok(())
}
```

## 🎓 Key Takeaways

1. **Bytes are fundamental**: Git objects are binary data
2. **Parse carefully**: Always check bounds and handle errors
3. **Use str::from_utf8**: Safe conversion from bytes to string
4. **Null bytes matter**: Git uses `\0` as separator
5. **Binary data**: SHA-1 hashes are 20 raw bytes, not hex strings
6. **Serialization mirrors parsing**: Build objects the same way you parse them

## 🧩 Common Patterns

### Safe byte access

```rust
// ❌ Can panic
let byte = data[10];

// ✅ Safe
if let Some(&byte) = data.get(10) {
    // Use byte
}
```

### Splitting at delimiter

```rust
let data = b"header\0content";

if let Some(pos) = data.iter().position(|&b| b == 0) {
    let (header, rest) = data.split_at(pos);
    let content = &rest[1..];  // Skip the null byte
}
```

### Building with Vec

```rust
let mut data = Vec::new();
data.extend_from_slice(b"blob ");
data.extend_from_slice(b"11");
data.push(0);  // Null byte
data.extend_from_slice(b"Hello World");
```

## ✅ Checkpoint Quiz

1. How do you find a null byte in a byte slice?
2. What's the difference between `[u8; 20]` and `Vec<u8>`?
3. How do you safely convert `&[u8]` to `&str`?
4. Why does Git store SHA-1 hashes as binary in tree objects?
5. What's the format of a Git object: `[type] [size]\0[content]` - which part is the header?

<details>
<summary>Click to see answers</summary>

1. `data.iter().position(|&b| b == 0)`
2. `[u8; 20]` is fixed-size array, `Vec<u8>` is dynamic vector
3. `std::str::from_utf8(bytes)` returns `Result<&str, Utf8Error>`
4. Binary is more compact (20 bytes vs 40 characters) and faster to parse
5. Header is `[type] [size]\0`, content comes after
</details>

## 🧪 Hands-On Exercises

### Exercise 1: Header Parser

Write a robust header parser that validates the object type.

### Exercise 2: Blob Serializer

Write a function that takes content and returns a complete blob object (with header).

### Exercise 3: Tree Entry Parser

Parse a single tree entry and extract mode, name, and hash.

## 🔜 Next Lesson

You now understand all the Rust fundamentals! Next, we'll set up our Git implementation project.

**→ [Lesson 12: Project Setup and Architecture](../phase-4-building-git/12-project-setup-and-architecture.md)**

---

**Phase**: 3 - Rust Fundamentals
**Lesson**: 11 of 20
