use once_cell::sync::Lazy;
use serde::Deserialize;
use include_dir::{include_dir, Dir};
use std::sync::Arc;

use crate::utils::url::percent_decode;

// 引用在app.rs中定义的静态资源
static POST_FILES: Dir<'_> = include_dir!("src/md");

#[derive(Debug, Deserialize, Clone, PartialEq)]
pub struct PostMetadata {
    pub title: String,
    pub date: String,
    pub slug: String,
}

pub static POSTS: Lazy<Vec<(PostMetadata, String)>> = Lazy::new(load_markdown_posts);

pub fn load_markdown_posts() -> Vec<(PostMetadata, String)> {
    let mut posts = Vec::new();

    for file_entry in POST_FILES.files() {
        if let Some(content_str) = file_entry.contents_utf8() {
            let parts: Vec<&str> = content_str.splitn(3, "---").collect();
            if parts.len() == 3 {
                let frontmatter_str = parts[1];
                let md_content = parts[2].trim_start();

                if let Ok(metadata) = serde_yaml::from_str::<PostMetadata>(frontmatter_str) {
                    posts.push((metadata, md_content.to_string()));
                }
            }
        }
    }
    posts
}

pub fn get_post_by_slug(slug: &str) -> Option<(Arc<PostMetadata>, String)> {
    let decoded_slug = percent_decode(slug);
    for (meta, content) in POSTS.iter() {
        if meta.slug == decoded_slug {
            return Some((Arc::new(meta.clone()), content.clone()));
        }
    }
    None
}