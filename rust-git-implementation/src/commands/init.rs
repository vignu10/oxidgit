//! Repository initialization command

use crate::repository::Repository;
use anyhow::Result;

/// Initialize a new oxid repository
///
/// Creates a .git directory structure with all necessary subdirectories and files.
///
/// # Arguments
///
/// * `path` - Path where the repository should be initialized
///
/// # Example
///
/// ```no_run
/// oxid::commands::init::run(".").unwrap();
/// ```
pub fn run(path: &str) -> Result<()> {
    Repository::init(path)?;
    println!("Initialized empty oxid repository in {}/.git/", path);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_init_creates_git_directory() {
        let dir = tempdir().unwrap();
        run(dir.path().to_str().unwrap()).unwrap();

        // Verify .git directory exists
        assert!(dir.path().join(".git").exists());
        assert!(dir.path().join(".git/objects").exists());
        assert!(dir.path().join(".git/refs/heads").exists());
        assert!(dir.path().join(".git/HEAD").exists());
    }
}
