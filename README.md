# oxid - Oxidized Git Implementation

A complete learning journey to understand Git by building it in Rust from scratch!

## 🎯 What is This?

This repository contains:

1. **Comprehensive Learning Materials** - 20+ lessons covering Git internals and Rust
2. **Hands-On Exercises** - Practice problems and challenges
3. **Complete Project Starter** - Rust project structure ready for implementation
4. **Reference Materials** - Glossary and quick references

## 📚 Project Structure

```
oxid/
├── learning-materials/          # Complete course materials
│   ├── README.md               # ⭐ START HERE
│   ├── phase-1-foundations/     # Git concepts
│   ├── phase-2-git-internals/   # How Git works
│   ├── phase-3-rust-fundamentals/ # Rust programming
│   ├── phase-4-building-git/    # Implementation guide
│   ├── exercises/               # Practice problems
│   └── reference/               # Quick references
│
└── rust-git-implementation/  # Your implementation project
    ├── src/
    ├── tests/
    └── Cargo.toml
```

## 🚀 Quick Start

### 1. Install Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Verify installation
rustc --version
cargo --version
```

### 2. Start Learning

```bash
# Read the introduction
cat learning-materials/README.md

# Begin with Lesson 00
cat learning-materials/phase-1-foundations/00-introduction-and-roadmap.md
```

### 3. When You're Ready to Code (After Phase 3)

```bash
cd rust-git-implementation

# Build the project
cargo build

# Run tests
cargo test

# Try the CLI
cargo run -- init
```

## 📖 Learning Path

### Phase 1: Foundations (6-10 hours)
Learn what Git really is and how content-addressable storage works.

**Start here:** [learning-materials/phase-1-foundations/00-introduction-and-roadmap.md](learning-materials/phase-1-foundations/00-introduction-and-roadmap.md)

### Phase 2: Git Internals (10-15 hours)
Explore the `.git` directory and understand every component.

### Phase 3: Rust Fundamentals (15-20 hours)
Learn the Rust you need to build a systems project.

### Phase 4: Building Git (30-40 hours)
Implement Git commands step by step in Rust.

## 🎯 What You'll Build

A working Git implementation with these commands:

- ✅ `oxid init` - Initialize repository
- ✅ `oxid hash-object` - Store objects
- ✅ `oxid cat-file` - Read objects
- ✅ `oxid write-tree` - Create trees
- ✅ `oxid commit` - Create commits
- ✅ `oxid add` - Stage files

**Your implementation will be compatible with real Git!**

## 📚 Course Highlights

### Comprehensive Coverage
- All four Git object types (blob, tree, commit, tag)
- SHA-1 hashing and zlib compression
- Object database and pack files
- References, HEAD, and branches
- The index (staging area)
- Complete commit workflow

### Hands-On Learning
- 50+ hands-on exercises
- Test-driven development approach
- Compare your implementation with real Git
- Debug real Git issues

### Rust Programming
- Ownership and borrowing
- Error handling with Result<T>
- File I/O and system programming
- Building CLI tools with clap
- Writing tests

## 🎓 Learning Outcomes

After completing this course, you will:

- ✅ Understand Git better than 95% of developers
- ✅ Be able to debug Git issues by examining internals
- ✅ Write idiomatic Rust code
- ✅ Have a working version control system
- ✅ Know how to build systems software

## 📅 Time Commitment

Choose your pace:

| Pace | Time/Day | Total Duration |
|------|----------|----------------|
| Light | 1-2 hours | 4-6 weeks |
| Moderate | 3-4 hours | 2-3 weeks (recommended) |
| Intensive | 6+ hours | 1-2 weeks |

## 💡 Why Build Git?

1. **Deep Understanding** - You'll truly understand version control
2. **Practical Skills** - Learn Rust and systems programming
3. **Career Growth** - Stand out with deep technical knowledge
4. **Problem Solving** - Debug Git issues with confidence
5. **Foundation** - Build other tools (Git hosting, custom workflows)

## 🛠️ What You Need

- Basic programming knowledge (any language)
- Command line familiarity
- Curiosity and patience!
- Time to learn and experiment

## 📖 Additional Resources

- [Pro Git Book](https://git-scm.com/book)
- [Git Internals](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain)
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Building Git by James Coglan](https://shop.jcoglan.com/building-git/)

## 🎯 Your First Steps

1. **Read the intro:** `learning-materials/README.md`
2. **Start Phase 1:** `learning-materials/phase-1-foundations/00-introduction-and-roadmap.md`
3. **Join the journey:** Work through lessons at your own pace
4. **Build something amazing:** Create your own Git!

## 📝 Tips for Success

- Don't rush - understanding matters more than speed
- Type code manually - don't copy-paste
- Run all the exercises - hands-on practice is key
- Test frequently - verify each piece works
- Compare with Git - check your implementation is correct
- Take breaks - complex concepts need time to sink in

## 🎉 Let's Begin!

Ready to understand Git at a level most developers never reach?

**Start here:** [learning-materials/README.md](learning-materials/README.md)

Or jump straight to the first lesson: [Lesson 00 - Introduction and Roadmap](learning-materials/phase-1-foundations/00-introduction-and-roadmap.md)

---

**Current Version:** 1.0.0
**Last Updated:** 2025-11-01
**License:** MIT (Educational purposes)

Happy learning! 🚀
