# Lesson 07: The Git Index (Staging Area)

**Estimated Time**: 2-3 hours
**Prerequisites**: Lessons 01-06

## 🎯 Learning Objectives

By the end of this lesson, you will:

1. Understand what the Git index (staging area) is
2. Know how `git add` works internally
3. Learn the index file format
4. Understand the three-tree architecture
5. Explore how the index enables fast status checks

## 🎭 The Three Trees of Git

Git manages three "trees" (snapshots of files):

```
1. Working Directory    ← Your actual files
         ↓
2. Index (Staging)      ← Proposed next commit
         ↓
3. HEAD (Repository)    ← Last commit
```

### Example Workflow

```bash
# State 1: All trees match
echo "Hello" > file.txt
git add file.txt
git commit -m "Add file"

# Working Directory: file.txt = "Hello"
# Index: file.txt = "Hello"
# HEAD: file.txt = "Hello"

# State 2: Modify file
echo "World" >> file.txt

# Working Directory: file.txt = "Hello\nWorld"
# Index: file.txt = "Hello"  (unchanged)
# HEAD: file.txt = "Hello"   (unchanged)

# State 3: Stage changes
git add file.txt

# Working Directory: file.txt = "Hello\nWorld"
# Index: file.txt = "Hello\nWorld"  (updated!)
# HEAD: file.txt = "Hello"          (unchanged)

# State 4: Commit
git commit -m "Update file"

# Working Directory: file.txt = "Hello\nWorld"
# Index: file.txt = "Hello\nWorld"
# HEAD: file.txt = "Hello\nWorld"  (updated!)
```

## 📄 The Index File

The index is stored in `.git/index` - a binary file.

### What's In the Index?

For each file, the index stores:

1. **File metadata**:
   - File name and path
   - File mode (permissions)
   - File size
   - Modification time

2. **Blob hash**: The SHA-1 of the file content

3. **Stage number**: For merge conflicts (0 = normal)

### Hands-On: Explore the Index

```bash
# Create test repo
mkdir index-test
cd index-test
git init

# Check if index exists
ls .git/index
# ls: .git/index: No such file or directory
# (Index doesn't exist yet!)

# Create and add a file
echo "Hello World" > hello.txt
git add hello.txt

# Now index exists!
ls -lh .git/index
# -rw-r--r-- 1 user user 104 Nov 11 10:00 .git/index

# View index contents (human-readable)
git ls-files --stage
# 100644 557db03de997c86a4a028e1ebd3a1ceb225be238 0	hello.txt
```

**Breaking down the output**:
- `100644` = File mode (regular file, readable by all, writable by owner)
- `557db03...` = Blob hash
- `0` = Stage number (0 = normal, 1-3 for conflicts)
- `hello.txt` = File path

## 🔍 Index File Format

The index is a binary file with this structure:

```
+-------------------+
| Header            | 12 bytes
|  - Signature      | 4 bytes: 'DIRC'
|  - Version        | 4 bytes: 2, 3, or 4
|  - Entry count    | 4 bytes: number of entries
+-------------------+
| Index Entry 1     | Variable size
|  - ctime          | 8 bytes
|  - mtime          | 8 bytes
|  - dev, ino       | 8 bytes
|  - mode           | 4 bytes
|  - uid, gid       | 8 bytes
|  - file size      | 4 bytes
|  - SHA-1 hash     | 20 bytes
|  - flags          | 2 bytes
|  - path name      | Variable (null-terminated)
+-------------------+
| Index Entry 2     |
+-------------------+
| ...               |
+-------------------+
| Extensions        | Optional
+-------------------+
| Checksum          | 20 bytes (SHA-1 of index)
+-------------------+
```

### Hands-On: Parse Index Header

```bash
# View raw bytes (first 12 bytes = header)
xxd -l 12 .git/index
# 00000000: 4449 5243 0000 0002 0000 0001
# D I R C  [version 2]  [1 entry]

# In detail:
# DIRC = Signature
# 00 00 00 02 = Version 2
# 00 00 00 01 = 1 entry
```

