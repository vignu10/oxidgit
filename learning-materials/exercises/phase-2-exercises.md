# Phase 2 Exercises: Git Internals

**Prerequisites**: Complete Phase 2 (Lessons 04-07)

These hands-on exercises will deepen your understanding of Git's internal structures.

## 🎯 Exercise Set 1: Object Exploration

### Exercise 1.1: Manual Object Creation

Create a Git blob manually without using any Git commands (except `git init`).

**Steps**:
1. Create a new Git repository
2. Calculate the SHA-1 hash of "Hello, Git!"
3. Create the blob object format: `blob [size]\0[content]`
4. Compress the data using zlib
5. Write to the correct path in `.git/objects/`
6. Verify with `git cat-file -p [hash]`

**Expected Hash**: `2d832d9044c698081e59c322d5a2a459da546469`

<details>
<summary>Hint 1</summary>

The blob format is: `blob 11\0Hello, Git!` (11 is the length of "Hello, Git!")
</details>

<details>
<summary>Hint 2</summary>

Use Python for testing:
```python
import hashlib
import zlib

content = b"Hello, Git!"
header = f"blob {len(content)}\0".encode()
data = header + content

hash_obj = hashlib.sha1(data)
print(hash_obj.hexdigest())

compressed = zlib.compress(data)
# Write to .git/objects/2d/832d9044c698081e59c322d5a2a459da546469
```
</details>

### Exercise 1.2: Tree Object Analysis

In an existing Git repository:
1. Find a tree object
2. Decompress and parse it manually
3. List all entries with their modes, names, and hashes
4. Identify which entries are files vs subdirectories

**Deliverable**: A script or manual notes showing the tree structure

### Exercise 1.3: Commit Chain Walking

Write a script that:
1. Starts at HEAD
2. Reads each commit
3. Follows the parent pointer(s)
4. Prints the commit history
5. Stops at the first commit (no parents)

**Challenge**: Handle merge commits (multiple parents)

## 🎯 Exercise Set 2: References and HEAD

### Exercise 2.1: Branch Manipulation

Without using `git branch` or `git checkout`:
1. Create a new branch called `manual-branch`
2. Point it to the current HEAD commit
3. Update HEAD to point to this branch
4. Verify with `git status`

<details>
<summary>Solution</summary>

```bash
# Get current commit hash
HASH=$(cat .git/HEAD | sed 's/ref: //' | xargs cat)

# Create branch
echo "$HASH" > .git/refs/heads/manual-branch

# Update HEAD
echo "ref: refs/heads/manual-branch" > .git/HEAD

# Verify
git status
```
</details>

### Exercise 2.2: Detached HEAD Exploration

1. Put your repository in detached HEAD state
2. Make a commit
3. Observe where the commit goes (hint: nowhere!)
4. Save the commit hash
5. Return to a branch
6. Recover your detached commit by creating a branch at that hash

### Exercise 2.3: Tag Investigation

1. Create a lightweight tag
2. Create an annotated tag
3. Compare their storage in `.git/refs/tags/`
4. For the annotated tag, read the tag object
5. Document the differences

## 🎯 Exercise Set 3: The Index

### Exercise 3.1: Index State Tracking

Track the index through this workflow:

1. **Initial**: Start with a clean working directory
   - Run: `git ls-files --stage`
   - What's in the index?

2. **Modify**: Change a tracked file
   - Run: `git ls-files --stage`
   - Did the index change?

3. **Stage**: Run `git add`
   - Run: `git ls-files --stage`
   - What changed?

4. **Commit**: Run `git commit`
   - Run: `git ls-files --stage`
   - What's the index state now?

Document your findings at each step.

### Exercise 3.2: Staging Partial Changes

1. Modify a file in two different places
2. Stage only one change using `git add -p`
3. Use `git diff` and `git diff --cached` to see the difference
4. Explain what's in each of the three trees (Working Directory, Index, HEAD)

### Exercise 3.3: Merge Conflict Index

1. Create a merge conflict
2. While in conflict state, run: `git ls-files --stage`
3. Observe the three stages (1, 2, 3)
4. Manually resolve by editing `.git/index` (advanced!)

## 🎯 Exercise Set 4: Storage and Compression

### Exercise 4.1: Compression Analysis

1. Create a large text file (1MB+)
2. Add and commit it
3. Find the blob object
4. Compare the original file size with the compressed object size
5. Calculate the compression ratio

### Exercise 4.2: Pack File Investigation

1. Create a repository with 100+ commits
2. Run `git gc`
3. Explore `.git/objects/pack/`
4. Use `git verify-pack -v` to see what's in the pack
5. Identify delta chains

### Exercise 4.3: Object Type Statistics

Write a script that:
1. Scans all objects in `.git/objects/`
2. Counts objects by type (blob, tree, commit, tag)
3. Calculates total compressed size
4. Reports statistics

