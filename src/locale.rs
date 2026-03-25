#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Locale {
    ZhCN,
    EnUS,
    JaJP,
    FrFR,
    DeDE,
    ArSA,
}

pub fn detect_locale_from_env() -> Locale {
    let lang = std::env::var("LC_ALL")
        .or_else(|_| std::env::var("LANG"))
        .unwrap_or_else(|_| "en-US".to_string());

    if lang.starts_with("zh_CN") || lang.starts_with("zh-CN") || lang.starts_with("zh") {
        Locale::ZhCN
    } else if lang.starts_with("fr_FR") || lang.starts_with("fr-FR") || lang.starts_with("fr") {
        Locale::FrFR
    } else if lang.starts_with("de_DE") || lang.starts_with("de-DE") || lang.starts_with("de") {
        Locale::DeDE
    } else if lang.starts_with("ja_JP") || lang.starts_with("ja-JP") || lang.starts_with("ja") {
        Locale::JaJP
    } else if lang.starts_with("ar_SA") || lang.starts_with("ar-SA") || lang.starts_with("ar") {
        Locale::ArSA
    } else {
        Locale::EnUS
    }
}