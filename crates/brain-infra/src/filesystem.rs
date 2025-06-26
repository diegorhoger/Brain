//! Filesystem Infrastructure
//! 
//! File system utilities and operations for the Brain AI system.

use brain_types::*;
use std::path::{Path, PathBuf};
use tokio::fs as async_fs;

/// Filesystem manager for Brain AI operations
pub struct FileSystemManager {
    base_path: PathBuf,
}

impl FileSystemManager {
    /// Create a new filesystem manager
    pub fn new<P: AsRef<Path>>(base_path: P) -> Self {
        Self {
            base_path: base_path.as_ref().to_path_buf(),
        }
    }

    /// Ensure the base directory exists
    pub async fn ensure_base_directory(&self) -> Result<()> {
        async_fs::create_dir_all(&self.base_path)
            .await?;
        Ok(())
    }

    /// Write content to a file
    pub async fn write_file<P: AsRef<Path>>(&self, path: P, content: &str) -> Result<()> {
        let full_path = self.base_path.join(path);
        
        // Ensure parent directory exists
        if let Some(parent) = full_path.parent() {
            async_fs::create_dir_all(parent)
                .await?;
        }

        async_fs::write(&full_path, content)
            .await?;
        
        Ok(())
    }

    /// Read content from a file
    pub async fn read_file<P: AsRef<Path>>(&self, path: P) -> Result<String> {
        let full_path = self.base_path.join(path);
        
        async_fs::read_to_string(&full_path)
            .await
            .map_err(|e| e.into())
    }

    /// Check if a file exists
    pub async fn file_exists<P: AsRef<Path>>(&self, path: P) -> bool {
        let full_path = self.base_path.join(path);
        async_fs::metadata(&full_path).await.is_ok()
    }

    /// Delete a file
    pub async fn delete_file<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let full_path = self.base_path.join(path);
        
        async_fs::remove_file(&full_path)
            .await?;
        
        Ok(())
    }

    /// List files in a directory
    pub async fn list_files<P: AsRef<Path>>(&self, path: P) -> Result<Vec<PathBuf>> {
        let full_path = self.base_path.join(path);
        
        let mut entries = async_fs::read_dir(&full_path)
            .await?;
        
        let mut files = Vec::new();
        while let Some(entry) = entries.next_entry()
            .await?
        {
            if entry.file_type()
                .await?
                .is_file()
            {
                files.push(entry.path());
            }
        }
        
        Ok(files)
    }

    /// Get the full path for a relative path
    pub fn full_path<P: AsRef<Path>>(&self, path: P) -> PathBuf {
        self.base_path.join(path)
    }
} 