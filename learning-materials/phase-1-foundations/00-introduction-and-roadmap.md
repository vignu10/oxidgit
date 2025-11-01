# Introduction and Learning Roadmap

Welcome to your journey of building Git in Rust! This is an ambitious but incredibly rewarding project that will give you deep insights into how version control systems work and make you proficient in Rust.

## 🎯 What You'll Achieve

By the end of this learning path, you will:

1. **Understand Git Deeply**: Know exactly what happens when you run `git add`, `git commit`, `git branch`, etc.
2. **Master Core Rust**: Write idiomatic Rust code with confidence
3. **Build a Working Git Clone**: Have a functional (simplified) version control system
4. **Think Like a Systems Programmer**: Understand file systems, hashing, compression, and data structures

## 📚 Learning Philosophy

This curriculum is designed for someone with **zero knowledge** of Git internals and Rust. We'll:

- Start with fundamental concepts before diving into code
- Use analogies and visual diagrams extensively
- Build incrementally (simple first, then add complexity)
- Focus on hands-on practice over theory
- Connect every concept to practical implementation

## 🗺️ The Complete Roadmap

### **Phase 1: Foundations** (Days 1-3, ~6-10 hours)

**Goal**: Understand what Git really is and how it stores data

- Lesson 00: Introduction and Roadmap (you are here!)
- Lesson 01: What Is Git Really?
- Lesson 02: Content-Addressable Storage
- Lesson 03: Git Objects Explained

**Milestone**: You can explain Git's core architecture to someone else

### **Phase 2: Git Internals Deep Dive** (Days 4-7, ~10-15 hours)

**Goal**: Explore Git's internal structures hands-on

- Lesson 04: Exploring the .git Directory
- Lesson 05: How Git Stores Data
- Lesson 06: Refs and HEAD
- Lesson 07: The Index (Staging Area)

**Milestone**: You can manually create Git objects and understand every file in `.git/`

### **Phase 3: Rust Fundamentals** (Days 8-12, ~15-20 hours)

**Goal**: Learn enough Rust to build our Git implementation

- Lesson 08: Rust Basics for Git
- Lesson 09: Rust Error Handling
- Lesson 10: Rust File I/O
- Lesson 11: Rust Traits and Structs

**Milestone**: You can write Rust programs that read/write files and handle errors

### **Phase 4: Building Git in Rust** (Days 13-25, ~30-40 hours)

**Goal**: Implement core Git commands in Rust

- Lesson 12: Project Setup and Architecture
- Lesson 13: Implementing `hash-object`
- Lesson 14: Implementing `cat-file`
- Lesson 15: Implementing `write-tree`
- Lesson 16: Implementing `commit-tree`
- Lesson 17: Implementing `init`
- Lesson 18: Implementing `add`
- Lesson 19: Implementing `commit`
- Lesson 20: Next Steps and Extensions

**Milestone**: You have a working version control system!

## ⏱️ Time Commitment

Choose a pace that works for you:

| Pace | Hours/Day | Total Time | Best For |
|------|-----------|------------|----------|
| **Light** | 1-2 hours | 4-6 weeks | Working professionals, students |
| **Moderate** | 3-4 hours | 2-3 weeks | Dedicated learning time |
| **Intensive** | 6+ hours | 1-2 weeks | Bootcamp-style immersion |

**Recommendation**: Go at moderate pace. Rushing through won't build deep understanding.

## 📋 Prerequisites

You need:

- **Basic programming knowledge**: Variables, functions, loops (any language)
- **Command line comfort**: Navigate directories, run commands
- **Text editor**: VS Code, Vim, or your favorite editor
- **Curiosity**: Willingness to explore and experiment!

You do NOT need:

- Prior Git expertise (we'll build it from scratch!)
- Rust experience (we'll teach you what you need)
- Systems programming background (we'll explain everything)

## 🛠️ Tools You'll Need

Install these before starting:

1. **Rust** (via rustup):
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Git** (to explore while learning):
   ```bash
   # Ubuntu/Debian
   sudo apt install git

   # macOS
   brew install git
   ```

3. **A text editor** with Rust support:
   - VS Code + rust-analyzer extension (recommended)
   - Or your preferred editor

## 📖 How to Use These Materials

### Daily Study Pattern

1. **Read the lesson** (20-40 minutes)
   - Don't rush, understand each concept
   - Draw diagrams if it helps
   - Take notes

2. **Do hands-on exercises** (20-40 minutes)
   - Actually run the commands
   - Experiment and break things
   - See what happens

3. **Check understanding** (10 minutes)
   - Complete the checkpoint quiz
   - Review key takeaways
   - Can you explain it simply?

4. **Write code** (30-60 minutes for Phase 4)
   - Follow along with examples
   - Type code yourself (don't copy-paste!)
   - Make small modifications

### When You Get Stuck

1. **Re-read the relevant section** - Sometimes it clicks the second time
2. **Do the exercises** - Hands-on practice often clarifies confusion
3. **Look at the glossary** - Reference section has quick definitions
4. **Take a break** - Come back with fresh eyes
5. **Experiment** - Try things out and see what happens!

## 🎓 Learning Outcomes

### After Phase 1
- Explain Git's content-addressable storage model
- Describe the four types of Git objects
- Understand why Git uses SHA-1 hashing

### After Phase 2
- Navigate `.git/` directory with confidence
- Manually create Git objects
- Understand refs, HEAD, and the index
- Explain the complete workflow of a commit

### After Phase 3
- Write Rust programs with proper error handling
- Work with files and directories in Rust
- Use Rust's ownership system correctly
- Implement basic data structures

### After Phase 4
- Have a working Git implementation with core commands
- Understand every line of code you wrote
- Be able to extend your implementation
- Debug Git issues in real projects

## 🚀 What Comes After

Once you complete this curriculum, you can:

1. **Add more Git features**:
   - Branching and merging
   - Remote repositories
   - Diff and patch
   - Rebase and cherry-pick

2. **Optimize your implementation**:
   - Pack files for efficiency
   - Delta compression
   - Better index handling

3. **Build related projects**:
   - Git hosting server
   - Git GUI
   - Custom Git workflows

4. **Contribute to Git**:
   - With this knowledge, you can understand Git's C codebase
   - Fix bugs or add features to Git itself

## 📝 A Note on Learning

This is a challenging project. You will:

- Feel confused sometimes (that's normal!)
- Need to re-read concepts (everyone does!)
- Make mistakes (that's how you learn!)
- Have "aha!" moments (so satisfying!)

**The goal is understanding, not speed.** Take your time. The deep knowledge you gain will be worth it.

## ✅ Ready to Start?

If you have Rust installed and are ready to learn, proceed to:

**→ [Lesson 01: What Is Git Really?](01-what-is-git-really.md)**

Remember: You're about to understand Git better than 95% of developers. Let's go!

---

## Quick Reference

- **Current Phase**: Phase 1 - Foundations
- **Next Lesson**: 01-what-is-git-really.md
- **Estimated Time for This Phase**: 6-10 hours
- **Exercises**: phase-1-exercises.md (complete after all Phase 1 lessons)
