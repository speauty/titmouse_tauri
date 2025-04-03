use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WhisperTranformTask {
    pub path_video: String,
    pub model: String,
    pub lang: Option<String>,
    pub graphic: Option<String>,
    pub num_cores: Option<u32>,
    pub num_threads: Option<u32>,
    pub max_len_chars: Option<u32>,
    pub path_save: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct STDSrtLine {
    pub idx: u32,
    pub ts_begin: String,
    pub ts_end: String,
    pub line_main: String,
}

#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data")]
pub enum WhisperTranformTaskEvent {
    Progress {
        is_success: bool,
        message: String,
        ts_execed: i64,
        ts: i64,
    },
}

impl Default for STDSrtLine {
    fn default() -> Self {
        Self {idx: 0, ts_begin: String::new(), ts_end: String::new(), line_main: String::new()}
    }
}