**Example Output**:
```
Object Statistics:
  Blobs:   156 (2.3 MB)
  Trees:    89 (456 KB)
  Commits:  45 (123 KB)
  Tags:      3 (12 KB)
  Total:   293 (2.9 MB)
```

## 🎯 Exercise Set 5: Advanced Exploration

### Exercise 5.1: Build a Cat-File Clone

Write a script that implements `git cat-file -p`:

```bash
./my-cat-file.py <object-hash>
```

Requirements:
- Read object from `.git/objects/`
- Decompress with zlib
- Parse header
- Display content based on type
- Format trees and commits nicely

### Exercise 5.2: Build a Hash-Object Clone

Write a script that implements `git hash-object -w`:

```bash
./my-hash-object.py --write <file>
```

Requirements:
- Read file content
- Create blob format
- Compute SHA-1 hash
- Compress with zlib
- Write to `.git/objects/`
- Print hash

### Exercise 5.3: Visualize Object Graph

Create a visualization of your repository's object graph:

1. Start from HEAD
2. Follow all references (commits, trees, blobs)
3. Create a graph showing relationships
4. Use a tool like Graphviz or ASCII art

**Example ASCII Output**:
```
HEAD -> main -> commit abc123
                   |
                   +-- tree def456
                   |     +-- blob 111111 (README.md)
                   |     +-- blob 222222 (main.rs)
                   |
                   +-- parent commit 789abc
```

## 🎯 Challenge Exercises

### Challenge 1: Repository Forensics

Given a `.git` directory, determine:
1. When was the repository created? (hint: first commit)
2. How many unique authors have committed?
3. What's the largest file ever committed?
4. What's the most frequently modified file?
5. What's the longest commit message?

### Challenge 2: Garbage Collection Simulator

Write a script that identifies unreachable objects:
1. Start from all refs (branches, tags, HEAD)
2. Mark all reachable objects
3. Find unmarked objects in `.git/objects/`
4. Report what could be garbage collected

### Challenge 3: Repository Integrity Check

Build a repository integrity checker that:
1. Verifies all object hashes match their content
2. Checks that all referenced objects exist
3. Validates object format (headers, etc.)
4. Reports any corruption

## 📊 Project: Git Internals Explorer

Build a complete Git internals exploration tool with these features:

### Core Features:
- List all objects by type
- Display object content
- Follow references
- Show object dependencies
- Visualize commit history

### CLI Interface:
```bash
git-explorer objects              # List all objects
git-explorer show <hash>          # Show object content
git-explorer refs                 # List all refs
git-explorer history              # Show commit history
git-explorer stats                # Repository statistics
git-explorer verify               # Check integrity
```

### Bonus Features:
- Compare two commits
- Find large objects
- Analyze repository growth over time
- Export object graph as DOT file

## ✅ Completion Checklist

Mark exercises as you complete them:

**Object Exploration**
- [ ] Exercise 1.1: Manual Object Creation
- [ ] Exercise 1.2: Tree Object Analysis
- [ ] Exercise 1.3: Commit Chain Walking

**References and HEAD**
- [ ] Exercise 2.1: Branch Manipulation
- [ ] Exercise 2.2: Detached HEAD Exploration
- [ ] Exercise 2.3: Tag Investigation

**The Index**
- [ ] Exercise 3.1: Index State Tracking
- [ ] Exercise 3.2: Staging Partial Changes
- [ ] Exercise 3.3: Merge Conflict Index

**Storage and Compression**
- [ ] Exercise 4.1: Compression Analysis
- [ ] Exercise 4.2: Pack File Investigation
- [ ] Exercise 4.3: Object Type Statistics

**Advanced Exploration**
- [ ] Exercise 5.1: Build a Cat-File Clone
- [ ] Exercise 5.2: Build a Hash-Object Clone
- [ ] Exercise 5.3: Visualize Object Graph

**Challenges**
- [ ] Challenge 1: Repository Forensics
- [ ] Challenge 2: Garbage Collection Simulator
- [ ] Challenge 3: Repository Integrity Check

**Project**
- [ ] Git Internals Explorer

## 🎓 Learning Outcomes

After completing these exercises, you should be able to:

✅ Manually create and manipulate Git objects
✅ Navigate the `.git` directory confidently
✅ Understand how Git stores and retrieves data
✅ Debug Git issues at the object level
✅ Explain Git internals to others

## 🔜 Next Steps

Once you've completed Phase 2 exercises, move on to:

**→ [Phase 3: Rust Fundamentals](../phase-3-rust-fundamentals/08-rust-basics-for-git.md)**

---

**Phase**: 2 - Git Internals
**Difficulty**: Intermediate to Advanced
**Estimated Time**: 8-12 hours
