## ADDED Requirements

### Requirement: AES-256-GCM authenticated encryption

The system SHALL use AES-256-GCM (Galois/Counter Mode) as the authenticated encryption algorithm. AES-256-GCM provides both confidentiality and integrity through a 16-byte authentication tag appended to the ciphertext.

#### Scenario: Encrypt plaintext data

- **WHEN** the system encrypts plaintext with a 32-byte key and a 12-byte nonce
- **THEN** the system SHALL produce ciphertext of length `plaintext.len() + 16` bytes (plaintext length plus 16-byte authentication tag)

#### Scenario: Encrypt empty plaintext

- **WHEN** the system encrypts zero-length plaintext
- **THEN** the system SHALL produce exactly 16 bytes of ciphertext (the authentication tag only)

### Requirement: Nonce generation

The system SHALL generate a cryptographically random 12-byte nonce for each encryption operation using a cryptographic random number generator. The nonce SHALL NOT be reused with the same key.

#### Scenario: Random nonce generation

- **WHEN** the system generates a nonce
- **THEN** the nonce SHALL be exactly 12 bytes long and produced by a cryptographic random number generator

### Requirement: Decryption with authentication verification

The system SHALL verify the AES-GCM authentication tag during decryption. If verification fails (wrong key, wrong nonce, or tampered ciphertext), the system SHALL reject the decryption and return an error. The system SHALL NOT produce any partial plaintext output on authentication failure.

#### Scenario: Successful decryption with correct key and nonce

- **WHEN** the system decrypts ciphertext using the same key and nonce that were used for encryption
- **THEN** the system SHALL produce the original plaintext identically

#### Scenario: Decryption with wrong key

- **WHEN** the system attempts decryption with a different key than was used for encryption
- **THEN** the system SHALL return a `PasswordError` with the message "Decryption failed: wrong password or corrupted data"

#### Scenario: Decryption with wrong nonce

- **WHEN** the system attempts decryption with a different nonce than was used for encryption
- **THEN** the system SHALL return a `PasswordError` with the message "Decryption failed: wrong password or corrupted data"

#### Scenario: Decryption of tampered ciphertext

- **WHEN** any byte of the ciphertext (including the authentication tag) has been modified
- **THEN** the system SHALL return a `PasswordError` with the message "Decryption failed: wrong password or corrupted data"

### Requirement: Ciphertext minimum length validation

The system SHALL validate that ciphertext is at least 16 bytes long (the minimum size for an AES-GCM authentication tag) before attempting decryption.

#### Scenario: Ciphertext too short

- **WHEN** the system receives ciphertext shorter than 16 bytes for decryption
- **THEN** the system SHALL return a `CryptoError` with the message "Ciphertext too short"

### Requirement: Large data support

The system SHALL support encryption and decryption of data up to at least 1 MB in a single operation without data loss.

#### Scenario: Encrypt and decrypt 1 MB data

- **WHEN** the system encrypts and then decrypts 1 MB of data
- **THEN** the decrypted output SHALL be byte-for-byte identical to the original input

## Requirements

### Requirement: AES-256-GCM authenticated encryption

The system SHALL use AES-256-GCM (Galois/Counter Mode) as the authenticated encryption algorithm. AES-256-GCM provides both confidentiality and integrity through a 16-byte authentication tag appended to the ciphertext.

#### Scenario: Encrypt plaintext data

- **WHEN** the system encrypts plaintext with a 32-byte key and a 12-byte nonce
- **THEN** the system SHALL produce ciphertext of length `plaintext.len() + 16` bytes (plaintext length plus 16-byte authentication tag)

#### Scenario: Encrypt empty plaintext

- **WHEN** the system encrypts zero-length plaintext
- **THEN** the system SHALL produce exactly 16 bytes of ciphertext (the authentication tag only)

---
### Requirement: Nonce generation

The system SHALL generate a cryptographically random 12-byte nonce for each encryption operation using a cryptographic random number generator. The nonce SHALL NOT be reused with the same key.

#### Scenario: Random nonce generation

- **WHEN** the system generates a nonce
- **THEN** the nonce SHALL be exactly 12 bytes long and produced by a cryptographic random number generator

---
### Requirement: Decryption with authentication verification

The system SHALL verify the AES-GCM authentication tag during decryption. If verification fails (wrong key, wrong nonce, or tampered ciphertext), the system SHALL reject the decryption and return an error. The system SHALL NOT produce any partial plaintext output on authentication failure.

#### Scenario: Successful decryption with correct key and nonce

- **WHEN** the system decrypts ciphertext using the same key and nonce that were used for encryption
- **THEN** the system SHALL produce the original plaintext identically

#### Scenario: Decryption with wrong key

- **WHEN** the system attempts decryption with a different key than was used for encryption
- **THEN** the system SHALL return a `PasswordError` with the message "Decryption failed: wrong password or corrupted data"

#### Scenario: Decryption with wrong nonce

- **WHEN** the system attempts decryption with a different nonce than was used for encryption
- **THEN** the system SHALL return a `PasswordError` with the message "Decryption failed: wrong password or corrupted data"

#### Scenario: Decryption of tampered ciphertext

- **WHEN** any byte of the ciphertext (including the authentication tag) has been modified
- **THEN** the system SHALL return a `PasswordError` with the message "Decryption failed: wrong password or corrupted data"

---
### Requirement: Ciphertext minimum length validation

The system SHALL validate that ciphertext is at least 16 bytes long (the minimum size for an AES-GCM authentication tag) before attempting decryption.

#### Scenario: Ciphertext too short

- **WHEN** the system receives ciphertext shorter than 16 bytes for decryption
- **THEN** the system SHALL return a `CryptoError` with the message "Ciphertext too short"

---
### Requirement: Large data support

The system SHALL support encryption and decryption of data up to at least 1 MB in a single operation without data loss.

#### Scenario: Encrypt and decrypt 1 MB data

- **WHEN** the system encrypts and then decrypts 1 MB of data
- **THEN** the decrypted output SHALL be byte-for-byte identical to the original input
