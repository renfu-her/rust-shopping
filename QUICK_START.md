# 快速解決 MySQL 客戶端庫問題

## 問題
編譯時出現錯誤：`Did not find a compatible version of libmysqlclient`

## 解決方案（3 步驟）

### 步驟 1: 下載 MySQL Connector/C

1. 訪問：https://downloads.mysql.com/archives/c-c/
2. 選擇版本：**8.0.x** (例如 8.0.40)
3. 選擇平台：**Windows (x86, 64-bit), ZIP Archive**
4. 下載 ZIP 檔案

### 步驟 2: 解壓縮

將下載的 ZIP 檔案解壓到：
```
C:\mysql-connector-c
```

**重要**: 確保解壓後的目錄結構包含 `lib` 和 `include` 資料夾

### 步驟 3: 設定環境變數並編譯

在 Git Bash 中執行：

```bash
# 方法 1: 使用自動檢測腳本
source quick-setup-mysql.sh
cargo build

# 方法 2: 手動設定
export MYSQLCLIENT_LIB_DIR="C:/mysql-connector-c/lib"
export MYSQLCLIENT_VERSION="8.0"
cargo build
```

## 驗證

執行後應該看到環境變數已設定：
```bash
echo $MYSQLCLIENT_LIB_DIR
echo $MYSQLCLIENT_VERSION
```

## 如果仍然失敗

1. **檢查路徑是否正確**：
   ```bash
   ls "C:/mysql-connector-c/lib" | grep -i "\.lib"
   ```
   應該看到 `libmysqlclient.lib` 或類似的檔案

2. **檢查版本號**：
   如果使用 MySQL 8.4，嘗試：
   ```bash
   export MYSQLCLIENT_VERSION="8.4"
   ```

3. **使用完整路徑**：
   確保使用 Windows 風格的路徑（使用 `/` 或 `\\`，不要使用 `\`）

## 永久設定（可選）

要永久設定環境變數，將以下內容添加到 `~/.bashrc`：
```bash
export MYSQLCLIENT_LIB_DIR="C:/mysql-connector-c/lib"
export MYSQLCLIENT_VERSION="8.0"
```

然後執行：
```bash
source ~/.bashrc
```