## 🎯 How `git add` Works

When you run `git add file.txt`:

1. **Read file** from working directory
2. **Create blob object**:
   ```
   blob [size]\0[content]
   ```
3. **Hash the blob** (SHA-1)
4. **Write blob** to `.git/objects/` (compressed)
5. **Update index** with:
   - File path
   - File metadata
   - Blob hash
   - Stage number (0)

### Hands-On: Track `git add`

```bash
# Create file
echo "Test content" > test.txt

# Before add
git ls-files --stage
# (empty)

find .git/objects -type f
# (only pack files maybe)

# Add file
git add test.txt

# After add - index updated
git ls-files --stage
# 100644 d670460b4b4aece5915caf5c68d12f560a9fe3e4 0	test.txt

# After add - blob created
find .git/objects -type f | grep -v pack
# .git/objects/d6/70460b4b4aece5915caf5c68d12f560a9fe3e4

# Verify blob content
git cat-file -p d670460b4b
# Test content
```

## 🚀 Fast Status Checks

The index enables Git to check status quickly.

### How `git status` Works

```bash
# For each file in working directory:
#   1. Get current file metadata (size, mtime)
#   2. Compare with index metadata
#   3. If different → file is modified
#   4. If same → file is clean (no need to read content!)
```

This is why `git status` is fast even in huge repositories!

### Hands-On: Status Check

```bash
# Clean state
git status
# nothing to commit, working tree clean

# Modify file
echo "More content" >> test.txt

# Check status
git status
# Changes not staged for commit:
#   modified:   test.txt

# Under the hood:
# 1. Git checks test.txt metadata
# 2. Size changed → file is modified
# 3. Shows as modified (doesn't even need to read content yet!)
```

## 📊 Index States

Files can be in different states:

### State 1: Untracked

```bash
# File not in index
echo "new" > new.txt

git ls-files --stage | grep new.txt
# (no output - not in index)

git status
# Untracked files:
#   new.txt
```

### State 2: Staged (Tracked, Clean)

```bash
# File in index, working directory matches
git add new.txt

git ls-files --stage | grep new.txt
# 100644 fa49b077972391ad58037050f2a75f74e3671e92 0	new.txt

git status
# Changes to be committed:
#   new file:   new.txt
```

### State 3: Modified (Not Staged)

```bash
# File in index, working directory differs
echo "modified" > new.txt

git status
# Changes not staged for commit:
#   modified:   new.txt
```

### State 4: Modified (Staged)

```bash
# Stage the modification
git add new.txt

git status
# Changes to be committed:
#   modified:   new.txt
```

### State 5: Deleted (Not Staged)

```bash
# Delete file
rm new.txt

git status
# Changes not staged for commit:
#   deleted:    new.txt

git ls-files --stage | grep new.txt
# Still in index!
```

## 🔧 Index Commands

### Viewing Index

```bash
# Show index contents
git ls-files --stage

# Show index with full hash
git ls-files --stage --full-name --abbrev=40

# Show cached (index) version
git diff --cached
```

### Manipulating Index

```bash
# Add to index
git add file.txt

# Remove from index (keep file)
git rm --cached file.txt

# Update index from HEAD
git reset file.txt

# Clear entire index
git rm --cached -r .

# Update index with all changes
git add -A
```

## 🧩 Merge Conflicts and the Index

During a merge conflict, the index holds multiple versions:

```bash
# Stage 0: Normal (no conflict)
# Stage 1: Common ancestor version
# Stage 2: "Ours" (current branch)
# Stage 3: "Theirs" (merging branch)
```

### Example:

```bash
# Create conflict
git checkout -b branch-a
echo "Version A" > conflict.txt
git add conflict.txt
git commit -m "Version A"

git checkout main
echo "Version B" > conflict.txt
git add conflict.txt
git commit -m "Version B"

# Try to merge
git merge branch-a
# CONFLICT (content): Merge conflict in conflict.txt

# View index during conflict
git ls-files --stage
# 100644 [hash] 1	conflict.txt  ← stage 1 (ancestor)
# 100644 [hash] 2	conflict.txt  ← stage 2 (ours)
# 100644 [hash] 3	conflict.txt  ← stage 3 (theirs)

# After resolving
git add conflict.txt

git ls-files --stage
# 100644 [hash] 0	conflict.txt  ← back to stage 0
```

## 🎯 Why the Index Exists

The index provides several benefits:

### 1. Granular Commits

```bash
# Modify multiple files
echo "A" >> file1.txt
echo "B" >> file2.txt

# Stage only one
git add file1.txt

# Commit only staged changes
git commit -m "Update file1"
# file2.txt changes not committed!
```

### 2. Performance

- Fast status checks (metadata comparison)
- No need to scan entire working directory
- Cached blob hashes

### 3. Atomic Operations

- All index changes are atomic
- Consistent view of what will be committed

### 4. Conflict Resolution

- Stores multiple versions during merge
- Tracks resolution progress

## 🧪 Hands-On: Complete Index Workflow

```bash
# Setup
mkdir index-workflow
cd index-workflow
git init

# 1. Create files
echo "File 1" > file1.txt
echo "File 2" > file2.txt

# Check state
git ls-files --stage
# (empty - nothing in index)

git status
# Untracked: file1.txt, file2.txt

# 2. Add first file
git add file1.txt

git ls-files --stage
# 100644 [hash] 0	file1.txt

git status
# Changes to be committed: new file: file1.txt
# Untracked: file2.txt

# 3. Modify staged file
echo "Modified" >> file1.txt

git status
# Changes to be committed: new file: file1.txt
# Changes not staged: modified: file1.txt
# (Both states at once!)

git ls-files --stage
# 100644 [old-hash] 0	file1.txt
# (Index has old version)

# 4. Add the modification
git add file1.txt

git ls-files --stage
# 100644 [new-hash] 0	file1.txt

# 5. Commit
git commit -m "Add file1"

# 6. Index still has entry
git ls-files --stage
# 100644 [new-hash] 0	file1.txt
# (Index persists after commit!)
```

## 🎓 Key Takeaways

1. **Three trees**: Working Directory → Index → HEAD
2. **Index is binary**: Stored in `.git/index`
3. **git add creates blobs**: And updates the index
4. **Fast status**: Index enables quick metadata comparison
5. **Staging area**: Lets you craft commits precisely
6. **Merge conflicts**: Use stages 1-3 in the index
7. **Index persists**: Stays after commits for performance

## 📐 Mental Model

```
Working Directory          Index                HEAD
    (files)              (.git/index)      (last commit)

     file.txt              file.txt           file.txt
   "modified"    git add→  "modified"  commit→ "modified"
                          [blob hash]        [commit → tree → blob]
```

## ✅ Checkpoint Quiz

1. What are Git's three trees?
2. Where is the index stored?
3. What does `git add` do to the index?
4. How does Git check if a file is modified so quickly?
5. What are stages 1, 2, and 3 in the index used for?

<details>
<summary>Click to see answers</summary>

1. Working Directory, Index (Staging Area), HEAD (Repository)
2. `.git/index` (binary file)
3. Creates a blob object and adds entry to index with file metadata and blob hash
4. Compares file metadata (size, mtime) with index; only reads content if metadata differs
5. Merge conflicts: 1=common ancestor, 2=ours (current), 3=theirs (merging)
</details>

## 🔜 Next Lesson

You've completed Phase 2! You now understand Git's internal structures. Next, we'll learn Rust to implement our own Git!

**→ [Lesson 08: Rust Basics for Git](../phase-3-rust-fundamentals/08-rust-basics-for-git.md)**

---

**Phase**: 2 - Git Internals
**Lesson**: 07 of 20
