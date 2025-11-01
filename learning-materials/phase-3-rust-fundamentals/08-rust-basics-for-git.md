# Lesson 08: Rust Basics for Git

**Estimated Time**: 3-4 hours
**Prerequisites**: Phase 1 & 2 complete

## 🎯 Learning Objectives

By the end of this lesson, you will:

1. Understand Rust's ownership system
2. Write basic Rust programs
3. Use Rust's type system effectively
4. Understand borrowing and references
5. Work with strings and vectors

## 🦀 Why Rust for Git?

Rust is perfect for building Git because:

1. **Memory safety without garbage collection** - like C, but safe
2. **Zero-cost abstractions** - performance of C, ergonomics of high-level languages
3. **Strong type system** - catch bugs at compile time
4. **Great tooling** - cargo, clippy, rustfmt

## 📝 Your First Rust Program

### Hello World

Create `hello.rs`:

```rust
fn main() {
    println!("Hello, World!");
}
```

Compile and run:

```bash
rustc hello.rs
./hello
# Output: Hello, World!
```

### Using Cargo (Rust's build tool)

```bash
# Create new project
cargo new hello_world
cd hello_world

# Project structure:
# hello_world/
# ├── Cargo.toml  (like package.json)
# └── src/
#     └── main.rs

# Run it
cargo run
# Output: Hello, World!
```

## 🧱 Rust Fundamentals

### Variables and Mutability

```rust
fn main() {
    // Immutable by default (like 'const' in JS)
    let x = 5;
    // x = 6;  // ❌ Error! Cannot assign twice

    // Mutable variable
    let mut y = 5;
    y = 6;  // ✅ OK!

    println!("x = {}, y = {}", x, y);
}
```

**Why immutable by default?**
- Prevents accidental changes
- Makes code easier to reason about
- Compiler can optimize better

### Types

```rust
fn main() {
    let integer: i32 = 42;          // 32-bit signed integer
    let unsigned: u32 = 42;         // 32-bit unsigned integer
    let float: f64 = 3.14;          // 64-bit float
    let boolean: bool = true;       // Boolean
    let character: char = 'A';      // Unicode character

    // Type inference works
    let inferred = 42;  // Rust knows this is i32
}
```

### Functions

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b  // No semicolon = return value
}

fn print_number(n: i32) {
    println!("Number: {}", n);
    // No return type = returns () (unit type, like void)
}

fn main() {
    let sum = add(5, 3);
    print_number(sum);
}
```

## 🔑 Ownership: Rust's Superpower

This is THE key concept in Rust:

### Ownership Rules

1. Each value has a single owner
2. When the owner goes out of scope, the value is dropped
3. You can transfer ownership (move) or borrow

### Example: Moving Ownership

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1;  // Ownership moved to s2

    // println!("{}", s1);  // ❌ Error! s1 no longer owns the string
    println!("{}", s2);  // ✅ OK!
}
```

### Why This Matters for Git

When building Git, you'll create and pass around objects (blobs, trees, commits). Ownership ensures:

- **No double-free errors**: Can't accidentally free the same memory twice
- **No use-after-free**: Can't use data after it's been deallocated
- **No data races**: Can't have multiple threads modifying the same data

### Borrowing (References)

Instead of transferring ownership, you can borrow:

```rust
fn print_string(s: &String) {  // &String = borrow
    println!("{}", s);
}

fn main() {
    let s = String::from("hello");
    print_string(&s);  // Borrow s
    println!("{}", s);  // Still own s, can use it!
}
```

### Mutable Borrows

```rust
fn append_world(s: &mut String) {
    s.push_str(" world");
}

fn main() {
    let mut s = String::from("hello");
    append_world(&mut s);
    println!("{}", s);  // "hello world"
}
```

**Rules**:
- Can have EITHER one mutable borrow OR multiple immutable borrows
- Cannot have mutable and immutable borrows simultaneously

## 📚 Strings in Rust

Rust has two string types:

### `&str` (String Slice)

```rust
let s: &str = "hello";  // String literal, immutable, stack-allocated
```

### `String` (Owned String)

```rust
let s: String = String::from("hello");  // Heap-allocated, growable
```

### Converting Between Them

```rust
let slice: &str = "hello";
let owned: String = slice.to_string();
let back_to_slice: &str = &owned;
```

### For Git Implementation

