# Rust Rocket MVC 電商網站

基於 Rust + Rocket 框架的 MVC 架構電商網站。

## 功能

- 使用者認證（註冊、登入、登出）
- 產品管理（CRUD、搜尋、篩選、分頁）
- 產品類別管理
- 購物車功能
- 訂單管理
- 圖片上傳

## 技術棧

- **後端**: Rust + Rocket 0.5
- **資料庫**: MySQL/MariaDB
- **ORM**: Diesel
- **模板引擎**: Tera
- **前端**: Bootstrap 5, jQuery, DataTables

## 設定

1. 安裝 Rust 和 Cargo
2. 安裝 MySQL/MariaDB
3. 安裝 Diesel CLI: `cargo install diesel_cli --no-default-features --features mysql`
4. 複製 `.env.example` 為 `.env` 並設定資料庫連接字串：
   ```bash
   cp .env.example .env
   ```
   編輯 `.env` 檔案，設定您的資料庫連接資訊：
   ```
   DATABASE_URL=mysql://使用者名稱:密碼@主機:埠號/資料庫名稱
   ```
   例如：
   ```
   DATABASE_URL=mysql://root:mypassword@localhost:3306/shopping
   ```
5. 執行資料庫遷移: `diesel migration run`
6. 執行應用程式: `cargo run`

## 專案結構

```
shopping/
├── src/
│   ├── models/          # Model 層
│   ├── controllers/     # Controller 層
│   ├── views/           # View 層（Tera 模板）
│   ├── middleware/      # 中介層
│   └── utils/           # 工具函數
├── migrations/          # 資料庫遷移
└── static/              # 靜態資源
```

## 資料庫設定

使用 `.env` 檔案設定資料庫連接（推薦方式）：

1. 複製 `.env.example` 為 `.env`：
   ```bash
   cp .env.example .env
   ```

2. 編輯 `.env` 檔案，設定您的資料庫連接資訊：
   ```
   DATABASE_URL=mysql://使用者名稱:密碼@主機:埠號/資料庫名稱
   ```

3. 應用程式啟動時會自動載入 `.env` 檔案中的環境變數

**注意**: `.env` 檔案已加入 `.gitignore`，不會被提交到版本控制系統，確保資料庫密碼等敏感資訊的安全性。

