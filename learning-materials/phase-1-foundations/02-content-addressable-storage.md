# Lesson 02: Content-Addressable Storage

**Estimated Time**: 1.5-2 hours
**Prerequisites**: Lesson 01 - What Is Git Really?

## 🎯 Learning Objectives

By the end of this lesson, you will:

1. Understand what cryptographic hashing is and why Git uses it
2. Know how SHA-1 hashes work (without needing to implement them)
3. Understand why content-addressable storage is powerful
4. Be able to manually compute Git object hashes
5. Understand Git's object storage format

## 🔐 What is a Hash Function?

A hash function takes input of any size and produces a fixed-size output (the "hash" or "digest").

Think of it like a fingerprint for data:

```
Input (any size)  →  [Hash Function]  →  Hash (fixed size)

"Hello"           →  [SHA-1]  →  f7ff9e8b7bb2e09b70935a5d785e0cc5d9d0abf0
"Hello World"     →  [SHA-1]  →  0a4d55a8d778e5022fab701977c5d840bbc486d0
(Entire book)     →  [SHA-1]  →  3c5e8f... (still 40 characters)
```

## 🎲 Properties of Cryptographic Hashes

Git uses **SHA-1** (Secure Hash Algorithm 1), which has special properties:

### 1. Deterministic

Same input **always** produces same output:

```bash
echo "Hello World" | git hash-object --stdin
# Always: 557db03de997c86a4a028e1ebd3a1ceb225be238

echo "Hello World" | git hash-object --stdin
# Still: 557db03de997c86a4a028e1ebd3a1ceb225be238
```

This is crucial! If everyone's Git didn't produce the same hashes, repositories couldn't sync.

### 2. Avalanche Effect

Tiny input change → completely different hash:

```bash
echo "Hello World" | git hash-object --stdin
# 557db03de997c86a4a028e1ebd3a1ceb225be238

echo "Hello World!" | git hash-object --stdin  # Added "!"
# 980a0d5f19a64b4b30a87d4206aade58726b60e3
```

Changed one character, hash is completely different!

### 3. One-Way Function

You cannot reverse a hash to get the original content:

```
Hash → Original Content  ❌ Impossible
Original Content → Hash  ✅ Easy
```

This provides integrity checking: if someone gives you a hash, you can verify content matches, but can't create fake content with that hash.

### 4. Collision Resistant

It's (practically) impossible to find two different inputs with the same hash:

```
"Hello World" → 557db03...
"???"         → 557db03...  ← Can't find this "???"
```

### 5. Fixed Output Size

No matter the input size, SHA-1 always produces 160 bits (40 hexadecimal characters):

```
"Hi"                        → 40 characters
(Linux kernel source code)  → 40 characters
(4K video file)             → 40 characters
```

## 🔢 SHA-1 Format

SHA-1 produces 160 bits, displayed as 40 hexadecimal characters:

```
557db03de997c86a4a028e1ebd3a1ceb225be238
│                                          │
└──────────── 40 hex digits ───────────────┘

160 bits = 20 bytes = 40 hex characters
(Each hex digit represents 4 bits)
```

Git often shortens hashes to the first 7 characters when displaying:

```bash
git log --oneline
# a1b2c3d Fix bug in parser
# e4f5g6h Add new feature
```

But internally, Git uses the full 40-character hash.

## 🏗️ How Git Stores Objects

Git doesn't just hash your file content directly. It adds metadata first!

### The Git Object Format

```
[type] [size]\0[content]
│      │     │  │
│      │     │  └─ The actual data
│      │     └──── Null byte separator
│      └──────── Size in bytes
└──────────── Object type (blob, tree, commit, tag)
```

### Example: Storing "Hello World"

Let's see what Git actually hashes:

```
Content: "Hello World"
         ↓
Git prepends header: "blob 11\0Hello World"
         ↓
         "blob 11\0Hello World"
          │    │  │  │
          │    │  │  └─ Content
          │    │  └──── Null byte (0x00)
          │    └──────── Size (11 bytes)
          └──────────── Type (blob)
         ↓
Apply SHA-1 hash
         ↓
Result: 557db03de997c86a4a028e1ebd3a1ceb225be238
```

Let's verify this manually:

