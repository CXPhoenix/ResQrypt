## ADDED Requirements

### Requirement: Encrypt subcommand

The system SHALL provide an `encrypt` subcommand that encrypts a file or directory. The subcommand SHALL accept the following arguments:

- `-i, --input` (required): Input file or directory path
- `-o, --output` (required): Output encrypted file path (.resqrypt)
- `-p, --password` (optional): Encryption password; also accepts `RESQRYPT_PASSWORD` environment variable
- `--argon2-memory` (optional, default: 64): Argon2id memory cost in MB
- `--argon2-iterations` (optional, default: 3): Argon2id iteration count
- `--argon2-parallelism` (optional, default: 4): Argon2id parallelism degree
- `-v, --verbose` (optional): Enable verbose output

#### Scenario: Encrypt a file with all default parameters

- **WHEN** the user runs `resqrypt encrypt -i secret.txt -o secret.txt.resqrypt`
- **THEN** the system SHALL prompt for a password interactively, encrypt the file with default KDF parameters, and write the output to the specified path

#### Scenario: Encrypt with password from argument

- **WHEN** the user runs `resqrypt encrypt -i file.txt -o file.resqrypt -p mypassword`
- **THEN** the system SHALL use "mypassword" as the encryption password without prompting

#### Scenario: Encrypt with password from environment variable

- **WHEN** the `RESQRYPT_PASSWORD` environment variable is set and no `-p` argument is provided
- **THEN** the system SHALL use the value of `RESQRYPT_PASSWORD` as the encryption password without prompting

#### Scenario: Encrypt with custom KDF parameters

- **WHEN** the user runs `resqrypt encrypt -i file.txt -o file.resqrypt --argon2-memory 128 --argon2-iterations 5 --argon2-parallelism 8`
- **THEN** the system SHALL use the specified KDF parameters (memory=128MB, iterations=5, parallelism=8) for key derivation

### Requirement: Decrypt subcommand

The system SHALL provide a `decrypt` subcommand that decrypts a `.resqrypt` file. The subcommand SHALL accept the following arguments:

- `-i, --input` (required): Input encrypted file path (.resqrypt)
- `-o, --output` (required): Output file or directory path
- `-p, --password` (optional): Decryption password; also accepts `RESQRYPT_PASSWORD` environment variable
- `-v, --verbose` (optional): Enable verbose output

#### Scenario: Decrypt a file

- **WHEN** the user runs `resqrypt decrypt -i secret.txt.resqrypt -o secret.txt`
- **THEN** the system SHALL prompt for a password, read KDF parameters from the file header, derive the key, decrypt, and write the output

#### Scenario: Decrypt with password from argument

- **WHEN** the user runs `resqrypt decrypt -i file.resqrypt -o file.txt -p mypassword`
- **THEN** the system SHALL use "mypassword" as the decryption password without prompting

### Requirement: Interactive password prompt for encryption

When no password is provided via `-p` argument or `RESQRYPT_PASSWORD` environment variable during encryption, the system SHALL:

1. Prompt the user with "Enter encryption password: " (input hidden)
2. Prompt for confirmation with "Confirm password: " (input hidden)
3. Verify both entries match

#### Scenario: Password confirmation matches

- **WHEN** the user enters the same password twice during encryption
- **THEN** the system SHALL proceed with encryption using that password

#### Scenario: Password confirmation does not match

- **WHEN** the user enters different passwords for the initial and confirmation prompts
- **THEN** the system SHALL return a `PasswordError` with the message "Passwords do not match"

#### Scenario: Empty password rejected

- **WHEN** the user enters an empty string at the password prompt
- **THEN** the system SHALL return a `PasswordError` with the message "Password cannot be empty"

### Requirement: Interactive password prompt for decryption

When no password is provided via `-p` argument or `RESQRYPT_PASSWORD` environment variable during decryption, the system SHALL prompt the user with "Enter decryption password: " (input hidden). No confirmation prompt is required for decryption.

#### Scenario: Single password prompt for decryption

- **WHEN** the user is prompted for a decryption password
- **THEN** the system SHALL prompt exactly once (no confirmation step)

#### Scenario: Empty decryption password rejected

- **WHEN** the user enters an empty string at the decryption password prompt
- **THEN** the system SHALL return a `PasswordError` with the message "Password cannot be empty"

### Requirement: Input validation

The system SHALL validate that the input path exists before proceeding with encryption or decryption. If the input path does not exist, the system SHALL return a `NotFound` error.

#### Scenario: Input file does not exist

- **WHEN** the user specifies an input path that does not exist
- **THEN** the system SHALL return a `NotFound` error with the path

### Requirement: Output overwrite protection

The system SHALL refuse to overwrite an existing output file. If the output path already exists, the system SHALL return an `AlreadyExists` error.

#### Scenario: Output file already exists

- **WHEN** the user specifies an output path that already exists
- **THEN** the system SHALL return an `AlreadyExists` error with the path

### Requirement: Verbose output mode

When the `-v` or `--verbose` flag is provided, the system SHALL display additional information:

- During encryption: input size, output size, and compression ratio as a percentage
- During decryption: input size, output size, and the type of decrypted content (file or directory)
- During processing: progress spinner messages indicating the current operation step

#### Scenario: Verbose encryption output

- **WHEN** the user encrypts a file with `-v`
- **THEN** the system SHALL display input bytes, output bytes, and a percentage ratio (e.g., "Input: 1000 bytes, Output: 850 bytes (85.0%)")

#### Scenario: Verbose decryption output for directory

- **WHEN** the user decrypts a file that was originally a directory, with `-v`
- **THEN** the system SHALL display input/output sizes and "Type: Directory (extracted from archive)"

#### Scenario: Verbose decryption output for file

- **WHEN** the user decrypts a file that was originally a single file, with `-v`
- **THEN** the system SHALL display input/output sizes and "Type: File"

### Requirement: Success message

Upon successful completion, the system SHALL display a confirmation message indicating the input and output paths.

#### Scenario: Encrypt success message

- **WHEN** encryption completes successfully
- **THEN** the system SHALL display "Encrypted: {input_path} -> {output_path}"

#### Scenario: Decrypt success message

- **WHEN** decryption completes successfully
- **THEN** the system SHALL display "Decrypted: {input_path} -> {output_path}"

### Requirement: Exit code

The system SHALL exit with code 0 on success and code 1 on any error.

#### Scenario: Successful operation exits with code 0

- **WHEN** an encrypt or decrypt operation completes successfully
- **THEN** the process SHALL exit with code 0

#### Scenario: Failed operation exits with code 1

- **WHEN** an encrypt or decrypt operation fails for any reason
- **THEN** the process SHALL exit with code 1
