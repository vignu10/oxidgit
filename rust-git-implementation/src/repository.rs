//! Repository structure and operations

use anyhow::{Context, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Represents an oxid/Git repository
///
/// A repository consists of:
/// - `.git/` directory containing all Git data
/// - Working tree with project files
pub struct Repository {
    /// Path to the .git directory
    pub git_dir: PathBuf,
    /// Path to the working directory
    pub work_tree: PathBuf,
}

impl Repository {
    /// Open an existing repository
    ///
    /// Searches for a .git directory starting from the given path
    /// and walking up the directory tree.
    ///
    /// # Arguments
    ///
    /// * `path` - Starting path to search from
    ///
    /// # Errors
    ///
    /// Returns an error if no repository is found
    pub fn new(path: impl AsRef<Path>) -> Result<Self> {
        let mut current = path.as_ref().canonicalize()?;

        // Walk up directory tree looking for .git
        loop {
            let git_dir = current.join(".git");
            if git_dir.exists() && git_dir.is_dir() {
                return Ok(Repository {
                    work_tree: current,
                    git_dir,
                });
            }

            // Move to parent directory
            if !current.pop() {
                anyhow::bail!(
                    "Not an oxid repository (or any parent): {}",
                    path.as_ref().display()
                );
            }
        }
    }

    /// Initialize a new repository
    ///
    /// Creates a .git directory structure with all necessary files and folders.
    ///
    /// # Arguments
    ///
    /// * `path` - Path where the repository should be initialized
    ///
    /// # Example
    ///
    /// ```no_run
    /// use oxid::Repository;
    ///
    /// let repo = Repository::init("my-project").unwrap();
    /// ```
    pub fn init(path: impl AsRef<Path>) -> Result<Self> {
        let work_tree = path.as_ref();
        let git_dir = work_tree.join(".git");

        // Create directory structure
        fs::create_dir_all(&git_dir)?;
        fs::create_dir_all(git_dir.join("objects"))?;
        fs::create_dir_all(git_dir.join("objects/info"))?;
        fs::create_dir_all(git_dir.join("objects/pack"))?;
        fs::create_dir_all(git_dir.join("refs/heads"))?;
        fs::create_dir_all(git_dir.join("refs/tags"))?;

        // Create HEAD pointing to main branch
        fs::write(git_dir.join("HEAD"), b"ref: refs/heads/main\n")?;

        // Create config file
        let config = r#"[core]
	repositoryformatversion = 0
	filemode = false
	bare = false
"#;
        fs::write(git_dir.join("config"), config)?;

        // Create description file
        fs::write(
            git_dir.join("description"),
            b"Unnamed oxid repository.\n",
        )?;

        Ok(Repository {
            git_dir: git_dir.canonicalize()?,
            work_tree: work_tree.to_path_buf(),
        })
    }

    /// Get the path to an object file for a given hash
    ///
    /// Objects are stored as `.git/objects/XX/YYYYYYYY...`
    /// where XX is the first 2 characters of the hash.
    ///
    /// # Arguments
    ///
    /// * `hash` - The object hash (40 character hex string)
    pub fn object_path(&self, hash: &str) -> PathBuf {
        let (dir, file) = hash.split_at(2);
        self.git_dir.join("objects").join(dir).join(file)
    }

    /// Read an object from the object database
    ///
    /// # Arguments
    ///
    /// * `hash` - The object hash to read
    ///
    /// # Returns
    ///
    /// Decompressed object data
    pub fn read_object(&self, hash: &str) -> Result<Vec<u8>> {
        let path = self.object_path(hash);
        let compressed = fs::read(&path)
            .with_context(|| format!("Failed to read object {}", hash))?;
        crate::utils::decompress(&compressed)
    }

    /// Write an object to the object database
    ///
    /// # Arguments
    ///
    /// * `hash` - The object hash (determines storage location)
    /// * `data` - The object data to write (will be compressed)
    pub fn write_object(&self, hash: &str, data: &[u8]) -> Result<()> {
        let path = self.object_path(hash);

        // Create parent directory if it doesn't exist
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Compress and write
        let compressed = crate::utils::compress(data)?;
        fs::write(&path, compressed)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init_creates_structure() {
        let dir = tempdir().unwrap();
        let repo = Repository::init(dir.path()).unwrap();

        // Verify all directories exist
        assert!(repo.git_dir.join("objects").exists());
        assert!(repo.git_dir.join("refs/heads").exists());
        assert!(repo.git_dir.join("refs/tags").exists());
        assert!(repo.git_dir.join("HEAD").exists());
        assert!(repo.git_dir.join("config").exists());
    }

    #[test]
    fn test_object_path() {
        let dir = tempdir().unwrap();
        let repo = Repository::init(dir.path()).unwrap();

        let hash = "557db03de997c86a4a028e1ebd3a1ceb225be238";
        let path = repo.object_path(hash);

        let expected = repo
            .git_dir
            .join("objects")
            .join("55")
            .join("7db03de997c86a4a028e1ebd3a1ceb225be238");

        assert_eq!(path, expected);
    }

    #[test]
    fn test_write_and_read_object() {
        let dir = tempdir().unwrap();
        let repo = Repository::init(dir.path()).unwrap();

        let hash = "557db03de997c86a4a028e1ebd3a1ceb225be238";
        let data = b"blob 11\0Hello World";

        // Write object
        repo.write_object(hash, data).unwrap();

        // Read it back
        let read_data = repo.read_object(hash).unwrap();

        assert_eq!(data.to_vec(), read_data);
    }

    #[test]
    fn test_new_finds_repository() {
        let dir = tempdir().unwrap();
        Repository::init(dir.path()).unwrap();

        // Should find repo from subdirectory
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();

        let repo = Repository::new(&subdir).unwrap();
        assert_eq!(repo.work_tree, dir.path().canonicalize().unwrap());
    }
}
