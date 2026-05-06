use dioxus::fullstack::FileStream;
use dioxus::prelude::ServerFnError;
use std::path::PathBuf;

const POSTS_ROOT: &str = "content/posts";

pub async fn get_post_asset(slug: &str, file: &str) -> Result<FileStream, ServerFnError> {
    if !is_safe_segment(slug) || !is_safe_segment(file) || !is_supported_asset(file) {
        return Err(not_found());
    }

    let path = PathBuf::from(POSTS_ROOT)
        .join(slug)
        .join(format!("img/{file}"));
    FileStream::from_path(path).await.map_err(|_| not_found())
}

fn is_safe_segment(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('.')
        && !value.contains('/')
        && !value.contains('\\')
        && value != ".."
}

fn is_supported_asset(file: &str) -> bool {
    matches!(
        file.rsplit_once('.')
            .map(|(_, extension)| extension.to_ascii_lowercase())
            .as_deref(),
        Some("png" | "jpg" | "jpeg" | "webp" | "gif" | "svg" | "avif")
    )
}

fn not_found() -> ServerFnError {
    ServerFnError::ServerError {
        message: "post asset not found".to_string(),
        code: 404,
        details: None,
    }
}
