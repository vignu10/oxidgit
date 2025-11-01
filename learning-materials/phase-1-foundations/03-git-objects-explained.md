# Lesson 03: Git Objects Explained

**Estimated Time**: 2-2.5 hours
**Prerequisites**: Lessons 01-02

## 🎯 Learning Objectives

By the end of this lesson, you will:

1. Understand all four Git object types in detail
2. Know how objects reference each other
3. Be able to manually create each object type
4. Understand how objects form a commit history
5. Visualize Git's internal structure

## 📦 The Four Git Object Types

Git's entire data model consists of just four object types:

```
┌─────────────────────────────────────────────────────┐
│                  GIT OBJECTS                        │
├─────────────────────────────────────────────────────┤
│                                                     │
│  1. BLOB   - File content                          │
│  2. TREE   - Directory structure                   │
│  3. COMMIT - Snapshot + metadata                   │
│  4. TAG    - Named reference                       │
│                                                     │
└─────────────────────────────────────────────────────┘
```

Each object type serves a specific purpose. Let's explore them!

## 1️⃣ Blob Objects (Binary Large Objects)

**Purpose**: Store file content

### What's in a Blob?

```
blob [size]\0[content]
```

That's it! A blob contains:

- **NO filename** - just raw content
- **NO permissions** - that's in trees
- **NO timestamps** - that's in commits
- **JUST DATA** - the file's contents

### Example

```bash
# Create a test file
echo "def hello():\n    print('Hello')" > hello.py

# Store as blob
git hash-object -w hello.py
# Output: 4cf76005e891fcf0d4bbc5765f8f73fbc7c0c18a

# Read it back
git cat-file -p 4cf7600
# Output: def hello():
#             print('Hello')

# Check its type
git cat-file -t 4cf7600
# Output: blob
```

### Key Properties

1. **Content-only**: Same content → same blob, regardless of filename

```bash
echo "data" > file1.txt
echo "data" > file2.txt
echo "data" > something_else.py

# All three create the SAME blob:
git hash-object file1.txt file2.txt something_else.py
# Same hash three times!
```

2. **Binary-safe**: Blobs can store any data (images, executables, etc.)

```bash
# Store an image
git hash-object -w photo.jpg
# Works perfectly!
```

## 2️⃣ Tree Objects

**Purpose**: Store directory structure (filenames, permissions, and references to blobs/trees)

### What's in a Tree?

```
tree [size]\0
[mode] [filename]\0[hash-as-20-bytes]
[mode] [filename]\0[hash-as-20-bytes]
...
```

Each entry contains:

- **Mode**: File permissions (e.g., `100644` for regular file)
- **Filename**: The actual filename
- **Hash**: SHA-1 of the blob or subtree

### Example Structure

```
my-project/
├── README.md          → blob abc123...
├── src/
│   ├── main.py       → blob def456...
│   └── utils.py      → blob 789abc...
└── tests/
    └── test_main.py  → blob fedcba...
```

This creates:

```
tree:root (hash: aaa111...)
├── 100644 README.md → blob abc123
├── 040000 src → tree bbb222
└── 040000 tests → tree ccc333

tree:src (hash: bbb222...)
├── 100644 main.py → blob def456
└── 100644 utils.py → blob 789abc

tree:tests (hash: ccc333...)
└── 100644 test_main.py → blob fedcba
```

### Mode Values

| Mode   | Meaning |
|--------|---------|
| 040000 | Directory (tree) |
| 100644 | Regular file (non-executable) |
| 100755 | Executable file |
| 120000 | Symbolic link |
| 160000 | Gitlink (submodule) |

### Hands-On: Create a Tree

```bash
# Create some files
mkdir /tmp/tree-test && cd /tmp/tree-test
git init

echo "# My Project" > README.md
mkdir src
echo "print('hello')" > src/main.py

# Store the blobs
git add README.md src/main.py

# Create tree object from index (staging area)
git write-tree
# Output: 2c97a8... (hash of root tree)

# Examine the tree
git cat-file -p 2c97a8
# Output:
# 100644 blob 5c91d... README.md
# 040000 tree a1b2c... src

# Examine the src tree
git cat-file -p a1b2c
# Output:
# 100644 blob d3e4f... main.py
```

### Key Properties

1. **Trees reference other trees**: Nested directories create nested trees
2. **Filenames only appear in trees**: Blobs have no filename metadata
3. **Trees are snapshots**: Capture complete directory state

