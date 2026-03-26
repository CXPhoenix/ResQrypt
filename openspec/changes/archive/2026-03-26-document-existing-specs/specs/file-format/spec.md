## ADDED Requirements

### Requirement: Magic bytes identification

Every `.resqrypt` file SHALL begin with the 8-byte ASCII sequence `RESQRYPT` (hex: `52 45 53 51 52 59 50 54`). The system SHALL reject any file that does not start with these exact magic bytes as an invalid format.

#### Scenario: Valid magic bytes

- **WHEN** a file begins with the bytes `RESQRYPT`
- **THEN** the system SHALL accept the file as a candidate for decryption and proceed to parse the header

#### Scenario: Invalid magic bytes

- **WHEN** a file does not begin with the bytes `RESQRYPT`
- **THEN** the system SHALL return an `InvalidFormat` error with the message "Not a valid resqrypt file (invalid magic bytes)"

### Requirement: Format version field

The 9th byte (offset 8) of the file SHALL contain the format version number. The current version SHALL be `0x01`. The system SHALL reject files with any other version number.

#### Scenario: Current version accepted

- **WHEN** the version byte is `0x01`
- **THEN** the system SHALL proceed to parse the remainder of the header

#### Scenario: Unsupported version rejected

- **WHEN** the version byte is any value other than `0x01`
- **THEN** the system SHALL return an `InvalidFormat` error with the message "Unsupported file format version: {version} (expected 1)"

### Requirement: Flags byte

The 10th byte (offset 9) SHALL be a flags byte encoding metadata about the encrypted payload. The following bit flags SHALL be defined:

- Bit 0 (`0b0000_0001`, `ALREADY_ZSTD`): When set, indicates the original data was already zstd-compressed and compression was skipped during encryption.
- Bit 1 (`0b0000_0010`, `IS_DIRECTORY`): When set, indicates the encrypted payload is a tar archive of a directory.

#### Scenario: File with no flags set

- **WHEN** the flags byte is `0x00`
- **THEN** the system SHALL treat the payload as a single file that was compressed during encryption

#### Scenario: ALREADY_ZSTD flag set

- **WHEN** bit 0 of the flags byte is set
- **THEN** the system SHALL skip decompression during decryption and return the decrypted data directly

#### Scenario: IS_DIRECTORY flag set

- **WHEN** bit 1 of the flags byte is set
- **THEN** the system SHALL treat the decrypted (and decompressed) payload as a tar archive and extract it to the output directory

#### Scenario: Both flags set

- **WHEN** both bit 0 and bit 1 of the flags byte are set
- **THEN** the system SHALL skip decompression and extract the decrypted data as a tar archive

### Requirement: KDF parameters in header

Bytes at offset 10-21 (12 bytes total) SHALL store the Argon2id key derivation parameters used during encryption, encoded as three consecutive little-endian unsigned 32-bit integers:

- Offset 10-13: `memory_cost` in KiB
- Offset 14-17: `time_cost` (iteration count)
- Offset 18-21: `parallelism` (degree of parallelism)

These values SHALL be read from the header during decryption to reproduce the same key derivation.

#### Scenario: Default KDF parameters stored

- **WHEN** a file is encrypted with default parameters (memory_cost=65536 KiB, time_cost=3, parallelism=4)
- **THEN** the header SHALL contain the bytes `00 00 01 00` (65536 LE), `03 00 00 00` (3 LE), `04 00 00 00` (4 LE) at offsets 10-21

#### Scenario: Custom KDF parameters stored

- **WHEN** a file is encrypted with custom parameters (e.g., memory_cost=32768 KiB, time_cost=5, parallelism=2)
- **THEN** the header SHALL store those exact values in little-endian format at offsets 10-21

### Requirement: Salt field

Bytes at offset 22-53 (32 bytes) SHALL contain the random salt used for Argon2id key derivation. A new random 32-byte salt SHALL be generated for each encryption operation.

#### Scenario: Salt stored in header

- **WHEN** a file is encrypted
- **THEN** the generated 32-byte salt SHALL be written at offset 22-53 of the output file

#### Scenario: Salt read during decryption

- **WHEN** a file is decrypted
- **THEN** the system SHALL read 32 bytes from offset 22-53 and use them as the salt for key derivation

### Requirement: Nonce field

Bytes at offset 54-65 (12 bytes) SHALL contain the AES-GCM nonce used for encryption. A new random 12-byte nonce SHALL be generated for each encryption operation.

#### Scenario: Nonce stored in header

- **WHEN** a file is encrypted
- **THEN** the generated 12-byte nonce SHALL be written at offset 54-65 of the output file

#### Scenario: Nonce read during decryption

- **WHEN** a file is decrypted
- **THEN** the system SHALL read 12 bytes from offset 54-65 and use them as the nonce for AES-GCM decryption

### Requirement: Fixed header size

The total header size SHALL be exactly 66 bytes: 8 (magic) + 1 (version) + 1 (flags) + 12 (KDF params) + 32 (salt) + 12 (nonce). All encrypted payload data SHALL begin immediately after offset 65.

#### Scenario: Header size verification

- **WHEN** a header is written
- **THEN** exactly 66 bytes SHALL be written before the ciphertext begins

#### Scenario: Ciphertext follows header

- **WHEN** a file is read for decryption
- **THEN** all bytes after the first 66 bytes SHALL be treated as ciphertext (encrypted data + 16-byte authentication tag)

### Requirement: Header roundtrip consistency

A header written by `write_header` SHALL be exactly reproducible by `read_header`. Every field (version, flags, KDF params, salt, nonce) SHALL be identical after a write-then-read cycle.

#### Scenario: Write then read produces identical header

- **WHEN** a `FileHeader` is written to a buffer and then read back from that buffer
- **THEN** all fields (version, flags, memory_cost, time_cost, parallelism, salt, nonce) SHALL be identical to the original header
