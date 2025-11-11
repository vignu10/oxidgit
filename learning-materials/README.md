# Building Git in Rust - Complete Learning Guide

Welcome! This is your comprehensive guide to understanding Git internals by building a simplified Git implementation in Rust from scratch.

## 🎯 What You'll Learn

- **Git Internals**: How Git really works under the hood
- **Rust Programming**: Systems programming with memory safety
- **Version Control Systems**: The fundamental concepts behind VCS
- **Systems Design**: Building robust, modular software

## 📚 Course Structure

This course is divided into 4 phases with 20+ lessons:

### 📖 Phase 1: Foundations (6-10 hours)

Understand Git's core concepts before diving into code.

- [00 - Introduction and Roadmap](phase-1-foundations/00-introduction-and-roadmap.md) ⭐ START HERE
- [01 - What Is Git Really?](phase-1-foundations/01-what-is-git-really.md)
- [02 - Content-Addressable Storage](phase-1-foundations/02-content-addressable-storage.md)
- [03 - Git Objects Explained](phase-1-foundations/03-git-objects-explained.md)

**Milestone**: You can explain Git's architecture and object model

### 🔍 Phase 2: Git Internals (10-15 hours)

Explore Git's internal structures hands-on.

- [04 - Exploring the .git Directory](phase-2-git-internals/04-exploring-git-directory.md)
- [05 - How Git Stores Data](phase-2-git-internals/05-how-git-stores-data.md)
- [06 - Git References and HEAD](phase-2-git-internals/06-refs-and-head.md)
- [07 - The Git Index (Staging Area)](phase-2-git-internals/07-git-index.md)

**Milestone**: You can navigate and understand every file in `.git/`

### 🦀 Phase 3: Rust Fundamentals (15-20 hours)

Learn the Rust you need to build Git.

- [08 - Rust Basics for Git](phase-3-rust-fundamentals/08-rust-basics-for-git.md)
- [09 - Rust Error Handling](phase-3-rust-fundamentals/09-rust-error-handling.md)
- [10 - Working with Files and Paths in Rust](phase-3-rust-fundamentals/10-rust-file-io.md)
- [11 - Parsing and Serialization in Rust](phase-3-rust-fundamentals/11-rust-parsing.md)

**Milestone**: You can write Rust programs that work with files and handle errors

### 🏗️ Phase 4: Building Git (30-40 hours)

Implement Git commands step by step.

- [12 - Project Setup and Architecture](phase-4-building-git/12-project-setup-and-architecture.md)
- [13 - Implementing hash-object](phase-4-building-git/13-implementing-hash-object.md)
- 14 - Implementing cat-file
- 15 - Implementing write-tree
- 16 - Implementing commit-tree
- 17 - Implementing init
- 18 - Implementing add
- 19 - Implementing commit
- 20 - Next Steps and Extensions

**Milestone**: You have a working Git implementation!

## 🚀 Getting Started

### Prerequisites

- Basic programming knowledge (any language)
- Command line familiarity
- Curiosity and patience!

### Required Tools

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Git (to compare against)
# Ubuntu/Debian:
sudo apt install git

# macOS:
brew install git

