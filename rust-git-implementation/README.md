# oxid - Oxidized Git Implementation

A Git implementation in Rust built from scratch for learning purposes.

## What is oxid?

oxid is a simplified Git implementation that demonstrates how Git works internally. By building this, you'll understand:

- Content-addressable storage
- Object database (blobs, trees, commits)
- References and HEAD
- The staging area (index)
- Core Git commands

## Installation

### Prerequisites

First, install Rust if you haven't already:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

Verify installation:

```bash
rustc --version
cargo --version
```

### Building oxid

```bash
cd /home/matrix/oxidgit/rust-git-implementation

# Build the project
cargo build

# Run tests
cargo test

# Install locally
cargo install --path .
```

## Usage

### Initialize a repository

```bash
oxid init
# or
oxid init /path/to/repo
```

### Hash an object

```bash
# Just compute hash
oxid hash-object myfile.txt

# Compute hash and write to database
oxid hash-object -w myfile.txt
```

### Read an object

```bash
# Show object type
oxid cat-file -t 557db03

# Pretty-print object content
oxid cat-file -p 557db03
```

### Create a commit

```bash
# Stage files (not implemented yet)
oxid add .

# Create commit
oxid commit -m "Initial commit"
```

## Project Structure

```
oxid/
├── src/
│   ├── main.rs              # CLI entry point
│   ├── lib.rs               # Library root
│   ├── commands/            # Git commands
│   │   ├── mod.rs
│   │   ├── init.rs
│   │   ├── hash_object.rs
│   │   └── cat_file.rs
│   ├── objects/             # Git objects
│   │   ├── mod.rs
│   │   ├── blob.rs
│   │   ├── tree.rs
│   │   ├── commit.rs
│   │   └── object.rs
│   ├── repository.rs        # Repository operations
│   ├── index.rs             # Staging area
│   └── utils.rs             # Helper functions
├── tests/                   # Integration tests
└── Cargo.toml              # Project manifest
```

## Implemented Features

- [x] Repository initialization (`init`)
- [x] Hash object (`hash-object`)
- [ ] Read object (`cat-file`)
- [ ] Write tree (`write-tree`)
- [ ] Create commit (`commit`)
- [ ] Staging area (`add`)
- [ ] Branch management
- [ ] Merge

## Development

### Running Tests

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_blob_hash

# Run with output
cargo test -- --nocapture
```

### Code Style

```bash
# Format code
cargo fmt

# Lint code
cargo clippy
```

## Compatibility with Git

oxid creates objects compatible with Git! You can:

```bash
# Create object with oxid
oxid hash-object -w file.txt

# Read with real git
git cat-file -p <hash>

# Or vice versa!
```

## Learning Resources

See the complete learning materials in `/home/matrix/oxidgit/learning-materials/`:

- Phase 1: Git Foundations
- Phase 2: Git Internals
- Phase 3: Rust Fundamentals
- Phase 4: Building Git (this project!)

## License

MIT License - built for educational purposes

## Acknowledgments

- [Pro Git Book](https://git-scm.com/book)
- [Building Git by James Coglan](https://shop.jcoglan.com/building-git/)
- [The Rust Programming Language](https://doc.rust-lang.org/book/)
