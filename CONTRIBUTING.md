# 🤝 貢獻指南

感謝您對 ResQrypt 的關注！歡迎提交 Issue 和 Pull Request。

---

## 🐛 回報問題

在提交 Issue 之前，請先確認：

1. 搜尋現有 Issue，確認問題尚未被回報
2. 使用最新版本測試問題是否仍然存在

提交 Issue 時請包含：

- 作業系統和版本
- ResQrypt 版本（`resqrypt --version`）
- 重現問題的步驟
- 預期行為與實際行為
- 相關錯誤訊息

---

## 🔧 開發環境設定

### 需求

- Rust 1.75+
- Git

### 設定步驟

```bash
# Clone 專案
git clone https://github.com/cxphoenix/ResQrypt.git
cd ResQrypt

# 編譯
cargo build

# 執行測試
cargo test

# 執行 lint
cargo clippy -- -D warnings
cargo fmt --check
```

---

## 📝 Commit 規範

使用 [Conventional Commits](https://www.conventionalcommits.org/zh-hant/) 格式：

```
<類型>(<範圍>): <描述>

[可選的內文]

[可選的頁尾]
```

### 類型

| 類型 | 說明 |
|------|------|
| `feat` | 新功能 |
| `fix` | 錯誤修復 |
| `docs` | 文件更新 |
| `style` | 程式碼風格（不影響功能） |
| `refactor` | 重構（不新增功能或修復錯誤） |
| `perf` | 效能改善 |
| `test` | 測試相關 |
| `ci` | CI 配置 |
| `chore` | 其他雜項 |

### 範例

```
feat(cli): 新增 --quiet 參數

fix(crypto): 修正解密時 KDF 參數讀取錯誤

docs: 更新 README 安裝說明
```

---

## 🔀 Pull Request 流程

1. Fork 本專案
2. 建立功能分支：`git checkout -b feat/my-feature`
3. 提交變更：`git commit -m 'feat: 新增功能'`
4. 推送分支：`git push origin feat/my-feature`
5. 建立 Pull Request

### PR 檢查清單

- [ ] 程式碼通過 `cargo fmt --check`
- [ ] 程式碼通過 `cargo clippy -- -D warnings`
- [ ] 所有測試通過 `cargo test`
- [ ] 新增適當的測試
- [ ] 更新相關文件

---

## 📜 授權

提交貢獻即表示您同意將程式碼以 [ECL-2.0](LICENSE) 授權釋出。
