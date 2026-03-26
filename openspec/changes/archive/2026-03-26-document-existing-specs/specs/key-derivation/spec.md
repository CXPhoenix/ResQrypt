## ADDED Requirements

### Requirement: Argon2id algorithm selection

The system SHALL use the Argon2id variant (combining Argon2i and Argon2d properties) with version `0x13` for all password-based key derivation. Argon2id provides resistance against both side-channel attacks and GPU/ASIC-based brute-force attacks.

#### Scenario: Key derivation uses Argon2id

- **WHEN** the system derives a key from a password
- **THEN** it SHALL use the Argon2id algorithm (not Argon2i or Argon2d) with version 0x13

### Requirement: Default KDF parameters

The system SHALL use the following default parameters for Argon2id key derivation:

- Memory cost: 65,536 KiB (64 MB)
- Time cost (iterations): 3
- Parallelism: 4
- Output key length: 32 bytes

#### Scenario: Default parameters applied when no custom values specified

- **WHEN** the user does not provide custom KDF parameters
- **THEN** the system SHALL use memory_cost=65536 KiB, time_cost=3, parallelism=4, output_len=32

### Requirement: Custom KDF parameters

The system SHALL accept custom Argon2id parameters via CLI arguments. The `KdfParams::new` constructor SHALL accept memory in megabytes and convert to KiB internally (multiply by 1024).

#### Scenario: Custom memory parameter

- **WHEN** the user specifies `--argon2-memory 32`
- **THEN** the system SHALL set memory_cost to 32,768 KiB (32 * 1024)

#### Scenario: Custom iteration count

- **WHEN** the user specifies `--argon2-iterations 5`
- **THEN** the system SHALL set time_cost to 5

#### Scenario: Custom parallelism

- **WHEN** the user specifies `--argon2-parallelism 2`
- **THEN** the system SHALL set parallelism to 2

### Requirement: Random salt generation

The system SHALL generate a cryptographically random 32-byte salt for each encryption operation using a cryptographic random number generator. The salt SHALL be stored in the file header for use during decryption.

#### Scenario: Unique salt per encryption

- **WHEN** two files are encrypted, even with the same password and input data
- **THEN** each encryption operation SHALL generate a different random 32-byte salt

### Requirement: Deterministic key derivation

Given the same password, salt, and KDF parameters, the system SHALL always produce the same 32-byte key. This determinism is essential for decryption to succeed.

#### Scenario: Same inputs produce same key

- **WHEN** `derive_key` is called twice with identical password, salt, and parameters
- **THEN** both calls SHALL return the same 32-byte key

#### Scenario: Different passwords produce different keys

- **WHEN** `derive_key` is called with different passwords but the same salt and parameters
- **THEN** the resulting keys SHALL differ

#### Scenario: Different salts produce different keys

- **WHEN** `derive_key` is called with the same password but different salts
- **THEN** the resulting keys SHALL differ

### Requirement: Output key length

The system SHALL always produce a 32-byte (256-bit) key from the key derivation function, matching the requirement of AES-256-GCM.

#### Scenario: Key length is 32 bytes

- **WHEN** a key is derived from any valid password and salt
- **THEN** the resulting key SHALL be exactly 32 bytes long

### Requirement: KDF parameter validation

The system SHALL validate that the provided Argon2id parameters are valid. If parameters are invalid (e.g., memory_cost too low for the parallelism degree), the system SHALL return a `CryptoError`.

#### Scenario: Invalid Argon2 parameters rejected

- **WHEN** the system receives Argon2id parameters that violate Argon2 constraints
- **THEN** the system SHALL return a `CryptoError` with the message "Invalid Argon2 params: {details}"
