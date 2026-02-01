# 🔐 Resqrypt

**Secure file and directory encryption tool**

Resqrypt uses **zstd** compression and **AES-256-GCM** encryption with **Argon2id** key derivation to securely protect your sensitive files.

## ✨ Features

- 🔒 **AES-256-GCM** authenticated encryption
- 🗜️ **zstd** high-performance compression
- 🔑 **Argon2id** password-based key derivation (configurable parameters)
- 📁 **Directory support** - encrypt entire folders
- 🧠 **Smart detection** - avoids double-compressing already-compressed files
- 🖥️ **Cross-platform** - Windows, macOS, Linux
- 🐳 **Docker** - available on GitHub Packages

## 📦 Installation

### From GitHub Releases

Download the latest binary for your platform from [Releases](https://github.com/cxphoenix/resqrypt/releases).

### Using Cargo

```bash
cargo install resqrypt
```

### Using Docker

```bash
docker pull ghcr.io/cxphoenix/resqrypt:latest
```

## 🚀 Usage

### Encrypt

```bash
# Encrypt a file (prompts for password)
resqrypt encrypt -i secret.txt -o secret.txt.resqrypt

# Encrypt a directory
resqrypt encrypt -i ./my-secrets/ -o backup.resqrypt

# Use environment variable for password
RESQRYPT_PASSWORD="mypassword" resqrypt encrypt -i file.txt -o file.resqrypt

# Custom Argon2id parameters (higher security)
resqrypt encrypt -i file.txt -o file.resqrypt \
    --argon2-memory 128 --argon2-iterations 5
```

### Decrypt

```bash
resqrypt decrypt -i secret.txt.resqrypt -o secret.txt
resqrypt decrypt -i backup.resqrypt -o ./restored/
```

## ⚙️ CLI Options

### Encrypt

| Option | Description | Default |
|--------|-------------|---------|
| `-i, --input` | Input file or directory | *required* |
| `-o, --output` | Output .resqrypt file | *required* |
| `-p, --password` | Encryption password | *prompt* |
| `--argon2-memory` | Memory cost (MB) | 64 |
| `--argon2-iterations` | Iteration count | 3 |
| `--argon2-parallelism` | Parallelism degree | 4 |
| `-v, --verbose` | Verbose output | false |

### Decrypt

| Option | Description | Default |
|--------|-------------|---------|
| `-i, --input` | Input .resqrypt file | *required* |
| `-o, --output` | Output file or directory | *required* |
| `-p, --password` | Decryption password | *prompt* |
| `-v, --verbose` | Verbose output | false |

## 🔒 Security

- **Password-based encryption**: Your password is never stored
- **Argon2id**: Memory-hard KDF resistant to GPU/ASIC attacks
- **AES-256-GCM**: Authenticated encryption (AEAD)
- **Random salt/nonce**: Unique for each encryption
- **No metadata leakage**: File content and structure are encrypted

## 📜 License

Educational Community License v2.0 (ECL-2.0)
