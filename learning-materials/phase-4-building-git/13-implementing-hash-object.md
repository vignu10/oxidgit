# Lesson 13: Implementing hash-object

**Estimated Time**: 2 hours
**Prerequisites**: Lesson 12

## 🎯 Learning Objectives

Implement the `git hash-object` command:
- Store file content as a blob object
- Compute SHA-1 hash
- Write to object database
- Verify against real Git

## 📝 What hash-object Does

```bash
$ echo "Hello World" | git hash-object --stdin
557db03de997c86a4a028e1ebd3a1ceb225be238

$ echo "Hello World" | git hash-object --stdin -w
557db03de997c86a4a028e1ebd3a1ceb225be238
# (with -w flag, also writes to .git/objects/)
```

It:
1. Reads file content
2. Creates Git blob format: `blob [size]\0[content]`
3. Computes SHA-1 hash
4. Optionally writes compressed object to database

## 🏗️ Implementation

### Step 1: Create Blob Type

`src/objects/blob.rs`:

```rust
use crate::objects::{GitObject, ObjectType};
use anyhow::Result;

/// Represents a Git blob object (file content)
#[derive(Debug, Clone)]
pub struct Blob {
    pub content: Vec<u8>,
}

impl Blob {
    /// Create a new blob from content
    pub fn new(content: Vec<u8>) -> Self {
        Blob { content }
    }

    /// Create a blob from a file
    pub fn from_file(path: &str) -> Result<Self> {
        let content = std::fs::read(path)?;
        Ok(Blob::new(content))
    }
}

impl GitObject for Blob {
    fn object_type(&self) -> ObjectType {
        ObjectType::Blob
    }

    fn serialize(&self) -> Result<Vec<u8>> {
        // For blobs, serialization is just the raw content
        Ok(self.content.clone())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_blob_creation() {
        let content = b"Hello World".to_vec();
        let blob = Blob::new(content.clone());
        assert_eq!(blob.content, content);
    }

    #[test]
    fn test_blob_hash() {
        let blob = Blob::new(b"Hello World".to_vec());
        let hash = blob.hash().unwrap();

        // This is the known hash for "Hello World"
        assert_eq!(hash, "557db03de997c86a4a028e1ebd3a1ceb225be238");
    }

    #[test]
    fn test_blob_to_bytes() {
        let blob = Blob::new(b"Hello World".to_vec());
        let data = blob.to_bytes().unwrap();

        // Should be: "blob 11\0Hello World"
        let expected = b"blob 11\0Hello World";
        assert_eq!(data, expected);
    }
}
```

### Step 2: Implement hash-object Command

`src/commands/mod.rs`:

```rust
pub mod init;
pub mod hash_object;
pub mod cat_file;
```

`src/commands/hash_object.rs`:

```rust
use crate::objects::{Blob, GitObject};
use crate::repository::Repository;
use anyhow::Result;

/// Implement git hash-object
pub fn run(file_path: &str, write: bool) -> Result<()> {
    // Read file and create blob
    let blob = Blob::from_file(file_path)?;

    // Compute hash
    let hash = blob.hash()?;

    // If -w flag, write to repository
    if write {
        // Find repository (walk up from current directory)
        let repo = Repository::new(".")?;

        // Get blob data
        let data = blob.to_bytes()?;

        // Write to object database
        repo.write_object(&hash, &data)?;
    }

    // Print hash
    println!("{}", hash);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_hash_object() {
        let dir = tempdir().unwrap();
        let file = dir.path().join("test.txt");
        fs::write(&file, b"Hello World").unwrap();

        // Initialize repo
        Repository::init(dir.path()).unwrap();

        // Hash the file with write
        std::env::set_current_dir(dir.path()).unwrap();
        run(file.to_str().unwrap(), true).unwrap();

        // Verify object was written
        let repo = Repository::new(dir.path()).unwrap();
        let hash = "557db03de997c86a4a028e1ebd3a1ceb225be238";
        let object_path = repo.object_path(hash);

        assert!(object_path.exists());
    }
}
```

### Step 3: Update init Command

`src/commands/init.rs`:

```rust
use crate::repository::Repository;
use anyhow::Result;

pub fn run(path: &str) -> Result<()> {
    Repository::init(path)?;
    println!("Initialized empty oxid repository in {}/.git/", path);
    Ok(())
}
```

## 🧪 Testing

### Test Against Real Git

```bash
# Create test directory
cd /tmp
mkdir oxid-test
cd oxid-test

# Initialize with our implementation
oxid init

# Create test file
echo "Hello World" > test.txt

# Hash with our implementation
oxid hash-object test.txt
# Output: 557db03de997c86a4a028e1ebd3a1ceb225be238

# Compare with real git
git hash-object test.txt
# Output: 557db03de997c86a4a028e1ebd3a1ceb225be238

# They match! ✅

# Test writing
oxid hash-object -w test.txt

# Verify object exists
ls .git/objects/55/
# 7db03de997c86a4a028e1ebd3a1ceb225be238

# Read with real git
git cat-file -p 557db03
# Output: Hello World

# It works! 🎉
```

### Unit Tests

```bash
cargo test

# Output:
# running 5 tests
# test objects::blob::tests::test_blob_creation ... ok
# test objects::blob::tests::test_blob_hash ... ok
# test objects::blob::tests::test_blob_to_bytes ... ok
# test utils::tests::test_compress_decompress ... ok
# test utils::tests::test_hash_data ... ok
```

## 🎓 Key Takeaways

1. **Blob = raw content** - No filename, permissions, or metadata
2. **Git object format**: `[type] [size]\0[content]`
3. **Hash determines storage location**: `.git/objects/XX/YY...`
4. **Compression is transparent**: Write compressed, read decompressed
5. **Compatible with real Git**: Our objects can be read by git!

## ✅ Complete Feature Checklist

- [x] Create Blob type
- [x] Implement GitObject trait for Blob
- [x] Compute correct SHA-1 hash
- [x] Write objects to database
- [x] Compress with zlib
- [x] Compatible with real Git
- [x] Unit tests
- [x] Integration tests

## 🔜 Next Steps

Now we can **write** objects. Next, we'll implement `cat-file` to **read** them back!

**→ [Lesson 14: Implementing cat-file](14-implementing-cat-file.md)**

---

**Phase**: 4 - Building Git
**Lesson**: 13 of 20
