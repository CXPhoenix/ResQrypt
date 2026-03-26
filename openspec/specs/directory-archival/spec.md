## ADDED Requirements

### Requirement: Tar archive creation from directory

The system SHALL create a tar archive from a directory and all its contents (files and subdirectories) for encryption. The archive SHALL use the source directory's name as the root entry in the archive.

#### Scenario: Archive a directory with files and subdirectories

- **WHEN** the system archives a directory containing files and nested subdirectories
- **THEN** the tar archive SHALL include all files and subdirectories, with paths prefixed by the source directory name

#### Scenario: Source directory name as archive root

- **WHEN** the system archives a directory named "my-folder"
- **THEN** all entries in the tar archive SHALL be prefixed with "my-folder/" (e.g., "my-folder/file1.txt", "my-folder/subdir/file2.txt")

#### Scenario: Archive a directory with no name (root path)

- **WHEN** the source directory path has no file name component
- **THEN** the system SHALL use "archive" as the fallback root directory name

### Requirement: Directory validation

The system SHALL validate that the source path is a directory before attempting to create a tar archive. If the source is a file, the system SHALL return an error.

#### Scenario: Source is not a directory

- **WHEN** `create_archive` is called with a path that is a file (not a directory)
- **THEN** the system SHALL return an `InvalidArgument` error with the message "Source is not a directory: {path}"

### Requirement: Symbolic link handling

The system SHALL NOT follow symbolic links when traversing directories for archival. The `walkdir` traversal SHALL use `follow_links(false)`.

#### Scenario: Symbolic links not followed

- **WHEN** a directory contains symbolic links
- **THEN** the system SHALL NOT follow symbolic links during directory traversal

### Requirement: Tar archive extraction

The system SHALL extract a tar archive to a specified destination directory during decryption. If the destination directory does not exist, the system SHALL create it (including parent directories).

#### Scenario: Extract archive to existing directory

- **WHEN** the system extracts a tar archive to an existing directory
- **THEN** the archive contents SHALL be unpacked into that directory, preserving the directory structure from the archive

#### Scenario: Extract archive to non-existing directory

- **WHEN** the system extracts a tar archive to a directory path that does not exist
- **THEN** the system SHALL create the directory (and any parent directories) before extraction

### Requirement: Archive roundtrip consistency

A directory archived with `create_archive` and then extracted with `extract_archive` SHALL preserve all file contents and directory structure identically.

#### Scenario: Roundtrip preserves file contents

- **WHEN** a directory is archived and then extracted
- **THEN** every file in the extracted output SHALL have byte-for-byte identical contents to the corresponding original file

#### Scenario: Roundtrip preserves directory structure

- **WHEN** a directory with nested subdirectories is archived and then extracted
- **THEN** the extracted output SHALL contain the same subdirectory hierarchy as the original

### Requirement: IS_DIRECTORY flag integration

When the input to the encrypt command is a directory, the system SHALL set the `IS_DIRECTORY` flag (bit 1) in the file header flags byte. During decryption, the system SHALL check this flag to determine whether to extract the decrypted data as a tar archive.

#### Scenario: Directory input sets IS_DIRECTORY flag

- **WHEN** the encrypt command receives a directory as input
- **THEN** the system SHALL create a tar archive from the directory and set the `IS_DIRECTORY` flag in the header

#### Scenario: File input does not set IS_DIRECTORY flag

- **WHEN** the encrypt command receives a regular file as input
- **THEN** the system SHALL NOT set the `IS_DIRECTORY` flag in the header

#### Scenario: IS_DIRECTORY flag triggers archive extraction on decrypt

- **WHEN** the `IS_DIRECTORY` flag is set in the decrypted file's header
- **THEN** the system SHALL extract the decrypted payload as a tar archive to the output path

### Requirement: File I/O utilities

The system SHALL provide utility functions for reading files into memory and writing data to files. The write utility SHALL create parent directories if they do not exist.

#### Scenario: Read existing file

- **WHEN** `read_file` is called with a path to an existing file
- **THEN** the system SHALL return the file's entire contents as a byte vector

#### Scenario: Read non-existing file

- **WHEN** `read_file` is called with a path to a file that does not exist
- **THEN** the system SHALL return a `NotFound` error

#### Scenario: Write file creates parent directories

- **WHEN** `write_file` is called with a path whose parent directories do not exist
- **THEN** the system SHALL create all necessary parent directories before writing the file

