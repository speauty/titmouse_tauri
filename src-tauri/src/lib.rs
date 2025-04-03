mod commands;
mod languages;
mod types;
mod whisper;


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(
            tauri_plugin_log::Builder::new()
                .targets([
                    // Target::new(TargetKind::Stdout),
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Folder{path: std::path::Path::new("logs").to_path_buf(), file_name: Some(String::from("titmouse"))}),
                ])
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::languages,
            commands::models,
            commands::graphics,
            commands::action_check_video_path_is_valid,
            commands::action_upload_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

fn init() {
    match std::env::current_exe() {
        Ok(path) => {
            let logs_path: std::path::PathBuf = path.parent().unwrap().join("logs");
            if !logs_path.exists() {
                let _ = std::fs::create_dir(logs_path);
            }

            let models_path: std::path::PathBuf = path.parent().unwrap().join("models");
            if !models_path.exists() {
                let _ = std::fs::create_dir(models_path);
            }
        }
        Err(e) => {
            log::error!("创建目录失败, 获取可执行文件路径异常: {}", e);
        }
    };
}
