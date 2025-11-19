#!/bin/bash
# MySQL Connector/C 環境變數設定腳本 (Git Bash)
# 請先下載並解壓 MySQL Connector/C 到指定目錄

# 設定 MySQL Connector/C 的路徑（請根據您的實際路徑修改）
MYSQL_DIR="/c/mysql-connector-c"

# 如果 MySQL Connector/C 不存在，提示用戶
if [ ! -d "$MYSQL_DIR" ]; then
    echo "錯誤: 找不到 MySQL Connector/C 目錄: $MYSQL_DIR"
    echo ""
    echo "請執行以下步驟:"
    echo "1. 從 https://downloads.mysql.com/archives/c-c/ 下載 MySQL Connector/C"
    echo "2. 解壓到 C:\\mysql-connector-c (或修改此腳本中的 MYSQL_DIR 變數)"
    echo "3. 重新執行此腳本"
    exit 1
fi

# 設定環境變數
export MYSQLCLIENT_LIB_DIR="$MYSQL_DIR/lib"
export MYSQLCLIENT_VERSION="8.0"
export PATH="$MYSQL_DIR/lib:$PATH"

echo "MySQL Connector/C 環境變數已設定:"
echo "MYSQLCLIENT_LIB_DIR=$MYSQLCLIENT_LIB_DIR"
echo "MYSQLCLIENT_VERSION=$MYSQLCLIENT_VERSION"
echo ""
echo "請在此終端視窗中執行 cargo build 或 cargo run"
echo "注意: 這些環境變數只在當前終端視窗中有效"

