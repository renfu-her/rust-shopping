#!/bin/bash
# 快速設定 MySQL 客戶端庫環境變數
# 此腳本會嘗試多種方式找到 MySQL 客戶端庫

echo "正在尋找 MySQL 客戶端庫..."

# 方法 1: 檢查 MySQL Connector/C 是否已安裝
MYSQL_PATHS=(
    "/c/mysql-connector-c"
    "C:/mysql-connector-c"
    "/d/mysql-connector-c"
    "D:/mysql-connector-c"
    "$HOME/mysql-connector-c"
)

MYSQL_DIR=""
for path in "${MYSQL_PATHS[@]}"; do
    if [ -d "$path" ] && [ -d "$path/lib" ]; then
        MYSQL_DIR="$path"
        echo "✓ 找到 MySQL Connector/C: $MYSQL_DIR"
        break
    fi
done

# 方法 2: 檢查 Laragon MySQL (通常不包含開發庫，但試試看)
if [ -z "$MYSQL_DIR" ]; then
    LARAGON_PATHS=(
        "/d/laragon/bin/mysql/mysql-8.4.3-winx64"
        "D:/laragon/bin/mysql/mysql-8.4.3-winx64"
        "/c/laragon/bin/mysql/mysql-8.4.3-winx64"
        "C:/laragon/bin/mysql/mysql-8.4.3-winx64"
    )
    
    for path in "${LARAGON_PATHS[@]}"; do
        if [ -d "$path" ] && [ -d "$path/lib" ]; then
            MYSQL_DIR="$path"
            echo "✓ 找到 Laragon MySQL: $MYSQL_DIR"
            break
        fi
    done
fi

# 如果找不到，提供下載指引
if [ -z "$MYSQL_DIR" ]; then
    echo ""
    echo "❌ 未找到 MySQL 客戶端庫！"
    echo ""
    echo "請執行以下步驟："
    echo ""
    echo "1. 下載 MySQL Connector/C:"
    echo "   https://downloads.mysql.com/archives/c-c/"
    echo "   選擇: Windows (x86, 64-bit), ZIP Archive"
    echo ""
    echo "2. 解壓縮到以下任一位置："
    for path in "${MYSQL_PATHS[@]}"; do
        echo "   - $path"
    done
    echo ""
    echo "3. 重新執行此腳本"
    echo ""
    echo "或者，手動設定環境變數："
    echo "   export MYSQLCLIENT_LIB_DIR=\"C:/mysql-connector-c/lib\""
    echo "   export MYSQLCLIENT_VERSION=\"8.0\""
    exit 1
fi

# 設定環境變數
export MYSQLCLIENT_LIB_DIR="$MYSQL_DIR/lib"
export MYSQLCLIENT_VERSION="8.0"

# 檢查 lib 目錄中是否有必要的檔案
if [ -f "$MYSQLCLIENT_LIB_DIR/libmysqlclient.lib" ] || [ -f "$MYSQLCLIENT_LIB_DIR/libmysql.lib" ] || [ -f "$MYSQLCLIENT_LIB_DIR/mysqlclient.lib" ]; then
    echo "✓ 找到 MySQL 客戶端庫檔案"
else
    echo "⚠ 警告: 在 $MYSQLCLIENT_LIB_DIR 中未找到 .lib 檔案"
    echo "   請確認 MySQL Connector/C 已正確安裝"
fi

echo ""
echo "環境變數已設定："
echo "  MYSQLCLIENT_LIB_DIR=$MYSQLCLIENT_LIB_DIR"
echo "  MYSQLCLIENT_VERSION=$MYSQLCLIENT_VERSION"
echo ""
echo "現在可以執行: cargo build"
echo ""
echo "注意: 這些環境變數只在當前終端視窗中有效"
echo "      要永久設定，請將以下內容添加到 ~/.bashrc:"
echo "      export MYSQLCLIENT_LIB_DIR=\"$MYSQLCLIENT_LIB_DIR\""
echo "      export MYSQLCLIENT_VERSION=\"$MYSQLCLIENT_VERSION\""

