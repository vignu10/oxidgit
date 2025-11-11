# Lesson 06: Git References and HEAD

**Estimated Time**: 1.5-2 hours
**Prerequisites**: Lessons 01-05

## 🎯 Learning Objectives

By the end of this lesson, you will:

1. Understand what Git references (refs) are
2. Know how branches work internally
3. Understand HEAD and its role
4. Learn about tags and remote refs
5. Explore symbolic references

## 🔗 What Are References?

A **reference** (or "ref") is a simple name that points to a commit hash.

Instead of remembering:
```
commit 7f8a9b2c1d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a
```

You can use:
```
main
```

### Why Refs?

1. **Human-readable**: `main` vs `7f8a9b2c1d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a`
2. **Mutable**: Refs can be updated to point to different commits
3. **Organized**: Refs are stored in a hierarchy

## 📂 Where Refs Live

All refs are stored in `.git/refs/`:

```
.git/refs/
├── heads/          # Local branches
│   ├── main
│   ├── feature-x
│   └── bugfix-y
├── tags/           # Tags
│   ├── v1.0.0
│   └── v2.0.0
└── remotes/        # Remote-tracking branches
    └── origin/
        ├── main
        └── develop
```

### Hands-On: Explore Refs

```bash
# Create a test repo
mkdir git-refs-test
cd git-refs-test
git init

# Make first commit
echo "Hello" > file.txt
git add file.txt
git commit -m "First commit"

# Look at the refs directory
tree .git/refs
# .git/refs/
# ├── heads/
# │   └── main
# └── tags/

# Read the main ref
cat .git/refs/heads/main
# 7f8a9b2c1d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a

# Verify it points to your commit
git log --oneline
# 7f8a9b2 First commit
```

## 🌿 Branches Are Just Refs

A **branch** is simply a ref that points to a commit.

### Creating a Branch

```bash
# Create a new branch
git branch feature

# It's just a new file!
cat .git/refs/heads/feature
# Same hash as main!

# Verify
ls .git/refs/heads/
# main
# feature
```

### What Happens When You Commit?

```bash
# Switch to feature branch
git checkout feature

# Make a commit
echo "Feature work" >> file.txt
git add file.txt
git commit -m "Add feature"

# The feature ref updated!
cat .git/refs/heads/feature
# 1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b (new hash)

# But main didn't change
cat .git/refs/heads/main
# 7f8a9b2c1d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a (old hash)
```

**Key insight**: Committing updates the current branch's ref to point to the new commit!

## 🎯 HEAD: The Special Reference

**HEAD** is a special ref that points to your current position.

### Direct vs Symbolic References

HEAD can work in two modes:

#### 1. Symbolic Reference (Normal Mode)

```bash
# Read HEAD
cat .git/HEAD
# ref: refs/heads/main

# HEAD points to a branch, not a commit!
```

This means:
- HEAD → refs/heads/main → commit hash
- When you commit, main updates, HEAD follows

#### 2. Detached HEAD (Special Mode)

```bash
# Checkout a specific commit
git checkout 7f8a9b2

# Read HEAD
cat .git/HEAD
# 7f8a9b2c1d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a

# HEAD points directly to a commit!
```

This means:
- HEAD → commit hash (no branch)
- When you commit, the new commit isn't on any branch
- Git will warn you: "detached HEAD state"

### Hands-On: HEAD Exploration

```bash
# Normal HEAD
git checkout main
cat .git/HEAD
# ref: refs/heads/main

# Switch branches
git checkout feature
cat .git/HEAD
# ref: refs/heads/feature

# Detach HEAD
git checkout HEAD~1
cat .git/HEAD
# [some commit hash]

git status
# HEAD detached at [hash]

# Go back to a branch
git checkout main
```

## 🏷️ Tags

Tags are like branches, but they **don't move**.

### Two Types of Tags

#### 1. Lightweight Tags

Just a ref that points to a commit:

```bash
# Create lightweight tag
git tag v1.0

# It's just a file with a hash
cat .git/refs/tags/v1.0
# 7f8a9b2c1d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a
```

#### 2. Annotated Tags

A full Git object with metadata:

```bash
# Create annotated tag
git tag -a v2.0 -m "Version 2.0"

# It's a tag object!
cat .git/refs/tags/v2.0
# 9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b

# Read the tag object
git cat-file -p v2.0
# object 7f8a9b2c1d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a
# type commit
# tag v2.0
# tagger John Doe <john@example.com> 1234567890 -0700
#
# Version 2.0
```

### Lightweight vs Annotated

| Feature | Lightweight | Annotated |
|---------|------------|-----------|
| Storage | Just a ref | Full object |
| Metadata | No | Yes (tagger, date, message) |
| Use case | Temporary markers | Releases, milestones |
| Size | Tiny | Small |

