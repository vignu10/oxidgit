# Lesson 04: Exploring the .git Directory

**Estimated Time**: 1.5-2 hours
**Prerequisites**: Phase 1 complete

## 🎯 Learning Objectives

By the end of this lesson, you will:

1. Know every important file and directory in `.git/`
2. Understand what each component does
3. Be able to manually navigate Git's internal structure
4. Recognize when Git creates or modifies internal files
5. Debug issues by examining `.git/`

## 📁 The .git Directory Structure

When you run `git init`, Git creates this structure:

```
.git/
├── HEAD                  # Points to current branch
├── config                # Repository configuration
├── description           # Repository description (for GitWeb)
├── hooks/                # Scripts that run on Git events
│   ├── pre-commit.sample
│   ├── post-commit.sample
│   └── ...
├── info/                 # Global exclude patterns
│   └── exclude
├── objects/              # The object database
│   ├── pack/            # Packed objects (compressed)
│   └── info/            # Object database metadata
├── refs/                 # References (branches, tags)
│   ├── heads/           # Local branches
│   ├── tags/            # Tags
│   └── remotes/         # Remote branches
├── index                 # Staging area (created on first 'git add')
├── logs/                 # Reference change history
│   ├── HEAD
│   └── refs/
└── packed-refs           # Packed references (performance)
```

Let's explore each component!

## 🔍 Core Files

### HEAD

**Purpose**: Points to the currently checked-out reference

```bash
# Check what HEAD points to
cat .git/HEAD
# Output: ref: refs/heads/main

# This means HEAD → refs/heads/main → (commit hash)
```

#### HEAD States

**Normal state** (on a branch):
```
ref: refs/heads/main
```

**Detached HEAD** (directly on a commit):
```
a1b2c3d4e5f6... (commit hash)
```

### config

**Purpose**: Repository-specific configuration

```bash
cat .git/config
```

Example content:
```ini
[core]
    repositoryformatversion = 0
    filemode = true
    bare = false
[remote "origin"]
    url = https://github.com/user/repo.git
    fetch = +refs/heads/*:refs/remotes/origin/*
[branch "main"]
    remote = origin
    merge = refs/heads/main
```

### index

**Purpose**: The staging area (binary file)

```bash
# View index contents (human-readable)
git ls-files --stage

# Output:
# 100644 557db03... 0	hello.txt
# 100644 a1b2c3d... 0	src/main.py
#  mode   hash    stage  filename
```

The index tracks:
- Staged files
- Their hashes
- Metadata (permissions, timestamps)

**Created when**: First `git add` in a new repository

## 📂 Important Directories

### objects/

**Purpose**: The content-addressable storage

#### Structure

```
objects/
├── 55/
│   └── 7db03de997c86a4a028e1ebd3a1ceb225be238  (blob)
├── a1/
│   └── b2c3d4e5f6...  (tree)
├── 8f/
│   └── 9a0b1c2d3e...  (commit)
├── pack/
│   ├── pack-abc123....pack  (packed objects)
│   └── pack-abc123....idx   (pack index)
└── info/
    └── packs  (list of pack files)
```

#### Loose Objects

Individual files in `objects/XX/YYYY...` format:

```bash
# Find all loose objects
find .git/objects -type f | grep -v pack | head -5

# Examine one
git cat-file -t 557db03  # Type
git cat-file -p 557db03  # Content
```

#### Pack Files

Git compresses multiple objects into pack files for efficiency:

```bash
# See pack files
ls .git/objects/pack/

# Verify a pack
git verify-pack -v .git/objects/pack/pack-*.idx
```

**When created**: During `git gc`, `git push`, or when too many loose objects exist

### refs/

**Purpose**: Store references (branches, tags, remotes)

```
refs/
├── heads/              # Local branches
│   ├── main           # Contains commit hash
│   ├── feature        # Another branch
│   └── bugfix         # Yet another
├── tags/              # Tags
│   └── v1.0.0        # Contains commit hash or tag object hash
└── remotes/           # Remote-tracking branches
    └── origin/
        ├── main      # Origin's main branch
        └── develop   # Origin's develop branch
```

#### Heads (Branches)

```bash
# View main branch
cat .git/refs/heads/main
# Output: 8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7

# This is just a file with a commit hash!
```

Creating a branch is just creating a file:

```bash
# Create branch manually
echo "8f9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7" > .git/refs/heads/my-branch

# Verify
git branch
# * main
#   my-branch
```

#### Tags

```bash
# Lightweight tag (just a file)
cat .git/refs/tags/v1.0.0
# Output: 8f9a0b1...  (commit hash)

# Annotated tag (points to tag object)
cat .git/refs/tags/v2.0.0
# Output: a1b2c3d...  (tag object hash)

git cat-file -t a1b2c3d
# Output: tag
```

### logs/

**Purpose**: Reflog - history of ref changes

```bash
# View HEAD history
cat .git/logs/HEAD

# Each line:
# old-hash new-hash Author <email> timestamp action
# Example:
# 0000000 a1b2c3d John <john@example.com> 1634567890 +0000  commit (initial): First
# a1b2c3d b2c3d4e John <john@example.com> 1634567900 +0000  commit: Second
```

This powers `git reflog`:

```bash
git reflog
# b2c3d4e HEAD@{0}: commit: Second
# a1b2c3d HEAD@{1}: commit (initial): First
```

### hooks/

**Purpose**: Custom scripts triggered by Git events

```bash
ls .git/hooks/
# pre-commit.sample
# pre-push.sample
# ...
```

#### Common Hooks

