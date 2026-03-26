## ADDED Requirements

### Requirement: Zstd compression

The system SHALL compress data using the Zstandard (zstd) algorithm at compression level 3 before encryption. Level 3 provides a balance between compression ratio and speed.

#### Scenario: Compress data

- **WHEN** the system compresses input data
- **THEN** the output SHALL be valid zstd-compressed data that can be decompressed back to the original input

#### Scenario: Compression reduces size for compressible data

- **WHEN** the system compresses highly repetitive data (e.g., 10,000 identical bytes)
- **THEN** the compressed output SHALL be smaller than the original input

#### Scenario: Compress empty data

- **WHEN** the system compresses zero-length input
- **THEN** the system SHALL produce valid zstd output that decompresses back to zero-length data

### Requirement: Zstd decompression

The system SHALL decompress zstd-compressed data to restore the original plaintext during decryption.

#### Scenario: Decompress valid zstd data

- **WHEN** the system decompresses valid zstd-compressed data
- **THEN** the output SHALL be byte-for-byte identical to the data before compression

#### Scenario: Decompress invalid zstd data

- **WHEN** the system attempts to decompress data that is not valid zstd
- **THEN** the system SHALL return a `CompressionError`

### Requirement: Zstd magic bytes detection

The system SHALL detect whether input data is already zstd-compressed by checking for the zstd magic bytes `0x28 0xB5 0x2F 0xFD` at the beginning of the data.

#### Scenario: Detect zstd-compressed data

- **WHEN** the input data begins with the bytes `0x28 0xB5 0x2F 0xFD`
- **THEN** the detection function SHALL return `true`

#### Scenario: Detect non-zstd data

- **WHEN** the input data does not begin with the bytes `0x28 0xB5 0x2F 0xFD`
- **THEN** the detection function SHALL return `false`

#### Scenario: Data shorter than 4 bytes

- **WHEN** the input data is shorter than 4 bytes
- **THEN** the detection function SHALL return `false`

#### Scenario: Empty data

- **WHEN** the input data is zero-length
- **THEN** the detection function SHALL return `false`

### Requirement: Smart compression skip

During encryption, the system SHALL check whether the input data is already zstd-compressed. If it is, the system SHALL skip compression and set the `ALREADY_ZSTD` flag in the file header. This prevents double compression which wastes CPU time and can increase file size.

#### Scenario: Already-compressed data skips compression

- **WHEN** the input data is detected as already zstd-compressed (starts with zstd magic bytes)
- **THEN** the system SHALL NOT compress the data, SHALL set the `ALREADY_ZSTD` flag (bit 0) in the header flags byte, and SHALL encrypt the data as-is

#### Scenario: Non-compressed data is compressed

- **WHEN** the input data is not detected as zstd-compressed
- **THEN** the system SHALL compress the data with zstd level 3 before encryption, and SHALL NOT set the `ALREADY_ZSTD` flag

### Requirement: Conditional decompression during decryption

During decryption, the system SHALL check the `ALREADY_ZSTD` flag in the file header. If the flag is set, the system SHALL skip decompression and return the decrypted data directly. If the flag is not set, the system SHALL decompress the decrypted data.

#### Scenario: Decompress when ALREADY_ZSTD flag is not set

- **WHEN** the `ALREADY_ZSTD` flag is not set in the header
- **THEN** the system SHALL decompress the decrypted data using zstd before writing the output

#### Scenario: Skip decompression when ALREADY_ZSTD flag is set

- **WHEN** the `ALREADY_ZSTD` flag is set in the header
- **THEN** the system SHALL write the decrypted data directly without decompression

### Requirement: Large data compression support

The system SHALL support compression and decompression of data up to at least 1 MB in a single operation.

#### Scenario: Compress and decompress 1 MB data

- **WHEN** the system compresses and then decompresses 1 MB of data
- **THEN** the decompressed output SHALL be byte-for-byte identical to the original input