## 3️⃣ Commit Objects

**Purpose**: Snapshot of project at a point in time + metadata

### What's in a Commit?

```
commit [size]\0
tree [tree-hash]
parent [parent-commit-hash]    ← Optional (not in first commit)
parent [parent-commit-hash]    ← Optional (for merge commits)
author [name] <[email]> [timestamp] [timezone]
committer [name] <[email]> [timestamp] [timezone]

[commit message]
```

### Example

```bash
# Create a commit
echo "test" > file.txt
git add file.txt
git commit -m "Initial commit"

# Find the commit hash
git log --oneline
# a1b2c3d Initial commit

# Examine the commit
git cat-file -p a1b2c3d
# Output:
# tree 4e8f9a...
# author John Doe <john@example.com> 1634567890 +0000
# committer John Doe <john@example.com> 1634567890 +0000
#
# Initial commit
```

### Commit Chains

Commits link together via parent references:

```
(oldest)                                      (newest)
commit A  ←─  commit B  ←─  commit C  ←─  commit D
(no parent)   (parent: A)   (parent: B)   (parent: C)
```

### Merge Commits

Merge commits have multiple parents:

```
     commit A
    /         \
commit B    commit C
    \         /
     commit D (merge)
     (parent: B, parent: C)
```

### Hands-On: Examine Commits

```bash
# Create a commit chain
cd /tmp/commit-test
git init

echo "v1" > file.txt
git add file.txt
git commit -m "First"

echo "v2" > file.txt
git add file.txt
git commit -m "Second"

echo "v3" > file.txt
git add file.txt
git commit -m "Third"

# See the chain
git log --oneline
# c1c1c1c Third
# b2b2b2b Second
# a3a3a3a First

# Examine the latest commit
git cat-file -p c1c1c1c
# tree ...
# parent b2b2b2b    ← Points to previous commit
# author ...
# committer ...
#
# Third

# Follow the chain
git cat-file -p b2b2b2b
# parent a3a3a3a    ← Points to earlier commit

git cat-file -p a3a3a3a
# (no parent)       ← First commit has no parent
```

### Key Properties

1. **Commits are immutable**: Once created, cannot change (change content → new hash → new commit)
2. **History is a graph**: Commits link via parent pointers
3. **Each commit is a complete snapshot**: Points to entire tree at that moment

## 4️⃣ Tag Objects

**Purpose**: Named reference to a commit (usually for releases)

### Lightweight Tags

Just a file containing a commit hash:

```bash
git tag v1.0.0
# Creates .git/refs/tags/v1.0.0 containing commit hash
```

### Annotated Tags

Full Git objects with metadata:

```
tag [size]\0
object [commit-hash]
type commit
tag [tag-name]
tagger [name] <[email]> [timestamp] [timezone]

[tag message]
```

### Example

```bash
# Create annotated tag
git tag -a v1.0.0 -m "First release"

# Find tag object
git show v1.0.0
# tag v1.0.0
# Tagger: ...
# Date: ...
#
# First release
#
# commit a1b2c3d...
```

### Key Properties

1. **Annotated tags are signed**: Can use GPG signatures for verification
2. **Tags are permanent markers**: Point to specific commits
3. **Lightweight tags are just refs**: Not objects, just pointers

## 🔗 How Objects Connect

Let's visualize a complete repository structure:

```
                    ┌──────────────┐
                    │   COMMIT     │
                    │  hash: c1c1  │
                    │ "Add feature"│
                    └──────┬───────┘
                           │ tree
                           ↓
                    ┌──────────────┐
                    │    TREE      │
                    │  hash: t1t1  │ (root directory)
                    └──┬─────────┬─┘
           ┌───────────┘         └────────────┐
           │ README.md                      src/
           ↓                                  ↓
    ┌────────────┐                    ┌────────────┐
    │    BLOB    │                    │    TREE    │
    │ hash: b1b1 │                    │ hash: t2t2 │
    │"# Project" │                    └─────┬──────┘
    └────────────┘                          │ main.py
                                            ↓
                                     ┌────────────┐
                                     │    BLOB    │
                                     │ hash: b2b2 │
                                     │ "print()" │
                                     └────────────┘
```

### Object Reference Chain

```
HEAD → refs/heads/main → COMMIT → TREE → BLOB
                           ↓
                        PARENT COMMIT → TREE → BLOB
```