## Requirements

### Requirement: Tar archive creation from directory

The system SHALL create a tar archive from a directory and all its contents (files and subdirectories) for encryption. The archive SHALL use the source directory's name as the root entry in the archive.

#### Scenario: Archive a directory with files and subdirectories

- **WHEN** the system archives a directory containing files and nested subdirectories
- **THEN** the tar archive SHALL include all files and subdirectories, with paths prefixed by the source directory name

#### Scenario: Source directory name as archive root

- **WHEN** the system archives a directory named "my-folder"
- **THEN** all entries in the tar archive SHALL be prefixed with "my-folder/" (e.g., "my-folder/file1.txt", "my-folder/subdir/file2.txt")

#### Scenario: Archive a directory with no name (root path)

- **WHEN** the source directory path has no file name component
- **THEN** the system SHALL use "archive" as the fallback root directory name

---
### Requirement: Directory validation

The system SHALL validate that the source path is a directory before attempting to create a tar archive. If the source is a file, the system SHALL return an error.

#### Scenario: Source is not a directory

- **WHEN** `create_archive` is called with a path that is a file (not a directory)
- **THEN** the system SHALL return an `InvalidArgument` error with the message "Source is not a directory: {path}"

---
### Requirement: Symbolic link handling

The system SHALL NOT follow symbolic links when traversing directories for archival. The `walkdir` traversal SHALL use `follow_links(false)`.

#### Scenario: Symbolic links not followed

- **WHEN** a directory contains symbolic links
- **THEN** the system SHALL NOT follow symbolic links during directory traversal

---
### Requirement: Tar archive extraction

The system SHALL extract a tar archive to a specified destination directory during decryption. If the destination directory does not exist, the system SHALL create it (including parent directories).

#### Scenario: Extract archive to existing directory

- **WHEN** the system extracts a tar archive to an existing directory
- **THEN** the archive contents SHALL be unpacked into that directory, preserving the directory structure from the archive

#### Scenario: Extract archive to non-existing directory

- **WHEN** the system extracts a tar archive to a directory path that does not exist
- **THEN** the system SHALL create the directory (and any parent directories) before extraction

---
### Requirement: Archive roundtrip consistency

A directory archived with `create_archive` and then extracted with `extract_archive` SHALL preserve all file contents and directory structure identically.

#### Scenario: Roundtrip preserves file contents

- **WHEN** a directory is archived and then extracted
- **THEN** every file in the extracted output SHALL have byte-for-byte identical contents to the corresponding original file

#### Scenario: Roundtrip preserves directory structure

- **WHEN** a directory with nested subdirectories is archived and then extracted
- **THEN** the extracted output SHALL contain the same subdirectory hierarchy as the original

---
### Requirement: IS_DIRECTORY flag integration

When the input to the encrypt command is a directory, the system SHALL set the `IS_DIRECTORY` flag (bit 1) in the file header flags byte. During decryption, the system SHALL check this flag to determine whether to extract the decrypted data as a tar archive.

#### Scenario: Directory input sets IS_DIRECTORY flag

- **WHEN** the encrypt command receives a directory as input
- **THEN** the system SHALL create a tar archive from the directory and set the `IS_DIRECTORY` flag in the header

#### Scenario: File input does not set IS_DIRECTORY flag

- **WHEN** the encrypt command receives a regular file as input
- **THEN** the system SHALL NOT set the `IS_DIRECTORY` flag in the header

#### Scenario: IS_DIRECTORY flag triggers archive extraction on decrypt

- **WHEN** the `IS_DIRECTORY` flag is set in the decrypted file's header
- **THEN** the system SHALL extract the decrypted payload as a tar archive to the output path

---
### Requirement: File I/O utilities

The system SHALL provide utility functions for reading files into memory and writing data to files. The write utility SHALL create parent directories if they do not exist.

#### Scenario: Read existing file

- **WHEN** `read_file` is called with a path to an existing file
- **THEN** the system SHALL return the file's entire contents as a byte vector

#### Scenario: Read non-existing file

- **WHEN** `read_file` is called with a path to a file that does not exist
- **THEN** the system SHALL return a `NotFound` error

#### Scenario: Write file creates parent directories

- **WHEN** `write_file` is called with a path whose parent directories do not exist
- **THEN** the system SHALL create all necessary parent directories before writing the file
