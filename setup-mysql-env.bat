@echo off
REM MySQL Connector/C 環境變數設定腳本
REM 請先下載並解壓 MySQL Connector/C 到指定目錄

REM 設定 MySQL Connector/C 的路徑（請根據您的實際路徑修改）
set MYSQL_DIR=C:\mysql-connector-c

REM 如果 MySQL Connector/C 不存在，提示用戶
if not exist "%MYSQL_DIR%" (
    echo 錯誤: 找不到 MySQL Connector/C 目錄: %MYSQL_DIR%
    echo.
    echo 請執行以下步驟:
    echo 1. 從 https://downloads.mysql.com/archives/c-c/ 下載 MySQL Connector/C
    echo 2. 解壓到 C:\mysql-connector-c (或修改此腳本中的 MYSQL_DIR 變數)
    echo 3. 重新執行此腳本
    pause
    exit /b 1
)

REM 設定環境變數
set MYSQLCLIENT_LIB_DIR=%MYSQL_DIR%\lib
set MYSQLCLIENT_VERSION=8.0
set PATH=%MYSQL_DIR%\lib;%PATH%

echo MySQL Connector/C 環境變數已設定:
echo MYSQLCLIENT_LIB_DIR=%MYSQLCLIENT_LIB_DIR%
echo MYSQLCLIENT_VERSION=%MYSQLCLIENT_VERSION%
echo.
echo 請在此命令提示字元視窗中執行 cargo build 或 cargo run
echo 注意: 這些環境變數只在當前命令提示字元視窗中有效
pause