# Verify installations
rustc --version
cargo --version
git --version
```

### Recommended Setup

- **Editor**: VS Code with rust-analyzer extension
- **Terminal**: Any Unix-like terminal (Linux, macOS, WSL on Windows)
- **Time**: 2-6 weeks depending on pace

## 📅 Learning Paths

Choose a pace that works for you:

### 🐌 Light Pace (4-6 weeks)
- 1-2 hours per day
- Great for working professionals
- Allows time to absorb concepts

### 🚶 Moderate Pace (2-3 weeks)
- 3-4 hours per day
- Recommended approach
- Good balance of speed and understanding

### 🏃 Intensive Pace (1-2 weeks)
- 6+ hours per day
- Bootcamp-style immersion
- Best if you have dedicated time

## 🗺️ How to Use This Guide

### Daily Study Pattern

1. **Read** (30-45 min): Work through the lesson
2. **Experiment** (30-45 min): Run the hands-on exercises
3. **Code** (30-60 min): Write and test code (in Phase 4)
4. **Review** (10-15 min): Complete checkpoint quiz

### When You Get Stuck

1. Re-read the relevant section
2. Run the hands-on examples
3. Check the reference materials
4. Take a break and come back fresh
5. Experiment and explore!

## 📊 Progress Tracking

Use this checklist to track your progress:

**Phase 1 - Foundations**
- [ ] Lesson 00: Introduction
- [ ] Lesson 01: What Is Git Really?
- [ ] Lesson 02: Content-Addressable Storage
- [ ] Lesson 03: Git Objects Explained

**Phase 2 - Git Internals**
- [ ] Lesson 04: Exploring .git Directory
- [ ] Lesson 05: How Git Stores Data
- [ ] Lesson 06: Git References and HEAD
- [ ] Lesson 07: The Git Index

**Phase 3 - Rust Fundamentals**
- [ ] Lesson 08: Rust Basics for Git
- [ ] Lesson 09: Rust Error Handling
- [ ] Lesson 10: Working with Files and Paths
- [ ] Lesson 11: Parsing and Serialization

**Phase 4 - Building Git**
- [ ] Lesson 12: Project Setup
- [ ] Lesson 13: hash-object
- [ ] Lesson 14: cat-file
- [ ] Lesson 15: write-tree
- [ ] Lesson 16: commit-tree
- [ ] Lesson 17: init
- [ ] Lesson 18: add
- [ ] Lesson 19: commit

## 🎯 Learning Outcomes

### After Phase 1
✅ Explain content-addressable storage
✅ Describe Git's four object types
✅ Understand why Git uses SHA-1 hashing

### After Phase 2
✅ Navigate `.git/` directory confidently
✅ Manually create Git objects
✅ Explain complete workflow of a commit

### After Phase 3
✅ Write Rust programs with proper error handling
✅ Use Rust's ownership system correctly
✅ Work with files and data structures

### After Phase 4
✅ Have a working Git implementation
✅ Understand every Git command you use
✅ Debug Git issues in real projects
✅ Extend your implementation with new features

## 🛠️ Project Structure

This repository contains:

```
learning-materials/
├── README.md (you are here)
├── phase-1-foundations/
│   ├── 00-introduction-and-roadmap.md
│   ├── 01-what-is-git-really.md
│   ├── 02-content-addressable-storage.md
│   └── 03-git-objects-explained.md
├── phase-2-git-internals/
│   ├── 04-exploring-git-directory.md
│   ├── 05-how-git-stores-data.md
│   ├── 06-refs-and-head.md
│   └── 07-git-index.md
├── phase-3-rust-fundamentals/
│   ├── 08-rust-basics-for-git.md
│   ├── 09-rust-error-handling.md
│   ├── 10-rust-file-io.md
│   └── 11-rust-parsing.md
├── phase-4-building-git/
│   ├── 12-project-setup-and-architecture.md
│   └── 13-implementing-hash-object.md
├── exercises/
│   ├── phase-1-exercises.md
│   ├── phase-2-exercises.md (NEW!)
│   ├── phase-3-exercises.md (NEW!)
│   └── phase-4-exercises.md
└── reference/
    └── glossary.md
```

Your implementation will go in:

```
rust-git-implementation/
```

## 💡 Tips for Success

1. **Don't rush**: Understanding matters more than speed
2. **Type code manually**: Don't copy-paste, type it yourself
3. **Experiment**: Try modifying examples to see what happens
4. **Test frequently**: Run tests after each change
5. **Compare with Git**: Verify your implementation matches Git's behavior
6. **Take breaks**: Complex concepts need time to sink in
7. **Build incrementally**: Get each piece working before moving on

## 🤝 Community

This is a self-paced learning journey, but you don't have to be alone:

- Share your progress and questions
- Help others who are learning
- Contribute improvements to these materials

## 📖 Additional Resources

### Git Internals
- [Pro Git Book - Git Internals](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain)
- [Git from the Bottom Up](https://jwiegley.github.io/git-from-the-bottom-up/)
- [Building Git by James Coglan](https://shop.jcoglan.com/building-git/)

### Rust Learning
- [The Rust Book](https://doc.rust-lang.org/book/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rustlings](https://github.com/rust-lang/rustlings) - Small exercises

### Similar Projects
- [gitoxide](https://github.com/Byron/gitoxide) - Pure Rust Git implementation
- [libgit2](https://github.com/libgit2/libgit2) - C library for Git

## 🎓 What's Next After Completion?

Once you finish this course, you can:

1. **Add More Features**
   - Branching and merging
   - Remote repositories (push/pull)
   - Diff and patch
   - Rebase and cherry-pick

2. **Optimize Performance**
   - Pack files and delta compression
   - Parallel object processing
   - Memory-mapped files

3. **Build Related Tools**
   - Git hosting server
   - Custom merge strategies
   - Git GUI or TUI

4. **Contribute to Git**
   - With this knowledge, you can understand Git's C codebase
   - Fix bugs or add features

## ✨ Ready to Begin?

Start your journey here:

**→ [Phase 1 - Lesson 00: Introduction and Roadmap](phase-1-foundations/00-introduction-and-roadmap.md)**

Remember: You're about to understand Git better than 95% of developers. Let's build something amazing!

---

**Current Version**: 1.0.0
**Last Updated**: 2025-11-01
**Estimated Completion Time**: 2-6 weeks depending on pace

Happy learning! 🚀
