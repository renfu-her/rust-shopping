# MySQL 客戶端庫設定指南 (Windows)

由於在 Windows 上編譯 Rust 專案時需要 MySQL C 客戶端庫，以下是幾種解決方案：

## 方案 1: 下載 MySQL Connector/C (推薦)

### 步驟：

1. **下載 MySQL Connector/C**
   - 訪問: https://downloads.mysql.com/archives/c-c/
   - 選擇適合的版本（建議 8.0.x）
   - 下載 Windows (x86, 64-bit), ZIP Archive

2. **解壓縮**
   - 解壓到 `C:\mysql-connector-c` (或您選擇的其他目錄)

3. **設定環境變數**
   
   在 Git Bash 中執行：
   ```bash
   source setup-mysql-env.sh
   cargo build
   ```
   
   或在 Windows CMD 中執行：
   ```cmd
   setup-mysql-env.bat
   cargo build
   ```

   或者手動設定環境變數：
   ```bash
   export MYSQLCLIENT_LIB_DIR="C:/mysql-connector-c/lib"
   export MYSQLCLIENT_VERSION="8.0"
   ```

## 方案 2: 使用 vcpkg (如果方案 1 失敗)

1. **設定 VCPKG_ROOT 環境變數**
   ```bash
   export VCPKG_ROOT="C:/Users/zivhs/vcpkg"
   ```

2. **嘗試安裝 libmariadb (通常比 libmysql 更穩定)**
   ```bash
   cd ~/vcpkg
   ./vcpkg install libmariadb:x64-windows
   ```

3. **如果成功，設定環境變數**
   ```bash
   export VCPKG_ROOT="C:/Users/zivhs/vcpkg"
   ```

## 方案 3: 使用 Laragon MySQL (如果包含開發庫)

如果您的 Laragon MySQL 安裝包含開發庫：

```bash
export MYSQLCLIENT_LIB_DIR="D:/laragon/bin/mysql/mysql-8.4.3-winx64/lib"
export MYSQLCLIENT_VERSION="8.4"
```

**注意**: Laragon 的標準安裝通常不包含開發庫，所以此方案可能不適用。

## 驗證設定

設定環境變數後，執行：
```bash
cargo build
```

如果仍然出現錯誤，請檢查：
- 環境變數是否正確設定 (`echo $MYSQLCLIENT_LIB_DIR`)
- MySQL Connector/C 的 lib 目錄中是否包含 `.lib` 或 `.dll` 檔案
- 版本號是否正確

## 常見問題

**Q: 如何確認環境變數已設定？**
A: 在終端執行 `echo $MYSQLCLIENT_LIB_DIR` (Git Bash) 或 `echo %MYSQLCLIENT_LIB_DIR%` (CMD)

**Q: 環境變數只在當前終端有效嗎？**
A: 是的，如果使用 `export` 或批次檔。要永久設定，請在系統環境變數中設定。

**Q: 可以使用 MariaDB Connector 嗎？**
A: 可以，MariaDB Connector/C 與 MySQL 相容，且通常在 Windows 上更容易安裝。

