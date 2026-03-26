## Why

ResQrypt 專案目前缺乏正式的規格文件（specs）。所有功能行為僅存在於原始碼中，沒有可獨立閱讀的規格定義。建立完整的 specs 能讓未來的開發、審查與貢獻者快速理解每個模組的職責、介面契約與行為保證，也為後續的 Spectra 變更流程奠定基礎。

## What Changes

- 為 ResQrypt 的所有核心功能建立正式的 spec 文件，涵蓋：
  - 自訂 `.resqrypt` 檔案格式（magic bytes、header 結構、版本控制）
  - AES-256-GCM 加密與解密流程
  - Argon2id 密碼衍生金鑰（KDF）機制
  - zstd 智慧壓縮與偵測邏輯
  - 目錄打包（tar archive）支援
  - CLI 介面定義（指令、參數、環境變數）
  - 錯誤處理模型（錯誤類型與使用者回饋）
- 不修改任何現有程式碼，僅新增 `openspec/specs/` 下的規格文件

## Non-Goals

- 不重構或修改任何現有實作程式碼
- 不新增功能或變更行為
- 不涵蓋 CI/CD pipeline 或 Docker 配置的規格
- 不處理未來規劃中的功能（如串流加密、多檔案批次處理等）

## Capabilities

### New Capabilities

- `file-format`: ResQrypt 自訂檔案格式規格 — magic bytes、header 結構（66 bytes）、版本號、flags 定義、欄位佈局與位元組順序
- `encryption`: AES-256-GCM 認證加密與解密流程 — nonce 生成、加密/解密操作、authentication tag 驗證、資料完整性保證
- `key-derivation`: Argon2id 密碼衍生金鑰機制 — salt 生成、預設參數（64MB/3 iterations/4 parallelism）、自訂參數支援、密碼處理規則
- `compression`: zstd 智慧壓縮系統 — 壓縮/解壓縮操作、magic bytes 偵測、避免重複壓縮邏輯、壓縮等級設定
- `directory-archival`: 目錄加密支援 — tar archive 建立與解壓、目錄結構保留、與加密 pipeline 的整合
- `cli-interface`: 命令列介面定義 — encrypt/decrypt 子命令、參數規格、環境變數（`RESQRYPT_PASSWORD`）、密碼互動式輸入、verbose 模式
- `error-handling`: 錯誤處理模型 — 錯誤類型定義（9 種變體）、使用者可見的錯誤訊息、退出碼規範

### Modified Capabilities

（無 — 目前不存在任何 specs）

## Impact

- Affected specs: 新增 7 個 spec 目錄於 `openspec/specs/` 下
- Affected code: 無程式碼變更。以下為各 spec 對應的原始碼參考：
  - `file-format` → `src/crypto/format.rs`, `src/lib.rs`
  - `encryption` → `src/crypto/aes.rs`
  - `key-derivation` → `src/crypto/kdf.rs`
  - `compression` → `src/compression/zstd.rs`, `src/compression/detect.rs`
  - `directory-archival` → `src/archive/tar.rs`
  - `cli-interface` → `src/cli.rs`, `src/main.rs`, `src/commands/encrypt.rs`, `src/commands/decrypt.rs`
  - `error-handling` → `src/error.rs`