```bash
# Create the exact format Git uses
echo -en "blob 11\0Hello World" | sha1sum
# Output: 557db03de997c86a4a028e1ebd3a1ceb225be238

# Compare with Git's hash
echo "Hello World" | git hash-object --stdin
# Output: 557db03de997c86a4a028e1ebd3a1ceb225be238

# They match! 🎉
```

## 📁 Storage Location

Once Git has the hash, it stores the object at:

```
.git/objects/[first 2 chars]/[remaining 38 chars]
```

For hash `557db03de997c86a4a028e1ebd3a1ceb225be238`:

```
.git/objects/55/7db03de997c86a4a028e1ebd3a1ceb225be238
             ││ │
             ││ └─ Filename (38 chars)
             │└─── Directory (2 chars)
             └──── Always in objects/
```

Why split into directories?

- **Filesystem performance**: Many filesystems slow down with thousands of files in one directory
- **Git uses first 2 hex chars** → 256 possible directories (00-ff)
- Spreads objects across directories evenly

## 🗜️ Compression

Before writing to disk, Git compresses the object with **zlib**:

```
"blob 11\0Hello World"
         ↓
Apply zlib compression
         ↓
Compressed binary data
         ↓
Write to .git/objects/55/7db03...
```

This saves disk space. Git decompresses when reading.

### See it yourself:

```bash
# Create an object
echo "Hello World" > test.txt
git add test.txt

# Find the object file
find .git/objects -type f

# Try to read it directly (it's compressed, so gibberish)
cat .git/objects/55/7db03de997c86a4a028e1ebd3a1ceb225be238
# Output: x��(K��/�I ��

# Use Git to decompress and read it
git cat-file -p 557db03
# Output: Hello World
```

## 🧪 Hands-On: Understanding Object Storage

### Exercise 1: Manually Create Git Objects

```bash
# Create a new test repo
cd /tmp
rm -rf git-test
mkdir git-test && cd git-test
git init

# Method 1: Let Git store an object
echo "Hello World" | git hash-object --stdin -w
# Output: 557db03de997c86a4a028e1ebd3a1ceb225be238
#         The -w flag means "write to database"

# Verify it's stored
ls .git/objects/55/
# Output: 7db03de997c86a4a028e1ebd3a1ceb225be238

# Read it back
git cat-file -p 557db03
# Output: Hello World
```

### Exercise 2: Compute Hash Manually

```bash
# Hash "Hello World" the way Git does
echo -en "blob 11\0Hello World" | sha1sum
# Output: 557db03de997c86a4a028e1ebd3a1ceb225be238

# Try different content
echo -en "blob 7\0Goodbye" | sha1sum
# Output: (some other hash)

# What if we get the size wrong?
echo -en "blob 10\0Hello World" | sha1sum
# Output: (different hash - size matters!)
```

### Exercise 3: See the Deduplication

```bash
# Create multiple files with same content
echo "Hello World" > file1.txt
echo "Hello World" > file2.txt
echo "Hello World" > file3.txt

# Add them all
git add file1.txt file2.txt file3.txt

# Count objects
find .git/objects -type f | wc -l
# Output: 1 (or 4 if you also have tree objects)

# Git stored "Hello World" once, even though it appears in 3 files!
```

### Exercise 4: See the Avalanche Effect

```bash
# Original content
echo "version 1" | git hash-object --stdin
# Output: 83baae61804e65cc73a7201a7252750c76066a30

# Tiny change (one character)
echo "version 2" | git hash-object --stdin
# Output: 0b66d2baebe479b28e65ed34846c38fea81d9c4e

# Completely different hashes!
```

## 💡 Why Content-Addressable Storage is Powerful

### 1. Automatic Deduplication

```
file1.txt: "function foo() { return 42; }"
file2.txt: "function foo() { return 42; }"

Traditional FS: Stores content twice
Git: Stores content once, both files reference same blob
```

### 2. Integrity Checking

```
Someone gives you hash: 557db03...
You hash the content: 557db03...

Match? ✅ Content is authentic
Mismatch? ❌ Content was corrupted/tampered
```

### 3. Content-Based Merging

```
Branch A: Changed content → new hash
Branch B: Same file, different content → different hash
Main: Original content → original hash

Git can detect: "All three versions are different, need to merge"
```

### 4. Efficient Synchronization

```
Local repo has: 557db03, a1b2c3d, e4f5a6b
Remote repo has: 557db03, a1b2c3d, e4f5a6b, 9876543

Git knows: Only need to transfer 9876543
```

