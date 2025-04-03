use std::fs::File;
use std::io::{self, Write};
use std::io::{BufRead, BufReader};
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::{Command, Stdio};

use tauri::Manager;

use crate::types::STDSrtLine;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Model {
    pub name: String,
    pub path: String,
}

pub fn create_model_directory() {
    match std::env::current_exe() {
        Ok(path) => {
            let dir_model: std::path::PathBuf = path.parent().unwrap().join("models");
            if !std::fs::metadata(&dir_model).is_ok() {
                if let Err(e) = std::fs::create_dir_all(&dir_model) {
                    log::error!("创建模型目录({:?})异常: {}", dir_model, e);
                } else {
                    log::info!("创建模型目录成功({:?})", dir_model);
                }
            }
        }
        Err(e) => {
            log::error!("创建模型目录失败, 获取可执行文件路径异常: {}", e);
        }
    };
}

pub fn get_model_files() -> Vec<Model> {
    let exe_path = std::env::current_exe().expect("获取可执行文件模型失败");
    let dir_model = exe_path.parent().unwrap().join("models");

    if !std::fs::metadata(&dir_model).is_ok() {
        return vec![];
    }

    let mut model_files: Vec<Model> = Vec::new();
    if let Ok(entries) = std::fs::read_dir(&dir_model) {
        for entry in entries.filter_map(Result::ok) {
            if let Some(ext) = entry.path().extension() {
                if ext == "bin" {
                    if let Some(file_name) = entry.path().file_name() {
                        let name = file_name.to_string_lossy().into_owned();
                        let path = entry.path().to_string_lossy().into_owned();
                        model_files.push(Model { name, path });
                    }
                }
            }
        }
    }
    model_files
}

pub fn get_graphics(app: tauri::AppHandle) -> Vec<String> {
    let whisper_path = app
        .path()
        .resolve("plugins/whisper/", tauri::path::BaseDirectory::Resource)
        .expect("获取可执行文件模型失败")
        .join("whisper.exe");
    let output = std::process::Command::new(whisper_path)
        .arg("-la")
        .creation_flags(0x08000000)
        .output()
        .expect("执行exe失败");

    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stderr.is_empty() {
        log::error!("查询显卡异常, 错误: {:?}", stderr);
        return vec![];
    }
    let adapters: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter_map(|line| {
            let trimmed = line.trim();
            if !trimmed.is_empty() && trimmed.starts_with('"') && trimmed.ends_with('"') {
                Some(trimmed.to_string())
            } else {
                None
            }
        })
        .collect();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    adapters
        .into_iter()
        .filter(|adapter| seen.insert(adapter.clone()))
        .map(|adapter| adapter.trim_matches('"').to_string())
        .collect()
}

