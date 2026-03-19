use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;

#[derive(Debug, Serialize, Deserialize)]
struct SignatureInfo {
    valid: bool,
    warning: bool,
    name: Option<String>,
    date: Option<String>,
    details: Option<String>,
}

#[tauri::command]
async fn verify_pdf_signatures(file_path: String) -> Result<Vec<SignatureInfo>, String> {
    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("无法打开文件: {}", e)),
    };
    
    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        return Err(format!("读取文件失败: {}", e));
    }
    
    let signatures = extract_and_verify_signatures(&buffer);
    
    Ok(signatures)
}

fn extract_and_verify_signatures(_pdf_data: &[u8]) -> Vec<SignatureInfo> {
    vec![]
}

#[tauri::command]
async fn get_pdf_properties(file_path: String) -> Result<serde_json::Value, String> {
    let mut file = match File::open(&file_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("无法打开文件: {}", e)),
    };
    
    let mut buffer = Vec::new();
    if let Err(e) = file.read_to_end(&mut buffer) {
        return Err(format!("读取文件失败: {}", e));
    }
    
    let properties = serde_json::json!({
        "fileSize": buffer.len(),
        "pages": 0
    });
    
    Ok(properties)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
  tauri::Builder::default()
    .plugin(tauri_plugin_dialog::init())
    .plugin(tauri_plugin_fs::init())
    .plugin(tauri_plugin_shell::init())
    .invoke_handler(tauri::generate_handler![
        verify_pdf_signatures,
        get_pdf_properties
    ])
    .setup(|app| {
      if cfg!(debug_assertions) {
        app.handle().plugin(
          tauri_plugin_log::Builder::default()
            .level(log::LevelFilter::Info)
            .build(),
        )?;
      }
      Ok(())
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