### 5. Immutable Objects

```
Once an object with hash 557db03... is created,
it can NEVER change.

If content changes → new hash → new object

Git's history is immutable by design!
```

## 📊 Visual Summary

```
┌──────────────────────────────────────────────────────┐
│ You create file: "Hello World"                       │
└───────────────────┬──────────────────────────────────┘
                    ↓
┌──────────────────────────────────────────────────────┐
│ Git prepends header: "blob 11\0Hello World"          │
└───────────────────┬──────────────────────────────────┘
                    ↓
┌──────────────────────────────────────────────────────┐
│ Apply SHA-1 hash function                            │
└───────────────────┬──────────────────────────────────┘
                    ↓
┌──────────────────────────────────────────────────────┐
│ Get hash: 557db03de997c86a4a028e1ebd3a1ceb225be238   │
└───────────────────┬──────────────────────────────────┘
                    ↓
┌──────────────────────────────────────────────────────┐
│ Compress with zlib                                   │
└───────────────────┬──────────────────────────────────┘
                    ↓
┌──────────────────────────────────────────────────────┐
│ Write to .git/objects/55/7db03...                    │
└──────────────────────────────────────────────────────┘
```

## 🎓 Key Takeaways

1. **SHA-1 is a cryptographic hash** that produces 40 hex characters
2. **Git hashes include metadata**: `[type] [size]\0[content]`
3. **Hash determines storage location**: `.git/objects/XX/YY...`
4. **Objects are compressed** with zlib before storage
5. **Content-addressable storage enables**: deduplication, integrity, immutability
6. **Same content always produces same hash** (on any computer!)

## ✅ Checkpoint Quiz

1. What does Git hash when storing a blob? Just the file content, or something more?
2. How many hexadecimal characters are in a SHA-1 hash?
3. Where does Git store an object with hash `abc123...`?
4. What happens if you change one character in a file?
5. Why does Git split objects into subdirectories?

<details>
<summary>Click to see answers</summary>

1. Git hashes `[type] [size]\0[content]`, not just the content
2. 40 hexadecimal characters (160 bits)
3. `.git/objects/ab/c123...` (first 2 chars = dir, rest = filename)
4. Completely different hash, new object stored
5. For filesystem performance (avoid too many files in one directory)
</details>

## 🚧 Common Pitfalls

**Pitfall**: "I'll just hash the file content to get Git's hash"
**Reality**: Need to prepend `blob [size]\0` first!

**Pitfall**: "Git stores files"
**Reality**: Git stores blobs (content). Filenames are in tree objects.

**Pitfall**: "SHA-1 is broken, Git is insecure"
**Reality**: Git is transitioning to SHA-256. Also, SHA-1 collisions are hard to exploit in Git's threat model.

## 🔬 Advanced: Try It Yourself

Create your own content-addressable storage in Bash:

```bash
#!/bin/bash

# Store content
store() {
    local content="$1"
    local size=${#content}
    local data="blob ${size}\0${content}"

    # Compute hash
    local hash=$(echo -en "$data" | sha1sum | cut -d' ' -f1)

    # Create storage directory
    local dir=".myobjects/${hash:0:2}"
    mkdir -p "$dir"

    # Compress and store
    echo -en "$data" | gzip > "${dir}/${hash:2}"

    echo "Stored with hash: $hash"
}

# Retrieve content
retrieve() {
    local hash="$1"
    local file=".myobjects/${hash:0:2}/${hash:2}"

    # Decompress and extract content (skip header)
    zcat "$file" | tail -c +$(($(zcat "$file" | head -c 20 | grep -abo '\x00' | cut -d: -f1) + 1))
}

# Test it
mkdir /tmp/cas-test
cd /tmp/cas-test
mkdir .myobjects

store "Hello World"
# Output: Stored with hash: 557db03de997c86a4a028e1ebd3a1ceb225be238

retrieve "557db03de997c86a4a028e1ebd3a1ceb225be238"
# Output: Hello World
```

Congratulations! You just built a minimal content-addressable storage system!

## 🔜 Next Steps

Now you understand how Git addresses and stores content. Next, we'll explore the **four types of objects** Git stores.

**→ [Lesson 03: Git Objects Explained](03-git-objects-explained.md)**

---

**Phase**: 1 - Foundations
**Lesson**: 02 of 20
**Next**: Git Objects Explained