pub fn tranform_task(
    app: tauri::AppHandle,
    task: crate::types::WhisperTranformTask,
    on_event: tauri::ipc::Channel<crate::types::WhisperTranformTaskEvent>,
) {
    log::info!("接收Whisper转录任务: {:?}", task);
    let ts_now: i64 = chrono::Utc::now().timestamp();
    on_event
        .send(crate::types::WhisperTranformTaskEvent::Progress {
            is_success: true,
            message: "任务开始".to_string(),
            ts_execed: 0,
            ts: ts_now,
        })
        .unwrap();
    let mut args: Vec<String> = Vec::new();
    args.push("-m".to_string());
    args.push(task.model.clone());

    {
        if let Some(ref graphic) = task.graphic {
            args.push("-gpu".to_string());
            args.push(graphic.clone());
        }

        if let Some(ref lang) = task.lang {
            args.push("-l".to_string());
            args.push(lang.clone().to_lowercase());
        }

        if let Some(num_threads) = task.num_threads {
            args.push("-t".to_string());
            args.push(num_threads.to_string());
        }

        if let Some(num_cores) = task.num_cores {
            args.push("-p".to_string());
            args.push(num_cores.to_string());
        }

        if let Some(max_len_chars) = task.max_len_chars {
            args.push("-ml".to_string());
            args.push(max_len_chars.to_string());
        }
    }

    let path_video: &std::path::Path = std::path::Path::new(&task.path_video);
    let path_save: String = if let Some(ref path_save) = task.path_save {
        path_save.clone()
    } else {
        if let Some(parent_dir) = path_video.parent() {
            parent_dir.to_string_lossy().into_owned()
        } else {
            String::from(".")
        }
    };

    let whisper_path = app
        .path()
        .resolve("plugins/whisper/", tauri::path::BaseDirectory::Resource)
        .expect("获取可执行文件模型失败")
        .join("whisper.exe");
    on_event
        .send(crate::types::WhisperTranformTaskEvent::Progress {
            is_success: true,
            message: "命令合成".to_string(),
            ts_execed: chrono::Utc::now().timestamp() - ts_now,
            ts: chrono::Utc::now().timestamp(),
        })
        .unwrap();
    let mut process = Command::new(whisper_path)
        .args(args)
        .arg(&task.path_video)
        .stdout(Stdio::piped())
        .creation_flags(0x08000000)
        .spawn()
        .expect("执行exe失败");

    let mut srts: Vec<crate::types::STDSrtLine> = Vec::new();

    // 确保进程的标准输出可用
    if let Some(stdout) = process.stdout.take() {
        let lines_reader = BufReader::new(stdout);
        for line in lines_reader.lines() {
            match line {
                Ok(output) => {
                    if let Some(srt) =
                        parse_srt_from_str(&output, (srts.len() + 1).try_into().unwrap())
                    {
                        if srts.len() == 0 {
                            on_event
                                .send(crate::types::WhisperTranformTaskEvent::Progress {
                                    is_success: true,
                                    message: "字幕解析".to_string(),
                                    ts_execed: chrono::Utc::now().timestamp() - ts_now,
                                    ts: chrono::Utc::now().timestamp(),
                                })
                                .unwrap();
                        }
                        srts.push(srt);
                    }
                }
                Err(e) => log::error!("读取失败: {}", e),
            }
        }
    } else {
        log::error!("无法获取标准输出");
    }

    // 等待进程结束
    let exit_status = process.wait().expect("停止运行");
    if !exit_status.success() {
        log::error!("错误: {}", exit_status);
        on_event
            .send(crate::types::WhisperTranformTaskEvent::Progress {
                is_success: false,
                message: exit_status.to_string(),
                ts_execed: chrono::Utc::now().timestamp() - ts_now,
                ts: chrono::Utc::now().timestamp(),
            })
            .unwrap();
        return;
    }

    if srts.len() == 0 {
        on_event
            .send(crate::types::WhisperTranformTaskEvent::Progress {
                is_success: false,
                message: "字幕为空".to_string(),
                ts_execed: chrono::Utc::now().timestamp() - ts_now,
                ts: chrono::Utc::now().timestamp(),
            })
            .unwrap();
        return;
    }

    let path_video = Path::new(&task.path_video);
    let mut path_save = std::path::PathBuf::from(path_save);

    if let Some(stem) = path_video.file_stem().and_then(|s| s.to_str()) {
        let stem_string = stem.to_string();
        path_save = path_save.join(format!("{}.srt", stem_string));
    } else {
        path_save = path_save.join(format!(
            "{}.srt",
            chrono::Utc::now().timestamp().to_string()
        ));
    }
    if let Err(e) = write_srt_to_file(&srts, path_save) {
        on_event
            .send(crate::types::WhisperTranformTaskEvent::Progress {
                is_success: false,
                message: e.to_string(),
                ts_execed: chrono::Utc::now().timestamp() - ts_now,
                ts: chrono::Utc::now().timestamp(),
            })
            .unwrap();
        return;
    } else {
        on_event
            .send(crate::types::WhisperTranformTaskEvent::Progress {
                is_success: true,
                message: "写入成功".to_string(),
                ts_execed: chrono::Utc::now().timestamp() - ts_now,
                ts: chrono::Utc::now().timestamp(),
            })
            .unwrap();
    }
    on_event
        .send(crate::types::WhisperTranformTaskEvent::Progress {
            is_success: true,
            message: "任务完成".to_string(),
            ts_execed: chrono::Utc::now().timestamp() - ts_now,
            ts: chrono::Utc::now().timestamp(),
        })
        .unwrap();
    return;
}

fn parse_srt_from_str(subtitle: &str, index: u32) -> Option<crate::types::STDSrtLine> {
    let trimmed_line = subtitle.trim();
    if !trimmed_line.is_empty() {
        if let Some(start_pos) = trimmed_line.find('[') {
            if let Some(end_pos) = trimmed_line.find(']') {
                let ts_part = &trimmed_line[start_pos + 1..end_pos];
                let line_main = trimmed_line[end_pos + 1..].trim().to_string();
                let timestamps: Vec<&str> = ts_part.split(" --> ").collect();
                if timestamps.len() == 2 {
                    let ts_begin = timestamps[0].replace('.', ",");
                    let ts_end = timestamps[1].replace('.', ",");
                    let line_main = line_main
                        .to_lowercase()
                        .trim_end_matches(|c: char| c.is_ascii_punctuation() || c.is_whitespace())
                        .trim()
                        .to_string();
                    if !line_main.is_empty() {
                        return Some(crate::types::STDSrtLine {
                            idx: index,
                            ts_begin,
                            ts_end,
                            line_main,
                        });
                    }
                }
            }
        }
    }

    None // 如果无法解析，返回 None
}

fn write_srt_to_file<P: AsRef<Path>>(
    srt_lines: &[crate::types::STDSrtLine],
    path: P,
) -> io::Result<()> {
    let mut file = File::create(path)?;

    for srt_line in srt_lines {
        writeln!(file, "{}", srt_line.idx)?;
        writeln!(file, "{} --> {}", srt_line.ts_begin, srt_line.ts_end)?;
        writeln!(file, "{}", srt_line.line_main)?;
        writeln!(file)?;
    }

    Ok(())
}