## 🌐 Remote References

When you work with remotes, Git stores remote branches:

```bash
# After cloning or fetching
tree .git/refs/remotes/
# .git/refs/remotes/
# └── origin/
#     ├── main
#     └── develop

# Read a remote ref
cat .git/refs/remotes/origin/main
# a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0

# These are read-only snapshots of remote branches
```

## 📋 Packed Refs

For efficiency, Git can pack refs into a single file:

```bash
# Pack refs
git pack-refs --all

# Look at the packed-refs file
cat .git/packed-refs
# # pack-refs with: peeled fully-peeled sorted
# 7f8a9b2c1d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a refs/heads/main
# 1a2b3c4d5e6f7a8b9c0d1e2f3a4b5c6d7e8f9a0b refs/heads/feature
# 9a0b1c2d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a8b refs/tags/v2.0
# ^7f8a9b2c1d3e4f5a6b7c8d9e0f1a2b3c4d5e6f7a

# Individual ref files may be deleted after packing
```

**When does packing happen?**
- `git gc` (garbage collection)
- Repositories with many refs
- Automatically by Git for efficiency

## 🔄 How Git Resolves References

When you use a ref name, Git looks for it in this order:

1. `.git/[name]` - Special refs like HEAD
2. `.git/refs/[name]` - Direct ref
3. `.git/refs/tags/[name]` - Tags
4. `.git/refs/heads/[name]` - Branches
5. `.git/refs/remotes/[name]` - Remote branches
6. `.git/refs/remotes/[name]/HEAD` - Remote default branch

```bash
# All these work!
git show main                    # Branch
git show refs/heads/main         # Full path
git show v1.0                    # Tag
git show origin/main             # Remote branch
```

## 🎯 Refspecs

Refspecs define how local and remote refs map:

```bash
# Look at remote config
cat .git/config
# [remote "origin"]
#     url = https://github.com/user/repo.git
#     fetch = +refs/heads/*:refs/remotes/origin/*
```

**Breaking down** `+refs/heads/*:refs/remotes/origin/*`:
- `+` = Force update (allow non-fast-forward)
- `refs/heads/*` = All remote branches
- `:` = Mapping
- `refs/remotes/origin/*` = Store as remote-tracking branches

## 🧪 Hands-On: Complete Workflow

```bash
# Create test repo
mkdir refs-workflow
cd refs-workflow
git init

# First commit
echo "v1" > file.txt
git add file.txt
git commit -m "v1"

# Check state
cat .git/HEAD
# ref: refs/heads/main

cat .git/refs/heads/main
# [hash1]

# Create branch
git branch develop
cat .git/refs/heads/develop
# [hash1] - same as main

# Make commit on develop
git checkout develop
echo "v2" > file.txt
git commit -am "v2"

cat .git/refs/heads/develop
# [hash2] - updated!

cat .git/refs/heads/main
# [hash1] - unchanged

# Create tag
git tag v1.0 main
cat .git/refs/tags/v1.0
# [hash1]

# Detach HEAD
git checkout HEAD~1
cat .git/HEAD
# [hash1] - direct commit reference

# Return to branch
git checkout develop
cat .git/HEAD
# ref: refs/heads/develop
```

## 🎓 Key Takeaways

1. **Refs are pointers** to commits, stored as files with commit hashes
2. **Branches are refs** in `.git/refs/heads/` that move with new commits
3. **HEAD is special** - usually points to a branch, occasionally to a commit (detached)
4. **Tags are immutable refs** - they don't move like branches
5. **Remote refs** are read-only snapshots of remote branches
6. **Packed refs** optimize storage for many references

## 🧩 Mental Model

```
Working Directory
       ↓
   Staging Area
       ↓
   git commit
       ↓
   New Commit Object
       ↓
   Update Current Branch Ref ← HEAD points here
       ↓
   .git/refs/heads/[branch] updated
```

## ✅ Checkpoint Quiz

1. What's the difference between a branch and a tag?
2. Where is the `main` branch stored?
3. What does `HEAD` contain normally?
4. What is "detached HEAD" state?
5. How do you create a lightweight tag vs annotated tag?

<details>
<summary>Click to see answers</summary>

1. Branches move with commits; tags stay fixed
2. `.git/refs/heads/main` (or `.git/packed-refs`)
3. `ref: refs/heads/[branch-name]` - a symbolic reference to a branch
4. When HEAD points directly to a commit instead of a branch
5. Lightweight: `git tag v1.0`, Annotated: `git tag -a v1.0 -m "message"`
</details>

## 🔜 Next Lesson

Now that you understand refs and HEAD, let's explore the staging area!

**→ [Lesson 07: The Git Index (Staging Area)](07-git-index.md)**

---

**Phase**: 2 - Git Internals
**Lesson**: 06 of 20
