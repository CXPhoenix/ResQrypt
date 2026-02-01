# 🔐 ResQrypt

[![CI](https://github.com/cxphoenix/ResQrypt/actions/workflows/ci.yml/badge.svg)](https://github.com/cxphoenix/ResQrypt/actions/workflows/ci.yml)
[![Release](https://img.shields.io/github/v/release/cxphoenix/ResQrypt?display_name=tag&style=flat)](https://github.com/cxphoenix/ResQrypt/releases)
[![Docker](https://img.shields.io/badge/ghcr.io-ResQrypt-blue?logo=docker)](https://github.com/cxphoenix/ResQrypt/pkgs/container/resqrypt)
[![License](https://img.shields.io/badge/license-ECL--2.0-green)](LICENSE)

**安全的檔案/目錄加密工具** — 結合 **AES-256-GCM** 認證加密、**Argon2id** 密鑰派生、**zstd** 智慧壓縮

---

## ✨ 功能特色

| 功能 | 說明 |
|------|------|
| 🔒 **AES-256-GCM** | 業界標準認證加密演算法 |
| 🔑 **Argon2id** | 抗暴力破解的記憶體硬密鑰派生函數 |
| 🗜️ **zstd 壓縮** | 高效壓縮，壓縮率可達 90%+ |
| 🧠 **智慧偵測** | 已壓縮的 zstd 檔案不會重複壓縮 |
| 📁 **目錄支援** | 整個資料夾打包加密 |
| 🖥️ **跨平台** | Windows、macOS、Linux |
| 🐳 **Docker** | 多架構 image (amd64/arm64) |

---

## 📦 安裝

### 從 GitHub Releases 下載

前往 [Releases](https://github.com/cxphoenix/ResQrypt/releases) 下載對應平台的執行檔：

```bash
# macOS (Apple Silicon)
curl -LO https://github.com/cxphoenix/ResQrypt/releases/latest/download/resqrypt-aarch64-apple-darwin.tar.gz
tar -xzf resqrypt-*.tar.gz && sudo mv resqrypt /usr/local/bin/

# Linux (x86_64)
curl -LO https://github.com/cxphoenix/ResQrypt/releases/latest/download/resqrypt-x86_64-unknown-linux-gnu.tar.gz
tar -xzf resqrypt-*.tar.gz && sudo mv resqrypt /usr/local/bin/
```

### 使用 Cargo 編譯

```bash
cargo install --git https://github.com/cxphoenix/ResQrypt
```

### 使用 Docker

```bash
docker pull ghcr.io/cxphoenix/resqrypt:latest
docker run --rm -v $(pwd):/data ghcr.io/cxphoenix/resqrypt:latest encrypt -i /data/secret.txt -o /data/secret.resqrypt
```

---

## 🚀 快速開始

### 加密

```bash
# 加密檔案（會提示輸入密碼）
resqrypt encrypt -i secret.txt -o secret.resqrypt

# 加密整個目錄
resqrypt encrypt -i ./my-secrets/ -o backup.resqrypt

# 使用環境變數設定密碼（適合腳本）
RESQRYPT_PASSWORD="mypassword" resqrypt encrypt -i file.txt -o file.resqrypt

# 高安全性參數
resqrypt encrypt -i file.txt -o file.resqrypt --argon2-memory 128 --argon2-iterations 5
```

### 解密

```bash
resqrypt decrypt -i secret.resqrypt -o secret.txt
resqrypt decrypt -i backup.resqrypt -o ./restored/
```

---

## ⚙️ CLI 參數

### `resqrypt encrypt`

| 參數 | 說明 | 預設值 |
|------|------|--------|
| `-i, --input` | 輸入檔案或目錄 | *必填* |
| `-o, --output` | 輸出 `.resqrypt` 檔案 | *必填* |
| `-p, --password` | 加密密碼 | *提示輸入* |
| `--argon2-memory` | 記憶體成本 (MB) | 64 |
| `--argon2-iterations` | 迭代次數 | 3 |
| `--argon2-parallelism` | 平行度 | 4 |
| `-v, --verbose` | 顯示詳細資訊 | false |

### `resqrypt decrypt`

| 參數 | 說明 | 預設值 |
|------|------|--------|
| `-i, --input` | 輸入 `.resqrypt` 檔案 | *必填* |
| `-o, --output` | 輸出檔案或目錄 | *必填* |
| `-p, --password` | 解密密碼 | *提示輸入* |
| `-v, --verbose` | 顯示詳細資訊 | false |

---

## 🔒 安全設計

| 項目 | 說明 |
|------|------|
| **密碼處理** | 密碼永不儲存，僅用於派生金鑰 |
| **Argon2id** | 記憶體硬函數，抵抗 GPU/ASIC 攻擊 |
| **AES-256-GCM** | 認證加密，可偵測資料篡改 |
| **隨機 Salt/Nonce** | 每次加密皆使用全新隨機值 |
| **無 Metadata 洩漏** | 檔案內容與結構皆被加密 |

---

## 📊 檔案格式

```
+------------------+--------+----------------------------------+
| Magic (8 bytes)  | RESQRYPT                          |
| Version (1 byte) | 0x01                              |
| Flags (1 byte)   | 壓縮/目錄標記                       |
| KDF Params (12)  | Argon2id 參數 (memory/time/para)  |
| Salt (32 bytes)  | 隨機 salt                         |
| Nonce (12 bytes) | AES-GCM nonce                     |
| Encrypted Data   | payload + 16-byte auth tag        |
+------------------+--------+----------------------------------+
```

---

## 📜 授權

本專案採用 [Educational Community License v2.0 (ECL-2.0)](LICENSE)

---

## 🙏 致謝

感謝以下開源專案：

- [aes-gcm](https://crates.io/crates/aes-gcm) - AES-256-GCM 實作
- [argon2](https://crates.io/crates/argon2) - Argon2id 實作
- [zstd](https://crates.io/crates/zstd) - zstd 壓縮
- [clap](https://crates.io/crates/clap) - CLI 解析
