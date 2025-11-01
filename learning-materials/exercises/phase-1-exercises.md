# Phase 1 Exercises: Git Foundations

Complete these exercises after finishing Phase 1 lessons to reinforce your understanding.

## Exercise 1: Manual Hash Computation

**Goal**: Understand how Git computes hashes

```bash
# Create a test file
echo -n "test content" > test.txt

# Question: What hash will Git produce for this file?
# Compute it manually before checking with git!

# Your calculation:
# Content: "test content"
# Length: 12 bytes
# Format: "blob 12\0test content"
# SHA-1: ???

# Check your answer
git hash-object test.txt
```

**Challenge**: Write a bash/python script to compute Git blob hashes

## Exercise 2: Object Deduplication

**Goal**: Prove Git deduplicates identical content

```bash
mkdir dedup-test && cd dedup-test
git init

# Create 3 files with same content
echo "duplicate" > file1.txt
echo "duplicate" > file2.txt
echo "duplicate" > file3.txt

# Add them all
git add .

# Question 1: How many blob objects were created?
find .git/objects -type f | grep -v pack | wc -l

# Question 2: What is the hash of the blob?
git ls-files --stage

# Question 3: Why does Git store only one copy?
```

## Exercise 3: Explore Object Types

**Goal**: Create and examine each object type

```bash
mkdir obj-types && cd obj-types
git init

# Create a blob
echo "content" > file.txt
git add file.txt
BLOB=$(git ls-files --stage | cut -d' ' -f2)

# Create a tree
git write-tree
TREE=$(git write-tree)

# Create a commit
COMMIT=$(git commit-tree $TREE -m "Test commit")

# Examine each type
git cat-file -t $BLOB   # Should be: blob
git cat-file -t $TREE   # Should be: tree
git cat-file -t $COMMIT # Should be: commit

# View contents
git cat-file -p $BLOB
git cat-file -p $TREE
git cat-file -p $COMMIT
```

**Questions**:
1. What does the tree contain?
2. What metadata is in the commit?
3. How does the commit reference the tree?

## Exercise 4: Content-Addressable Detective

**Goal**: Track down objects in the database

```bash
mkdir detective && cd detective
git init

# Store an object
echo "Secret message" | git hash-object -w --stdin

# The hash is printed. Now find the file!
# 1. What is the full hash?
# 2. Where is the file stored?
# 3. What is the file size?
# 4. Can you read it directly? (hint: it's compressed)

# Try to decompress it (advanced)
# Use: python3 -c "import zlib; import sys; print(zlib.decompress(sys.stdin.buffer.read()).decode())" < [path]
```

## Exercise 5: Avalanche Effect Observation

**Goal**: See how small changes affect hashes

```bash
# Hash original
echo "version 1" | git hash-object --stdin

# Change one character
echo "version 2" | git hash-object --stdin

# Change capitalization
echo "VERSION 1" | git hash-object --stdin

# Add a space
echo "version 1 " | git hash-object --stdin
```

**Questions**:
1. How different are the hashes?
2. Can you predict the hash from seeing the change?
3. Why is this property important for Git?

## Exercise 6: Build a Commit Chain

**Goal**: Manually create a commit history

```bash
mkdir chain && cd chain
git init

# Create first commit
echo "v1" > file.txt
git add file.txt
TREE1=$(git write-tree)
COMMIT1=$(git commit-tree $TREE1 -m "First commit")

# Create second commit (child of first)
echo "v2" > file.txt
git add file.txt
TREE2=$(git write-tree)
COMMIT2=$(git commit-tree $TREE2 -p $COMMIT1 -m "Second commit")

# Point main to second commit
echo $COMMIT2 > .git/refs/heads/main
git checkout main

# View history
git log
```

**Questions**:
1. How does the second commit reference the first?
2. What happens if you point main to COMMIT1?
3. Can you create a third commit in the chain?

## Exercise 7: Object Inspection Challenge

**Goal**: Reverse-engineer a repository

```bash
# Clone a small repo
git clone https://github.com/torvalds/linux --depth=1 --single-branch linux-shallow
cd linux-shallow

# Find the latest commit
COMMIT=$(cat .git/refs/heads/master)

# Questions to answer:
# 1. What is the commit message?
# 2. Who is the author?
# 3. What is the root tree hash?
# 4. How many parent commits does it have?
# 5. List 5 files in the root tree

# Use only git cat-file and related plumbing commands!
```

## Solutions

<details>
<summary>Exercise 1 Solution</summary>

```bash
# Compute manually
echo -en "blob 12\0test content" | sha1sum
# Output: d670460b4b4aece5915caf5c68d12f560a9fe3e4

# Verify
echo -n "test content" | git hash-object --stdin
# Same hash!
```

</details>

<details>
<summary>Exercise 2 Solution</summary>

```bash
# Answer 1: Only 1 blob object was created
# Answer 2: All three files have the same hash
# Answer 3: Git stores content by its hash; same content = same hash = same storage
```

</details>

<details>
<summary>Exercise 6 Solution</summary>

```bash
# View parent relationship
git cat-file -p $COMMIT2
# Shows: parent <hash of COMMIT1>

# Point to COMMIT1
echo $COMMIT1 > .git/refs/heads/main
git log
# Shows only first commit

# Create third commit
echo "v3" > file.txt
git add file.txt
TREE3=$(git write-tree)
COMMIT3=$(git commit-tree $TREE3 -p $COMMIT2 -m "Third commit")
```

</details>

## Bonus Challenges

### Challenge 1: Implement hash-object in Python

```python
import hashlib

def git_hash_object(content):
    # Your code here
    pass

# Test it
assert git_hash_object(b"Hello World") == "557db03de997c86a4a028e1ebd3a1ceb225be238"
```

### Challenge 2: Visualize a Repository

Write a script to:
1. Read all objects in `.git/objects/`
2. Determine their types
3. Build a graph showing relationships
4. Output as ASCII art or DOT format

### Challenge 3: Object Database Stats

Write a script to analyze a repository:
- Total number of objects
- Breakdown by type (blob, tree, commit, tag)
- Total uncompressed size
- Total compressed size
- Compression ratio

---

**Completion Checklist**:
- [ ] Exercise 1: Manual Hash Computation
- [ ] Exercise 2: Object Deduplication
- [ ] Exercise 3: Explore Object Types
- [ ] Exercise 4: Content-Addressable Detective
- [ ] Exercise 5: Avalanche Effect
- [ ] Exercise 6: Build a Commit Chain
- [ ] Exercise 7: Object Inspection Challenge
- [ ] Bonus Challenge 1 (optional)
- [ ] Bonus Challenge 2 (optional)
- [ ] Bonus Challenge 3 (optional)

Once complete, move on to **Phase 2: Git Internals**!
