//! Tar archive operations
//!
//! Creates and extracts tar archives for directory encryption.

use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;

use tar::{Archive, Builder};
use walkdir::WalkDir;

use crate::error::{ResqryptError, Result};

/// Create a tar archive from a directory
///
/// # Arguments
/// * `source_dir` - Path to the directory to archive
///
/// # Returns
/// The tar archive as a byte vector
pub fn create_archive<P: AsRef<Path>>(source_dir: P) -> Result<Vec<u8>> {
    let source_dir = source_dir.as_ref();

    if !source_dir.is_dir() {
        return Err(ResqryptError::InvalidArgument(format!(
            "Source is not a directory: {}",
            source_dir.display()
        )));
    }

    let mut archive_data = Vec::new();

    {
        let mut builder = Builder::new(&mut archive_data);

        // Get the parent directory name to use as the archive root
        let dir_name = source_dir
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_else(|| "archive".to_string());

        for entry in WalkDir::new(source_dir).follow_links(false) {
            let entry =
                entry.map_err(|e| ResqryptError::ArchiveError(format!("Walk error: {}", e)))?;

            let path = entry.path();

            // Calculate relative path within the archive
            let relative_path = path
                .strip_prefix(source_dir)
                .map_err(|e| ResqryptError::ArchiveError(format!("Path error: {}", e)))?;

            // Skip the root directory itself
            if relative_path.as_os_str().is_empty() {
                continue;
            }

            // Create archive path with directory name as root
            let archive_path = Path::new(&dir_name).join(relative_path);

            if path.is_dir() {
                builder
                    .append_dir(&archive_path, path)
                    .map_err(|e| ResqryptError::ArchiveError(format!("Add dir error: {}", e)))?;
            } else if path.is_file() {
                let mut file = File::open(path)?;
                builder
                    .append_file(&archive_path, &mut file)
                    .map_err(|e| ResqryptError::ArchiveError(format!("Add file error: {}", e)))?;
            }
        }

        builder
            .finish()
            .map_err(|e| ResqryptError::ArchiveError(format!("Finish error: {}", e)))?;
    }

    Ok(archive_data)
}

/// Extract a tar archive to a directory
///
/// # Arguments
/// * `archive_data` - The tar archive bytes
/// * `dest_dir` - Destination directory (will be created if needed)
pub fn extract_archive<P: AsRef<Path>>(archive_data: &[u8], dest_dir: P) -> Result<()> {
    let dest_dir = dest_dir.as_ref();

    // Create destination directory if it doesn't exist
    fs::create_dir_all(dest_dir)?;

    let mut archive = Archive::new(archive_data);

    archive
        .unpack(dest_dir)
        .map_err(|e| ResqryptError::ArchiveError(format!("Extract error: {}", e)))?;

    Ok(())
}

/// Read a file's contents into memory
pub fn read_file<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    let path = path.as_ref();

    if !path.exists() {
        return Err(ResqryptError::NotFound(path.to_path_buf()));
    }

    let mut file = File::open(path)?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents)?;

    Ok(contents)
}

/// Write data to a file
pub fn write_file<P: AsRef<Path>>(path: P, data: &[u8]) -> Result<()> {
    let path = path.as_ref();

    // Create parent directories if needed
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(path)?;
    file.write_all(data)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_archive_roundtrip() {
        // Create a temp directory with some files
        let source_dir = TempDir::new().unwrap();
        let source_path = source_dir.path();

        // Create test files
        fs::write(source_path.join("file1.txt"), "Hello, World!").unwrap();
        fs::create_dir(source_path.join("subdir")).unwrap();
        fs::write(source_path.join("subdir/file2.txt"), "Nested file").unwrap();

        // Create archive
        let archive_data = create_archive(source_path).unwrap();
        assert!(!archive_data.is_empty());

        // Extract to a different directory
        let dest_dir = TempDir::new().unwrap();
        extract_archive(&archive_data, dest_dir.path()).unwrap();

        // Get the extracted directory name
        let extracted_dir = dest_dir.path().join(source_path.file_name().unwrap());

        // Verify contents
        let content1 = fs::read_to_string(extracted_dir.join("file1.txt")).unwrap();
        assert_eq!(content1, "Hello, World!");

        let content2 = fs::read_to_string(extracted_dir.join("subdir/file2.txt")).unwrap();
        assert_eq!(content2, "Nested file");
    }

    #[test]
    fn test_read_write_file() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("test.txt");

        let data = b"Test data";
        write_file(&file_path, data).unwrap();

        let read_data = read_file(&file_path).unwrap();
        assert_eq!(data.as_slice(), read_data.as_slice());
    }

    #[test]
    fn test_read_nonexistent_file() {
        let result = read_file("/nonexistent/path/file.txt");
        assert!(result.is_err());
    }

    #[test]
    fn test_archive_not_a_directory() {
        let temp_dir = TempDir::new().unwrap();
        let file_path = temp_dir.path().join("file.txt");
        fs::write(&file_path, "content").unwrap();

        let result = create_archive(&file_path);
        assert!(result.is_err());
    }
}
