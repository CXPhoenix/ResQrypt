## 1. 檔案格式規格（file-format）

- [x] 1.1 建立 `openspec/specs/file-format/spec.md`，涵蓋 magic bytes identification、format version field、flags byte、KDF parameters in header、salt field、nonce field、fixed header size、header roundtrip consistency 等所有需求
- [x] 1.2 驗證 file-format spec 與 `src/crypto/format.rs` 及 `src/lib.rs` 中的實作一致

## 2. 加密規格（encryption）

- [x] 2.1 建立 `openspec/specs/encryption/spec.md`，涵蓋 AES-256-GCM authenticated encryption、nonce generation、decryption with authentication verification、ciphertext minimum length validation、large data support 等所有需求
- [x] 2.2 驗證 encryption spec 與 `src/crypto/aes.rs` 中的實作一致

## 3. 金鑰衍生規格（key-derivation）

- [x] 3.1 建立 `openspec/specs/key-derivation/spec.md`，涵蓋 Argon2id algorithm selection、default KDF parameters、custom KDF parameters、random salt generation、deterministic key derivation、output key length、KDF parameter validation 等所有需求
- [x] 3.2 驗證 key-derivation spec 與 `src/crypto/kdf.rs` 中的實作一致

## 4. 壓縮規格（compression）

- [x] 4.1 建立 `openspec/specs/compression/spec.md`，涵蓋 zstd compression、zstd decompression、zstd magic bytes detection、smart compression skip、conditional decompression during decryption、large data compression support 等所有需求
- [x] 4.2 驗證 compression spec 與 `src/compression/zstd.rs` 及 `src/compression/detect.rs` 中的實作一致

## 5. 目錄打包規格（directory-archival）

- [x] 5.1 建立 `openspec/specs/directory-archival/spec.md`，涵蓋 tar archive creation from directory、directory validation、symbolic link handling、tar archive extraction、archive roundtrip consistency、IS_DIRECTORY flag integration、file I/O utilities 等所有需求
- [x] 5.2 驗證 directory-archival spec 與 `src/archive/tar.rs` 中的實作一致

## 6. CLI 介面規格（cli-interface）

- [x] 6.1 建立 `openspec/specs/cli-interface/spec.md`，涵蓋 encrypt subcommand、decrypt subcommand、interactive password prompt for encryption、interactive password prompt for decryption、input validation、output overwrite protection、verbose output mode、success message、exit code 等所有需求
- [x] 6.2 驗證 cli-interface spec 與 `src/cli.rs`、`src/main.rs`、`src/commands/encrypt.rs`、`src/commands/decrypt.rs` 中的實作一致

## 7. 錯誤處理規格（error-handling）

- [x] 7.1 建立 `openspec/specs/error-handling/spec.md`，涵蓋 error type enumeration、Io error auto-conversion、user-facing error messages、result type alias、error context preservation 等所有需求
- [x] 7.2 驗證 error-handling spec 與 `src/error.rs` 中的實作一致
