# Git & Rust Glossary

Quick reference for technical terms used throughout the course.

## Git Terms

### A

**Annotated Tag**
A Git object that points to a commit with additional metadata (tagger, message, signature).

**Avalanche Effect**
Property of hash functions where small input changes produce dramatically different outputs.

### B

**Blob (Binary Large Object)**
Git object type that stores file content without filename or metadata.

**Branch**
A movable pointer to a commit, stored as a file in `.git/refs/heads/`.

### C

**Commit**
Git object that represents a snapshot, containing tree ref, parent(s), author, and message.

**Content-Addressable Storage**
Storage system where content's address is derived from its hash (fingerprint).

**Cryptographic Hash**
One-way function producing fixed-size output with collision resistance (e.g., SHA-1).

### D

**Delta Compression**
Storing differences between similar objects instead of full copies.

**Detached HEAD**
State where HEAD points directly to a commit instead of a branch.

### H

**HEAD**
Reference pointing to the current branch or commit.

**Hash**
Fixed-size output of a hash function; Git uses SHA-1 (40 hex characters).

### I

**Index**
Staging area stored in `.git/index`; tracks files to be included in next commit.

### L

**Lightweight Tag**
Simple reference to a commit, just a file containing the commit hash.

**Loose Object**
Individual object file in `.git/objects/XX/YY...` format.

### O

**Object Database**
Git's content-addressable storage in `.git/objects/`.

**Object**
Any of Git's four storage types: blob, tree, commit, tag.

### P

**Pack File**
Compressed archive containing multiple objects with delta compression.

**Parent**
Commit(s) that came before the current commit in history.

**Plumbing Commands**
Low-level Git commands that manipulate objects directly (e.g., `hash-object`).

**Porcelain Commands**
User-friendly Git commands (e.g., `add`, `commit`, `push`).

### R

**Reference (Ref)**
Pointer to a commit, stored in `.git/refs/` (branches, tags).

**Reflog**
History of reference changes, stored in `.git/logs/`.

**Repository**
Directory containing `.git/` with all objects, refs, and metadata.

### S

**SHA-1**
Cryptographic hash algorithm producing 160-bit (40 hex char) output.

**Snapshot**
Complete state of all files at a point in time (what commits store).

**Staging Area**
See Index.

### T

**Tag**
Git object or reference marking a specific commit (usually for releases).

**Tree**
Git object storing directory structure: filenames, permissions, and blob/tree references.

### W

**Working Directory**
Your project files (outside `.git/`).

### Z

**Zlib**
Compression algorithm Git uses for storing objects.

## Rust Terms

### B

**Borrow**
Temporary access to data without taking ownership (via references).

**Borrowing Rules**
- Either one mutable reference OR any number of immutable references
- References must always be valid

### C

**Cargo**
Rust's build system and package manager.

**Crate**
Rust package (library or binary).

### D

**Destructuring**
Extracting values from structs/enums: `let Point { x, y } = point;`

### E

**Enum**
Type that can be one of several variants: `enum Result<T, E> { Ok(T), Err(E) }`

**Error Handling**
Using `Result<T, E>` for recoverable errors, `panic!` for unrecoverable.

### I

**Immutable**
Cannot be changed; variables are immutable by default in Rust.

**impl**
Keyword for implementing methods on types: `impl MyStruct { ... }`

### L

**Lifetime**
How long a reference is valid; prevents dangling references.

### M

**Match**
Pattern matching expression: `match value { pattern => result }`

**Mutable**
Can be changed; requires `mut` keyword: `let mut x = 5;`

**Move**
Transferring ownership from one variable to another.

### O

**Ownership**
Rust's system ensuring each value has a single owner; prevents memory bugs.

**Ownership Rules**:
1. Each value has an owner
2. Only one owner at a time
3. When owner goes out of scope, value is dropped

### R

**Reference**
Borrow of data without taking ownership: `&T` (immutable), `&mut T` (mutable).

**Result<T, E>**
Enum for recoverable errors: `Ok(T)` for success, `Err(E)` for error.

**rustc**
Rust compiler.

**rustup**
Rust toolchain installer.

### S

**Slice**
Reference to a contiguous sequence: `&str` (string slice), `&[T]` (array slice).

**String**
Heap-allocated, growable UTF-8 string.

**&str**
String slice; immutable view into string data.

**Struct**
Custom type grouping related data: `struct Point { x: i32, y: i32 }`

### T

**Trait**
Interface defining behavior: `trait Read { fn read(&mut self) -> Result<u8>; }`

**Type Inference**
Compiler deducing types: `let x = 5;` (infers `i32`).

### U

**Unit Type `()`**
Type with only one value, `()`; like `void` in other languages.

### V

**Vec<T>**
Growable array: `let v: Vec<i32> = vec![1, 2, 3];`

## Common Patterns

### Git Object Format
```
[type] [size]\0[content]

Example for "Hello World":
blob 11\0Hello World
```

### Git Object Storage
```
Hash: 557db03de997c86a4a028e1ebd3a1ceb225be238
Stored at: .git/objects/55/7db03de997c86a4a028e1ebd3a1ceb225be238
            (first 2 chars)/(remaining 38 chars)
```

### Rust Error Handling
```rust
fn might_fail() -> Result<String, Error> {
    let file = File::open("data.txt")?;  // ? propagates errors
    Ok("success".to_string())
}
```

### Rust Ownership Transfer
```rust
let s1 = String::from("hello");
let s2 = s1;  // s1 moved to s2, s1 no longer valid
```

### Rust Borrowing
```rust
fn use_string(s: &String) {  // Borrow
    println!("{}", s);
}

let s = String::from("hello");
use_string(&s);  // s still valid after call
```

## Quick Reference Tables

### Git Object Types

| Type   | Stores                    | Example Content                          |
|--------|---------------------------|------------------------------------------|
| Blob   | File content              | `print("hello")`                        |
| Tree   | Directory structure       | `100644 file.txt → blob abc123`         |
| Commit | Snapshot + metadata       | `tree def456\nauthor...\n\nMessage`     |
| Tag    | Named reference           | `object 123abc\ntag v1.0.0`             |

### Rust String Types

| Type       | Location | Mutable | Use Case                    |
|------------|----------|---------|----------------------------|
| `String`   | Heap     | Yes     | Own and modify strings     |
| `&str`     | Anywhere | No      | Read-only string access    |
| `&mut str` | Anywhere | Yes     | Modify in-place (rare)     |

### Rust Integer Types

| Type  | Signed | Bits | Range                                   |
|-------|--------|------|-----------------------------------------|
| `i8`  | Yes    | 8    | -128 to 127                             |
| `i32` | Yes    | 32   | -2³¹ to 2³¹-1 (default)                |
| `i64` | Yes    | 64   | -2⁶³ to 2⁶³-1                           |
| `u8`  | No     | 8    | 0 to 255                                |
| `u32` | No     | 32   | 0 to 2³²-1                              |
| `u64` | No     | 64   | 0 to 2⁶⁴-1                              |

---

**Tip**: Bookmark this page for quick reference during the course!