- Use `&str` for function parameters (don't need to own the string)
- Use `String` when you need to own or modify strings

## 📦 Collections: Vec<T>

Vectors are growable arrays:

```rust
fn main() {
    // Create empty vector
    let mut numbers: Vec<i32> = Vec::new();

    // Add elements
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    // Or use vec! macro
    let numbers = vec![1, 2, 3];

    // Access elements
    println!("First: {}", numbers[0]);

    // Iterate
    for num in &numbers {
        println!("{}", num);
    }
}
```

### For Git Implementation

```rust
// Store object data as bytes
let content: Vec<u8> = vec![/* ... */];

// Store tree entries
let entries: Vec<TreeEntry> = vec![/* ... */];
```

## 🏗️ Structs (Custom Types)

```rust
struct Blob {
    hash: String,
    content: Vec<u8>,
    size: usize,
}

impl Blob {
    // Constructor
    fn new(content: Vec<u8>) -> Blob {
        let size = content.len();
        let hash = compute_hash(&content);  // We'll implement this later

        Blob {
            hash,
            content,
            size,
        }
    }

    // Method
    fn display(&self) {
        println!("Blob {} ({} bytes)", self.hash, self.size);
    }
}

fn main() {
    let blob = Blob::new(vec![1, 2, 3]);
    blob.display();
}
```

## 🎯 Practice: Git-Related Examples

### Example 1: Compute SHA-1 Hash

```rust
use sha1::{Sha1, Digest};

fn hash_content(content: &[u8]) -> String {
    let mut hasher = Sha1::new();
    hasher.update(content);
    let result = hasher.finalize();

    // Convert to hex string
    format!("{:x}", result)
}

fn main() {
    let content = b"Hello World";
    let hash = hash_content(content);
    println!("Hash: {}", hash);
}
```

Add to `Cargo.toml`:
```toml
[dependencies]
sha1 = "0.10"
```

### Example 2: Create Git Object Format

```rust
fn create_blob_data(content: &[u8]) -> Vec<u8> {
    let header = format!("blob {}\0", content.len());
    let mut data = header.as_bytes().to_vec();
    data.extend_from_slice(content);
    data
}

fn main() {
    let content = b"Hello World";
    let blob_data = create_blob_data(content);

    println!("Blob data: {:?}", blob_data);
    // Output: [98, 108, 111, 98, 32, 49, 49, 0, 72, 101, 108, 108, 111, ...]
    //         b   l   o   b       1   1   \0  H   e   l   l   o   ...
}
```

### Example 3: Read File into Blob

```rust
use std::fs;

fn read_file_as_blob(path: &str) -> std::io::Result<Vec<u8>> {
    let content = fs::read(path)?;
    let blob_data = create_blob_data(&content);
    Ok(blob_data)
}

fn main() -> std::io::Result<()> {
    let data = read_file_as_blob("hello.txt")?;
    println!("Read {} bytes", data.len());
    Ok(())
}
```

## 🎓 Key Takeaways

1. **Variables are immutable by default** - use `mut` for mutable
2. **Ownership prevents memory bugs** - each value has one owner
3. **Borrowing allows temporary access** - `&T` for immutable, `&mut T` for mutable
4. **Two string types**: `&str` (borrowed) and `String` (owned)
5. **Vec<T>** is the growable array type
6. **Structs + impl** define custom types with methods

## ✅ Checkpoint Quiz

1. What's the difference between `let x = 5` and `let mut x = 5`?
2. What happens when you assign `let s2 = s1` where `s1` is a `String`?
3. What's the difference between `&str` and `String`?
4. How do you create a vector of integers?
5. What does `&mut` mean?

<details>
<summary>Click to see answers</summary>

1. First is immutable, second is mutable
2. Ownership moves from `s1` to `s2`; `s1` is no longer valid
3. `&str` is a string slice (borrowed), `String` is owned and growable
4. `let v: Vec<i32> = Vec::new()` or `let v = vec![1, 2, 3]`
5. Mutable borrow/reference
</details>

## 🧪 Hands-On Exercises

### Exercise 1: Hash "Hello World"

Write a program that:
1. Creates the Git blob format for "Hello World"
2. Computes its SHA-1 hash
3. Prints the hash

Expected output: `557db03de997c86a4a028e1ebd3a1ceb225be238`

### Exercise 2: Blob Struct

Create a `Blob` struct that:
1. Stores content as `Vec<u8>`
2. Computes and stores the hash
3. Has a method to display blob info

### Exercise 3: File Reader

Write a function that:
1. Reads a file from disk
2. Creates Git blob format
3. Returns the hash

## 🔜 Next Steps

Now you know Rust basics! Next, we'll learn error handling - critical for robust Git implementation.

**→ [Lesson 09: Rust Error Handling](09-rust-error-handling.md)**

---

**Phase**: 3 - Rust Fundamentals
**Lesson**: 08 of 20
