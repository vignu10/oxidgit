# Lesson 01: What Is Git Really?

**Estimated Time**: 1.5-2 hours
**Prerequisites**: None - this is our starting point!

## 🎯 Learning Objectives

By the end of this lesson, you will be able to:

1. Explain what Git is at its core (not just "version control")
2. Understand the key insight that makes Git different
3. Describe Git's two-layer architecture
4. Identify why Git is called "content-addressable"

## 🤔 What Most People Think Git Is

Most developers think of Git as:

- A tool for tracking code changes
- Something that lets you go back to old versions
- A way to collaborate with other developers
- That thing with confusing commands like `rebase` and `cherry-pick`

**All of this is true, but it misses the fundamental insight.**

## 💡 What Git Actually Is

At its core, Git is:

> **A content-addressable filesystem with a version control system (VCS) built on top.**

Let's break this down:

### Content-Addressable Filesystem

Imagine a library where books are organized not by title or author, but by their **content**. If two books have identical content, they get the same shelf location - even if they have different covers.

Git stores data the same way:

1. You give Git some content (a file, a directory structure, a commit message)
2. Git computes a unique "fingerprint" (hash) of that content
3. Git stores the content using that fingerprint as the address

**Key insight**: The content's fingerprint IS its address. Same content = same address.

### Example: Real-World Analogy

Think of a safety deposit box system where:

- Your box number is generated from what you put inside
- If you put in "Hello World", it always goes to box #123456
- If someone else puts in "Hello World", they get box #123456 too
- The box number changes ONLY if the contents change

This is exactly how Git works!

## 🏗️ Git's Two-Layer Architecture

Git has two distinct layers:

```
┌─────────────────────────────────────┐
│   Porcelain Commands (User Layer)  │  ← What you normally use
│                                     │     (add, commit, push, branch...)
│   git add, git commit, git log      │
└─────────────────────────────────────┘
              ↓  ↓  ↓
┌─────────────────────────────────────┐
│   Plumbing Commands (Core Layer)    │  ← What's actually happening
│                                     │     (hash-object, cat-file...)
│   Content-addressable storage       │
│   Object database                   │
└─────────────────────────────────────┘
```

### Porcelain Layer (What You See)

These are the friendly commands you use daily:

- `git add` - Stage files for commit
- `git commit` - Save a snapshot
- `git branch` - Create/manage branches
- `git merge` - Combine branches

They're called "porcelain" because they're the polished, user-friendly interface.

### Plumbing Layer (What Git Does)

These are the low-level commands that do the real work:

- `git hash-object` - Store content and get its hash
- `git cat-file` - Retrieve content by hash
- `git update-ref` - Update branch pointers
- `git write-tree` - Store directory structure

They're called "plumbing" because they're the hidden infrastructure.

**Our project will build the plumbing layer first, then add porcelain on top!**

## 🔑 The Key Insight: Content is Everything

In traditional filesystems:

```
Filename → Location → Content

"hello.txt" lives at /home/user/hello.txt → Contains "Hello World"
```

The filename determines where we find the content.

In Git's content-addressable system:

```
Content → Hash → Storage Location

"Hello World" → SHA-1 hash: 557db03... → Stored at .git/objects/55/7db03...
```

The content determines its own storage location!

### Why This Matters

1. **Deduplication is automatic**: Same content is stored only once
2. **Integrity is built-in**: Change one byte, the hash changes completely
3. **Distribution is easy**: Hashes are the same everywhere
4. **Merging is possible**: We can compare content by comparing hashes

## 📊 Visual Example

Let's see what happens when you save "Hello World":

```
Input: "Hello World"
       ↓
Apply SHA-1 hash function
       ↓
Get hash: 557db03de997c86a4a028e1ebd3a1ceb225be238
       ↓
Store at: .git/objects/55/7db03de997c86a4a028e1ebd3a1ceb225be238
       ↓
Compress with zlib
       ↓
Write to disk
```

Now when you want "Hello World" back:

```
Request: hash 557db03...
         ↓
Read from: .git/objects/55/7db03...
         ↓
Decompress with zlib
         ↓
Return: "Hello World"
```

**Notice**: We never used a filename! Content addressed by its hash.

## 🎨 Git vs Traditional VCS

### Traditional Systems (SVN, Perforce)

Store **changes** (deltas):

```
Version 1: Full file
Version 2: +added line 5, -deleted line 3
Version 3: +added line 7
```

To get Version 3, you need to replay all changes from Version 1.

### Git

Stores **snapshots**:

```
Version 1: Full snapshot
Version 2: Full snapshot (reusing unchanged files)
Version 3: Full snapshot (reusing unchanged files)
```

Each commit is a complete snapshot. Git uses hashes to avoid duplicating unchanged content.

## 🧩 Git's Four Types of Objects

Git stores four types of objects in its content-addressable database:

### 1. Blob (Binary Large Object)

- Stores file **content** (just the data, no filename)
- "Hello World" is one blob
- "print('hi')" is another blob

### 2. Tree

- Stores directory structure
- Lists blobs and other trees with their filenames
- Like a directory listing

### 3. Commit

- Stores a snapshot in time
- Points to a tree (the root directory)
- Contains metadata: author, message, timestamp, parent commit(s)

### 4. Tag

- Stores a named reference to a commit
- Used for releases (v1.0.0)

**We'll explore each type in detail in Lesson 03.**

## 🔬 Hands-On: See Git's Content-Addressable Storage

Let's peek under the hood of a real Git repository!

### Exercise 1: Create and Explore

```bash
# Create a test directory
cd /tmp
mkdir git-exploration
cd git-exploration

# Initialize a Git repository
git init

# Check what's in .git/
ls -la .git/

# Look at the objects directory
ls -la .git/objects/
```

**What you'll see**: Almost empty! Just `info/` and `pack/` subdirectories.

### Exercise 2: Store Your First Object

```bash
# Create a file
echo "Hello World" > hello.txt

# Tell Git to track it
git add hello.txt

# Look at objects directory now
ls -la .git/objects/

# You'll see a new directory like "55/" or similar
# Let's see what's inside
find .git/objects -type f
```

**What you'll see**: Something like `.git/objects/55/7db03de997c86a4a028e1ebd3a1ceb225be238`

The first 2 characters (55) are the directory name. The rest is the filename.

### Exercise 3: Examine the Object

```bash
# Let's see what type of object it is
git cat-file -t 557db03de997c86a4a028e1ebd3a1ceb225be238

# Output: blob

# Let's see its content
git cat-file -p 557db03de997c86a4a028e1ebd3a1ceb225be238

# Output: Hello World
```

**Insight**: Git stored the *content* "Hello World" and gave it the hash `557db03...`. No mention of "hello.txt" yet!

### Exercise 4: Same Content, Same Hash

```bash
# Create a different file with same content
echo "Hello World" > different.txt

# Add it
git add different.txt

# Check objects
find .git/objects -type f
```

**What you'll see**: Still only one object! Git didn't store "Hello World" twice.

This is content-addressable storage in action.

## 🎓 Key Takeaways

1. **Git is a content-addressable filesystem first**, version control second
2. **Content determines its own storage address** via SHA-1 hashing
3. **Git has two layers**: plumbing (low-level) and porcelain (user-friendly)
4. **Git stores snapshots, not changes** (but reuses unchanged content)
5. **Four object types**: blob, tree, commit, tag
6. **Deduplication is automatic** - same content stored only once

## ✅ Checkpoint Quiz

Test your understanding:

1. What does "content-addressable" mean?
2. What determines where Git stores a piece of content?
3. Name Git's two layers and explain the difference
4. What happens if two files have identical content?
5. What are Git's four object types?

<details>
<summary>Click to see answers</summary>

1. Content is stored at an address determined by its hash (fingerprint)
2. The SHA-1 hash of the content itself
3. Plumbing (low-level object storage) and Porcelain (user-friendly commands)
4. Git stores the content once and both files reference the same object
5. Blob, Tree, Commit, Tag
</details>

## 🚧 Common Misconceptions

**Misconception**: "Git stores differences between file versions"
**Reality**: Git stores complete snapshots, but reuses unchanged content

**Misconception**: "Git tracks files"
**Reality**: Git tracks content. Filenames are stored separately in tree objects

**Misconception**: "Git is complicated"
**Reality**: Git's core is simple - content-addressable storage. The complexity is in the porcelain commands

## 🔜 Next Steps

Now that you understand Git's fundamental architecture, we'll dive deeper into **content-addressable storage** in the next lesson.

**→ [Lesson 02: Content-Addressable Storage](02-content-addressable-storage.md)**

## 📚 Further Reading

- [Git from the Bottom Up](https://jwiegley.github.io/git-from-the-bottom-up/) - Classic deep dive
- [Git Internals](https://git-scm.com/book/en/v2/Git-Internals-Plumbing-and-Porcelain) - Official docs
- Your own experiments with `git cat-file` and exploring `.git/`!

---

**Phase**: 1 - Foundations
**Lesson**: 01 of 20
**Next**: Content-Addressable Storage
