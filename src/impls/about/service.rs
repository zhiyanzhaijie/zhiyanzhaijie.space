use std::fs;
use std::path::Path;

const ABOUT_CN_PATH: &str = "content/about/about_cn.md";
const ABOUT_EN_PATH: &str = "content/about/about_en.md";

pub fn get_about_markdown(lang: &str) -> String {
    let normalized = normalize_locale(lang);
    let primary = match normalized {
        "en" => ABOUT_EN_PATH,
        _ => ABOUT_CN_PATH,
    };
    let fallback = match normalized {
        "en" => ABOUT_CN_PATH,
        _ => ABOUT_EN_PATH,
    };

    read_markdown(primary).unwrap_or_else(|| read_markdown(fallback).unwrap_or_default())
}

fn normalize_locale(value: &str) -> &str {
    match value {
        "en" => "en",
        _ => "cn",
    }
}

fn read_markdown(path: &str) -> Option<String> {
    let file_path = Path::new(path);
    if !file_path.exists() {
        return None;
    }
    fs::read_to_string(file_path).ok()
}
