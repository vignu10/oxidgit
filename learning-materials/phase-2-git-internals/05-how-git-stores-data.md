# Lesson 05: How Git Stores Data

**Estimated Time**: 1.5 hours
**Prerequisites**: Lessons 01-04

## 🎯 Learning Objectives

- Understand zlib compression in detail
- Know when Git creates pack files
- Learn about delta compression
- Understand loose vs packed objects

## 🗜️ Zlib Compression

Git uses zlib to compress all objects before storage:

```
Original: "blob 11\0Hello World"  (16 bytes)
Compressed: x\x9CK\xCA\xC9OR04\xE4\... (smaller)
```

### Why Compression?

1. **Save disk space**: Text compresses well (often 50-80% reduction)
2. **Faster network transfer**: Smaller = faster push/pull
3. **More efficient**: Git can store more history in less space

### Hands-On: See Compression

```bash
# Create large file
python3 -c "print('a' * 100000)" > large.txt

# Check size
ls -lh large.txt
# 100K

# Add to Git
git add large.txt
git commit -m "Add large file"

# Find the blob
HASH=$(git rev-parse HEAD:large.txt)

# Check compressed size
ls -lh .git/objects/${HASH:0:2}/${HASH:2}
# Much smaller! Maybe 1-2K
```

## 📦 Pack Files

When you have many objects, Git packs them into compressed archives:

```
Loose Objects (inefficient):
.git/objects/aa/bbcc...  (v1 of file.txt - 10KB compressed)
.git/objects/dd/eeff...  (v2 of file.txt - 10KB compressed)
.git/objects/gg/hhii...  (v3 of file.txt - 10KB compressed)
Total: 30KB for 3 versions

Pack File (efficient):
.git/objects/pack/pack-123.pack
  - Full v3 (10KB)
  - Delta v2 → v3 (500 bytes)
  - Delta v1 → v2 (300 bytes)
Total: ~11KB for 3 versions!
```

### Delta Compression

Git stores:
- One full version (usually the newest)
- Deltas (differences) for others

```
v1: "Hello World"
v2: "Hello World!" (added !)
v3: "Hello World!!\n" (added ! and newline)

Packed:
v3: Full content
v2: Delta from v3 (remove one !)
v1: Delta from v2 (remove !)
```

### When Does Git Pack?

- `git gc` (garbage collection)
- `git push` (to transfer efficiently)
- Automatically when too many loose objects (>6700)

### Hands-On: Pack Objects

```bash
# Create many objects
for i in {1..100}; do
  echo "Version $i" > file.txt
  git add file.txt
  git commit -m "Version $i"
done

# Count loose objects
find .git/objects -type f | grep -v pack | wc -l
# Lots!

# Pack them
git gc

# Count again
find .git/objects -type f | grep -v pack | wc -l
# Much fewer!

# See pack files
ls .git/objects/pack/
# pack-abc123.pack
# pack-abc123.idx
```

## 🎓 Key Takeaways

1. **All objects are zlib compressed**
2. **Pack files use delta compression** for efficiency
3. **Git automatically packs** during gc, push, or when too many loose objects
4. **Packing is transparent** - you can still access objects normally

## 🔜 Next Lesson

**→ [Lesson 06: Refs and HEAD](06-refs-and-head.md)**

---

**Phase**: 2 - Git Internals
**Lesson**: 05 of 20
