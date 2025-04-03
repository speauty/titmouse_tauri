#[tauri::command]
pub fn languages() -> Vec<crate::languages::Language> {
    crate::languages::LANGUAGES.to_vec()
}

#[tauri::command]
pub fn models() -> Vec<crate::whisper::Model> {
    crate::whisper::create_model_directory();
    crate::whisper::get_model_files()
}

#[tauri::command]
pub fn graphics(app: tauri::AppHandle) -> Vec<String> {
    crate::whisper::get_graphics(app)
}

#[tauri::command]
pub fn action_check_video_path_is_valid(path_video: String) -> String {
    let path: &std::path::Path = std::path::Path::new(&path_video);
    if !path.exists() {
        return format!("目标视频不存在({})", path.display());
    }

    if let Some(extension) = path.extension() {
        if extension != "mp4" {
            return format!("目标视频({})非mp4有效格式", path.display());
        }
    }

    String::new()
}

#[tauri::command]
pub async fn action_upload_task(
    app: tauri::AppHandle,
    task: crate::types::WhisperTranformTask,
    on_event: tauri::ipc::Channel<crate::types::WhisperTranformTaskEvent>,
) {
    crate::whisper::tranform_task(app, task, on_event);
}
