//! Common trait and types for Git objects

use anyhow::Result;

/// Git object types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectType {
    Blob,
    Tree,
    Commit,
    Tag,
}

impl ObjectType {
    /// Convert object type to string representation
    pub fn as_str(&self) -> &str {
        match self {
            ObjectType::Blob => "blob",
            ObjectType::Tree => "tree",
            ObjectType::Commit => "commit",
            ObjectType::Tag => "tag",
        }
    }

    /// Parse object type from string
    pub fn from_str(s: &str) -> Result<Self> {
        match s {
            "blob" => Ok(ObjectType::Blob),
            "tree" => Ok(ObjectType::Tree),
            "commit" => Ok(ObjectType::Commit),
            "tag" => Ok(ObjectType::Tag),
            _ => anyhow::bail!("Unknown object type: {}", s),
        }
    }
}

/// Trait for all Git objects
///
/// This trait defines the common behavior all Git objects must implement:
/// - Identifying their type
/// - Serializing to bytes
/// - Computing their hash
/// - Converting to Git's storage format
pub trait GitObject {
    /// Get the object type
    fn object_type(&self) -> ObjectType;

    /// Serialize object content (without header)
    fn serialize(&self) -> Result<Vec<u8>>;

    /// Compute the object's SHA-1 hash
    fn hash(&self) -> Result<String> {
        let data = self.to_bytes()?;
        Ok(crate::utils::hash_data(&data))
    }

    /// Convert to bytes with Git object format: [type] [size]\0[content]
    fn to_bytes(&self) -> Result<Vec<u8>> {
        let content = self.serialize()?;
        let header = format!("{} {}\0", self.object_type().as_str(), content.len());

        let mut data = header.as_bytes().to_vec();
        data.extend_from_slice(&content);

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_object_type_as_str() {
        assert_eq!(ObjectType::Blob.as_str(), "blob");
        assert_eq!(ObjectType::Tree.as_str(), "tree");
        assert_eq!(ObjectType::Commit.as_str(), "commit");
        assert_eq!(ObjectType::Tag.as_str(), "tag");
    }

    #[test]
    fn test_object_type_from_str() {
        assert_eq!(ObjectType::from_str("blob").unwrap(), ObjectType::Blob);
        assert_eq!(ObjectType::from_str("tree").unwrap(), ObjectType::Tree);
        assert!(ObjectType::from_str("invalid").is_err());
    }
}