## 🧪 Hands-On: Build a Repository Manually

Let's create objects without porcelain commands!

```bash
# Setup
cd /tmp/manual-repo
rm -rf .git
git init

# Step 1: Create blobs
echo "Hello World" | git hash-object -w --stdin
# Output: 557db03de997c86a4a028e1ebd3a1ceb225be238

echo "print('hi')" | git hash-object -w --stdin
# Output: a1b2c3d... (some hash)

# Step 2: Create a tree manually
# Format: [mode] [type] [hash]\t[filename]
cat > /tmp/tree-input <<EOF
100644 blob 557db03de997c86a4a028e1ebd3a1ceb225be238	hello.txt
100644 blob a1b2c3d...	script.py
EOF

# Create tree from this input
git mktree < /tmp/tree-input
# Output: e5f6a7b... (tree hash)

# Step 3: Create a commit manually
git commit-tree e5f6a7b -m "Manual commit"
# Output: 8f9a0b1... (commit hash)

# Step 4: Point a branch at our commit
echo "8f9a0b1..." > .git/refs/heads/manual-branch

# Check it out!
git checkout manual-branch
git log
# Shows our manually created commit!
```

## 📊 Complete Example Visualization

Here's what happens with `git commit`:

```
Working Directory         Index               Repository
─────────────────        ───────             ──────────

hello.txt
"Hello World"

                ──git add──>

                          hello.txt
                          (staged)

                                    ──git commit──>

                                                  BLOB 557db03
                                                  "Hello World"
                                                      ↑
                                                  TREE a1b2c3
                                                  100644 hello.txt
                                                      ↑
                                                  COMMIT 8f9a0b
                                                  tree a1b2c3
                                                  "Initial commit"
```

## 🎓 Key Takeaways

1. **Blob**: Stores file content only (no filename)
2. **Tree**: Stores directory structure (filenames + permissions + refs)
3. **Commit**: Stores snapshot (tree ref + parent + metadata + message)
4. **Tag**: Stores named reference to commit
5. **Objects are immutable**: Hash changes if content changes
6. **Objects reference each other**: Commits → Trees → Blobs
7. **History is a graph**: Commits link via parent pointers

## ✅ Checkpoint Quiz

1. What information does a blob NOT contain?
2. How does a tree store subdirectories?
3. What is the parent of the first commit in a repository?
4. Can two commits point to the same tree? Why might this happen?
5. What's the difference between a lightweight and annotated tag?

<details>
<summary>Click to see answers</summary>

1. Filename, permissions, timestamps - only content!
2. As references to other tree objects (mode 040000)
3. No parent (root commit has no parent field)
4. Yes! If nothing changed between commits (empty commit or revert)
5. Lightweight = just a ref file; annotated = full object with message, tagger, signature
</details>

## 🚧 Common Misconceptions

**Misconception**: "Blobs store files"
**Reality**: Blobs store content. Trees associate that content with filenames.

**Misconception**: "Commits store changes"
**Reality**: Commits store complete snapshots. Git computes changes by comparing trees.

**Misconception**: "Deleting a file creates a deletion object"
**Reality**: New commit points to tree without that file. Old blob remains in database.

## 🔬 Advanced Exercise: Deduplicate Detection

```bash
# Create two commits with same content
cd /tmp/dedup-test
git init

echo "data" > file1.txt
git add file1.txt
git commit -m "Commit 1"

# Get the tree hash
TREE1=$(git cat-file -p HEAD | grep tree | cut -d' ' -f2)

# "Change" content to same value
echo "data" > file1.txt
git add file1.txt
git commit -m "Commit 2"

# Get new tree hash
TREE2=$(git cat-file -p HEAD | grep tree | cut -d' ' -f2)

# Compare
echo "Tree 1: $TREE1"
echo "Tree 2: $TREE2"

# They're the same! Git deduplicates automatically.
```

## 🔜 Next Steps

You now understand Git's four object types! In Phase 2, we'll explore Git's directory structure and see these objects in action.

**→ [Phase 2: Git Internals Deep Dive](../phase-2-git-internals/04-exploring-git-directory.md)**

---

**Phase**: 1 - Foundations (Complete!)
**Lesson**: 03 of 20
**Next Phase**: Git Internals Deep Dive
**Next Lesson**: Exploring the .git Directory
