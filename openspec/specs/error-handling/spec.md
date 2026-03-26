## ADDED Requirements

### Requirement: Error type enumeration

The system SHALL define a `ResqryptError` enum with the following 9 variants, each representing a distinct failure category:

1. `Io` — I/O operation failure (wraps `std::io::Error`)
2. `InvalidFormat` — File is not a valid `.resqrypt` file (invalid magic bytes or version)
3. `CryptoError` — Encryption, decryption, or key derivation failure
4. `PasswordError` — Password-related failure (empty password, mismatch, wrong password)
5. `CompressionError` — Compression or decompression failure
6. `ArchiveError` — Tar archive creation or extraction failure
7. `NotFound` — Input file or directory does not exist
8. `AlreadyExists` — Output file already exists
9. `InvalidArgument` — Invalid user-provided argument

#### Scenario: All error variants are distinct

- **WHEN** an error occurs in the system
- **THEN** it SHALL be categorized into exactly one of the 9 defined variants

### Requirement: Io error auto-conversion

The `Io` variant SHALL automatically convert from `std::io::Error` using the `From` trait. Any `std::io::Error` propagated with the `?` operator SHALL be wrapped in `ResqryptError::Io`.

#### Scenario: IO error propagation

- **WHEN** a standard I/O error occurs (e.g., file not readable, disk full)
- **THEN** it SHALL be automatically converted to `ResqryptError::Io` with the original error message preserved

### Requirement: User-facing error messages

Each error variant SHALL produce a human-readable error message via the `Display` trait. The message format for each variant SHALL be:

- `Io`: "I/O error: {original_error_message}"
- `InvalidFormat`: "Invalid file format: {details}"
- `CryptoError`: "Cryptographic operation failed: {details}"
- `PasswordError`: "Password error: {details}"
- `CompressionError`: "Compression error: {details}"
- `ArchiveError`: "Archive error: {details}"
- `NotFound`: "Not found: {path}"
- `AlreadyExists`: "File already exists: {path}"
- `InvalidArgument`: "Invalid argument: {details}"

#### Scenario: InvalidFormat error message

- **WHEN** the system encounters a file without valid magic bytes
- **THEN** the error message SHALL be "Invalid file format: Not a valid resqrypt file (invalid magic bytes)"

#### Scenario: PasswordError for wrong password

- **WHEN** decryption fails due to wrong password
- **THEN** the error message SHALL be "Password error: Decryption failed: wrong password or corrupted data"

#### Scenario: NotFound error message

- **WHEN** the specified input path does not exist
- **THEN** the error message SHALL be "Not found: {path}" where `{path}` is the full file path

#### Scenario: AlreadyExists error message

- **WHEN** the specified output path already exists
- **THEN** the error message SHALL be "File already exists: {path}" where `{path}` is the full file path

### Requirement: Result type alias

The system SHALL define a `Result<T>` type alias as `std::result::Result<T, ResqryptError>` for use throughout the codebase.

#### Scenario: Result type usage

- **WHEN** a function returns a `Result<T>`
- **THEN** the error type SHALL always be `ResqryptError`

### Requirement: Error context preservation

Each error variant that wraps a string message SHALL preserve the specific context of the failure (e.g., which operation failed, what was expected vs. found). Generic messages without context SHALL NOT be used.

#### Scenario: CryptoError includes operation context

- **WHEN** cipher creation fails
- **THEN** the error message SHALL include "Failed to create cipher: {details}"

#### Scenario: CompressionError includes operation context

- **WHEN** compression fails
- **THEN** the error message SHALL include "Compression failed: {details}"

#### Scenario: ArchiveError includes operation context

- **WHEN** archive extraction fails
- **THEN** the error message SHALL include "Extract error: {details}"

## Requirements

### Requirement: Error type enumeration

The system SHALL define a `ResqryptError` enum with the following 9 variants, each representing a distinct failure category:

1. `Io` — I/O operation failure (wraps `std::io::Error`)
2. `InvalidFormat` — File is not a valid `.resqrypt` file (invalid magic bytes or version)
3. `CryptoError` — Encryption, decryption, or key derivation failure
4. `PasswordError` — Password-related failure (empty password, mismatch, wrong password)
5. `CompressionError` — Compression or decompression failure
6. `ArchiveError` — Tar archive creation or extraction failure
7. `NotFound` — Input file or directory does not exist
8. `AlreadyExists` — Output file already exists
9. `InvalidArgument` — Invalid user-provided argument

#### Scenario: All error variants are distinct

- **WHEN** an error occurs in the system
- **THEN** it SHALL be categorized into exactly one of the 9 defined variants

---
### Requirement: Io error auto-conversion

The `Io` variant SHALL automatically convert from `std::io::Error` using the `From` trait. Any `std::io::Error` propagated with the `?` operator SHALL be wrapped in `ResqryptError::Io`.

#### Scenario: IO error propagation

- **WHEN** a standard I/O error occurs (e.g., file not readable, disk full)
- **THEN** it SHALL be automatically converted to `ResqryptError::Io` with the original error message preserved

---
### Requirement: User-facing error messages

Each error variant SHALL produce a human-readable error message via the `Display` trait. The message format for each variant SHALL be:

- `Io`: "I/O error: {original_error_message}"
- `InvalidFormat`: "Invalid file format: {details}"
- `CryptoError`: "Cryptographic operation failed: {details}"
- `PasswordError`: "Password error: {details}"
- `CompressionError`: "Compression error: {details}"
- `ArchiveError`: "Archive error: {details}"
- `NotFound`: "Not found: {path}"
- `AlreadyExists`: "File already exists: {path}"
- `InvalidArgument`: "Invalid argument: {details}"

#### Scenario: InvalidFormat error message

- **WHEN** the system encounters a file without valid magic bytes
- **THEN** the error message SHALL be "Invalid file format: Not a valid resqrypt file (invalid magic bytes)"

#### Scenario: PasswordError for wrong password

- **WHEN** decryption fails due to wrong password
- **THEN** the error message SHALL be "Password error: Decryption failed: wrong password or corrupted data"

#### Scenario: NotFound error message

- **WHEN** the specified input path does not exist
- **THEN** the error message SHALL be "Not found: {path}" where `{path}` is the full file path

#### Scenario: AlreadyExists error message

- **WHEN** the specified output path already exists
- **THEN** the error message SHALL be "File already exists: {path}" where `{path}` is the full file path

---
### Requirement: Result type alias

The system SHALL define a `Result<T>` type alias as `std::result::Result<T, ResqryptError>` for use throughout the codebase.

#### Scenario: Result type usage

- **WHEN** a function returns a `Result<T>`
- **THEN** the error type SHALL always be `ResqryptError`

---
### Requirement: Error context preservation

Each error variant that wraps a string message SHALL preserve the specific context of the failure (e.g., which operation failed, what was expected vs. found). Generic messages without context SHALL NOT be used.

#### Scenario: CryptoError includes operation context

- **WHEN** cipher creation fails
- **THEN** the error message SHALL include "Failed to create cipher: {details}"

#### Scenario: CompressionError includes operation context

- **WHEN** compression fails
- **THEN** the error message SHALL include "Compression failed: {details}"

#### Scenario: ArchiveError includes operation context

- **WHEN** archive extraction fails
- **THEN** the error message SHALL include "Extract error: {details}"
