#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Language {
    pub code: LangCode,
    pub name_zh: &'static str,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, Copy, Eq, PartialEq)]
pub enum LangCode {
    ZH = 1,
    EN,
    JP, // 日语
    FR, // 法语
    KR, // 韩语
    RU, // 俄罗斯语
}

pub const LANGUAGES: &[Language] = &[
    Language {
        code: LangCode::EN,
        name_zh: "英语",
    },
    Language {
        code: LangCode::ZH,
        name_zh: "中文",
    },
    Language {
        code: LangCode::JP,
        name_zh: "日语",
    },
    Language {
        code: LangCode::FR,
        name_zh: "法语",
    },
    Language {
        code: LangCode::KR,
        name_zh: "韩语",
    },
    Language {
        code: LangCode::RU,
        name_zh: "俄罗斯语",
    },
];

pub fn _languages_for_option() -> Vec<String> {
    LANGUAGES
        .iter()
        .map(|language| language.name_zh.to_string())
        .collect()
}

pub fn _name_zh_by_code(code: LangCode) -> Option<String> {
    LANGUAGES
        .iter()
        .find(|language| language.code == code) // Now valid
        .map(|language| language.name_zh.to_string())
}