- `pre-commit`: Runs before commit (e.g., run linters)
- `post-commit`: Runs after commit (e.g., send notification)
- `pre-push`: Runs before push (e.g., run tests)

#### Creating a Hook

```bash
# Remove .sample extension and make executable
mv .git/hooks/pre-commit.sample .git/hooks/pre-commit
chmod +x .git/hooks/pre-commit

# Edit to add your script
echo '#!/bin/bash\necho "Running pre-commit hook!"\npytest' > .git/hooks/pre-commit
```

## 🧪 Hands-On Exploration

### Exercise 1: Track Object Creation

```bash
cd /tmp/git-explore
git init

# Count objects initially
find .git/objects -type f | wc -l
# Output: 0

# Create a file
echo "Hello" > file.txt

# Still 0 objects
find .git/objects -type f | wc -l
# Output: 0 (not in Git yet!)

# Stage it
git add file.txt

# Now we have an object!
find .git/objects -type f | wc -l
# Output: 1

# Find it
find .git/objects -type f
# .git/objects/e9/65047ad7c57865823c7d992b1d046ea66edf78

# What type?
git cat-file -t e965047
# Output: blob

# Commit
git commit -m "Add file"

# More objects!
find .git/objects -type f | wc -l
# Output: 3 (blob + tree + commit)
```

### Exercise 2: Follow the Reference Chain

```bash
# Where does HEAD point?
cat .git/HEAD
# Output: ref: refs/heads/main

# Where does main point?
cat .git/refs/heads/main
# Output: a1b2c3d...

# What is that?
git cat-file -t a1b2c3d
# Output: commit

# Examine it
git cat-file -p a1b2c3d
# tree e4f5a6b...
# author ...
#
# Add file

# Follow the tree
git cat-file -p e4f5a6b
# 100644 blob e965047...  file.txt

# Follow the blob
git cat-file -p e965047
# Hello
```

Full chain: `HEAD → refs/heads/main → commit → tree → blob`

### Exercise 3: Manually Create a Branch

```bash
# Get current commit
COMMIT=$(cat .git/refs/heads/main)

# Create a new branch manually
echo "$COMMIT" > .git/refs/heads/experimental

# Verify
git branch
# * main
#   experimental

# They point to the same commit!
cat .git/refs/heads/main
cat .git/refs/heads/experimental
# Same hash
```

### Exercise 4: Inspect the Index

```bash
# Stage some files
echo "file1" > file1.txt
echo "file2" > file2.txt
git add file1.txt file2.txt

# View index
git ls-files --stage
# 100644 hash1 0  file1.txt
# 100644 hash2 0  file2.txt

# Modify but don't stage
echo "modified" > file1.txt

# Index still shows old version
git ls-files --stage
# Still shows original hash for file1.txt

# Stage the change
git add file1.txt

# Now index updated
git ls-files --stage
# Shows new hash for file1.txt
```

## 🎨 Visualizing Git's Structure

Here's how everything connects:

```
┌──────────────────────────────────────────────────────┐
│ Working Directory                                    │
│  file1.txt  file2.txt  src/main.py                  │
└──────────────────────────────────────────────────────┘
                         │
                    (git add)
                         ↓
┌──────────────────────────────────────────────────────┐
│ Index (.git/index)                                   │
│  file1.txt → blob hash1                             │
│  file2.txt → blob hash2                             │
│  src/main.py → blob hash3                           │
└──────────────────────────────────────────────────────┘
                         │
                   (git commit)
                         ↓
┌──────────────────────────────────────────────────────┐
│ Repository (.git/objects/ + .git/refs/)              │
│                                                      │
│  .git/refs/heads/main → commit abc123               │
│                              ↓                       │
│  .git/objects/ab/c123... (commit object)            │
│      tree: def456                                   │
│                              ↓                       │
│  .git/objects/de/f456... (tree object)              │
│      100644 file1.txt → blob hash1                  │
│      100644 file2.txt → blob hash2                  │
│      040000 src → tree hash4                        │
│                                                      │
│  .git/objects/... (blob objects for files)          │
└──────────────────────────────────────────────────────┘
                         ↑
                         │
┌──────────────────────────────────────────────────────┐
│ HEAD (.git/HEAD)                                     │
│  ref: refs/heads/main                                │
└──────────────────────────────────────────────────────┘
```

## 🎓 Key Takeaways

1. **`.git/` is just files and directories** - nothing magical!
2. **HEAD** points to current branch (or commit if detached)
3. **refs/** stores branches and tags as files containing hashes
4. **objects/** stores all content (blobs, trees, commits, tags)
5. **index** is the staging area (binary file)
6. **logs/** powers reflog (history of ref changes)
7. **Branches are just files** containing commit hashes

## ✅ Checkpoint Quiz

1. What does the HEAD file contain when you're on the main branch?
2. Where does Git store the commit hash for the `feature` branch?
3. How can you create a branch without using `git branch`?
4. What's the difference between loose objects and pack files?
5. Where is the staging area stored?

<details>
<summary>Click to see answers</summary>

1. `ref: refs/heads/main`
2. `.git/refs/heads/feature`
3. Create a file in `.git/refs/heads/` with a commit hash
4. Loose objects are individual files; pack files compress multiple objects together
5. `.git/index` (binary file)
</details>

## 🔜 Next Steps

Now you can navigate `.git/` with confidence! Next, we'll dive deeper into how Git stores and compresses data.

**→ [Lesson 05: How Git Stores Data](05-how-git-stores-data.md)**

---

**Phase**: 2 - Git Internals
**Lesson**: 04 of 20
