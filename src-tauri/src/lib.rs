mod commands;
mod languages;
mod types;
mod whisper;

use tauri_plugin_log::{Target, TargetKind};

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
                    Target::new(TargetKind::Stdout),
                    Target::new(TargetKind::LogDir { file_name: None }),
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
    let models_path: &std::path::Path = std::path::Path::new("models");
    if !models_path.exists() {
        let _ = std::fs::create_dir(models_path);
    }
}
