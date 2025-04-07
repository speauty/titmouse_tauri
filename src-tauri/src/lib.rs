use tauri::Manager;

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
                    tauri_plugin_log::Target::new(tauri_plugin_log::TargetKind::Folder {
                        path: std::path::Path::new("logs").to_path_buf(),
                        file_name: Some(String::from("titmouse")),
                    }),
                ])
                .build(),
        )
        .setup(|app: &mut tauri::App| {
            let m_show = tauri::menu::MenuItem::with_id(app, "show", "显示", true, None::<&str>)?;
            let m_hide = tauri::menu::MenuItem::with_id(app, "hide", "隐藏", true, None::<&str>)?;
            let m_quit = tauri::menu::MenuItem::with_id(app, "quit", "退出", true, None::<&str>)?;
            let menu = tauri::menu::Menu::with_items(app, &[&m_show, &m_hide, &m_quit])?;
            tauri::tray::TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id.as_ref() {
                    "quit" => {
                        log::info!("退出");
                        app.exit(0);
                    }
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            log::info!("显示");
                            let _ = window.show();
                            if window.is_minimized().is_ok() {
                                let _ = window.unminimize();
                            }
                            let _ = window.set_focus();
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            log::info!("隐藏");
                            let _ = window.hide();
                        }
                    }
                    _ => {
                        log::error!("未知菜单: {:?}", event.id);
                    }
                })
                .on_tray_icon_event(|tray: &tauri::tray::TrayIcon, event: tauri::tray::TrayIconEvent| match event {
                    tauri::tray::TrayIconEvent::Click {
                        button: tauri::tray::MouseButton::Left,
                        button_state: tauri::tray::MouseButtonState::Up,
                        ..
                    } => {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            log::info!("显示");
                            let _ = window.show();
                            if window.is_minimized().is_ok() {
                                let _ = window.unminimize();
                            }
                            let _ = window.set_focus();
                        }
                    }
                    _ => {
                        log::error!("未知事件: {:?}", event.id());
                    },
                })
                .build(app)?;
            Ok(())
        })
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
