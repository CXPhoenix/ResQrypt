# 📝 Changelog

本專案遵循 [Semantic Versioning](https://semver.org/lang/zh-TW/)。

---

## [0.1.1] - 2026-02-01

### 🐛 錯誤修復

- **CI**: 移除 macOS x86_64 (`x86_64-apple-darwin`) 支援，因 GitHub Runner 已棄用 `macos-13`
- **CI**: 修正 `aarch64-unknown-linux-gnu` 交叉編譯問題，改用 `cross`

### 📝 文件更新

- 更新 `README.md` 移除 macOS Intel 支援說明

## [0.1.0] - 2026-02-01

### ✨ 新增功能

- 🔐 **AES-256-GCM 認證加密**
  - 業界標準對稱加密演算法
  - 可偵測資料篡改
  
- 🔑 **Argon2id 密鑰派生**
  - 記憶體硬函數，抵抗 GPU/ASIC 攻擊
  - 可調整參數：`--argon2-memory`、`--argon2-iterations`、`--argon2-parallelism`
  
- 🗜️ **zstd 智慧壓縮**
  - 高效壓縮，壓縮率可達 90%+
  - 智慧偵測：已壓縮的 zstd 檔案不會重複壓縮

- 📁 **檔案與目錄支援**
  - 單一檔案加密
  - 整個目錄打包加密（使用 tar 封存）

- ⌨️ **CLI 介面**
  - `resqrypt encrypt` - 加密命令
  - `resqrypt decrypt` - 解密命令
  - 支援環境變數 `RESQRYPT_PASSWORD`

- 🖥️ **跨平台支援**
  - Linux (x86_64, x86_64-musl, aarch64)
  - macOS (x86_64, Apple Silicon)
  - Windows (x86_64)

- 🐳 **Docker 支援**
  - Multi-arch image (amd64/arm64)
  - 可從 `ghcr.io` 取得

- 🚀 **CI/CD**
  - GitHub Actions 多平台測試
  - 自動建構與發布 Release
  - Docker image 自動推送到 ghcr.io

### 📊 檔案格式

- Version: `0x01`
- Header: 66 bytes (Magic + Version + Flags + KDF Params + Salt + Nonce)
- 所有數值使用 little-endian 編碼
