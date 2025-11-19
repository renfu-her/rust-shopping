use std::path::PathBuf;
use rocket::fs::TempFile;
use std::fs;
use uuid::Uuid;

pub async fn save_uploaded_file(mut file: TempFile<'_>, upload_dir: &str) -> Result<String, String> {
    // 確保上傳目錄存在
    fs::create_dir_all(upload_dir)
        .map_err(|e| format!("Failed to create upload directory: {}", e))?;

    // 生成唯一檔名
    let extension = file.name()
        .and_then(|n| PathBuf::from(n).extension())
        .and_then(|e| e.to_str())
        .unwrap_or("jpg");
    
    let filename = format!("{}.{}", Uuid::new_v4(), extension);
    let filepath = PathBuf::from(upload_dir).join(&filename);

    // 儲存檔案
    file.persist_to(&filepath).await
        .map_err(|e| format!("Failed to save file: {}", e))?;

    Ok(format!("/static/images/{}", filename))
}

