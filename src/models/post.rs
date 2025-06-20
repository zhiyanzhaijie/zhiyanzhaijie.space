use chrono::NaiveDate;
use include_dir::{include_dir, Dir};
use once_cell::sync::Lazy;
use serde::Deserialize;
use std::sync::Arc;

use crate::models::tag::Tag;
use crate::utils::url::percent_decode;

// Reference static resources defined in app.rs
// Paths for include_dir! are relative to CARGO_MANIFEST_DIR
static POST_FILES: Dir<'_> = include_dir!("src/md");

// Raw metadata structure for deserializing from frontmatter
#[derive(Debug, Deserialize, Clone, PartialEq)]
struct RawPostMetadata {
    pub title: String,
    pub date: String,
    pub slug: String,
    #[serde(default)] // Tags are optional, default to an empty vec if not present or null
    pub tags: Option<Vec<String>>,
}

// Enriched metadata structure used in the application
#[derive(Debug, Clone, PartialEq)]
pub struct PostMetadata {
    pub title: String,
    pub date: String, // Keep as string for display, but parse for sorting
    pub slug: String,
    pub tags: Option<Vec<Tag>>,
    pub word_count: usize,
}

pub static POSTS: Lazy<Vec<(PostMetadata, String)>> = Lazy::new(load_markdown_posts);

fn calculate_word_count(content: &str) -> usize {
    content.split_whitespace().count()
}

pub fn load_markdown_posts() -> Vec<(PostMetadata, String)> {
    let mut posts_data = Vec::new();

    for file_entry in POST_FILES.files() {
        if let Some(content_str) = file_entry.contents_utf8() {
            let parts: Vec<&str> = content_str.splitn(3, "---").collect();
            if parts.len() == 3 {
                let frontmatter_str = parts[1];
                let md_content = parts[2].trim_start();

                match serde_yaml::from_str::<RawPostMetadata>(frontmatter_str) {
                    Ok(raw_meta) => {
                        let word_count = calculate_word_count(md_content);

                        // Convert string tags to Tag enum
                        let tags = raw_meta
                            .tags
                            .map(|tag_strings| {
                                tag_strings
                                    .iter()
                                    .filter_map(|tag_str| match tag_str.parse::<Tag>() {
                                        Ok(tag) => Some(tag),
                                        Err(e) => {
                                            eprintln!(
                                                "Warning: Unknown tag '{}' in post '{}': {}",
                                                tag_str, raw_meta.title, e
                                            );
                                            None
                                        }
                                    })
                                    .collect::<Vec<Tag>>()
                            })
                            .filter(|tags| !tags.is_empty()); // Remove empty tag lists

                        let metadata = PostMetadata {
                            title: raw_meta.title,
                            date: raw_meta.date,
                            slug: raw_meta.slug,
                            tags,
                            word_count,
                        };
                        posts_data.push((metadata, md_content.to_string()));
                    }
                    Err(e) => {
                        if let Some(path) = file_entry.path().to_str() {
                            eprintln!(
                                "Failed to parse frontmatter for [{}]: {}. Skipping this post. Frontmatter content:\\n{}",
                                path, e, frontmatter_str
                            );
                        } else {
                            eprintln!(
                                "Failed to parse frontmatter for an unknown file: {}. Skipping this post. Frontmatter content:\\n{}",
                                e, frontmatter_str
                            );
                        }
                    }
                }
            } else {
                if let Some(path) = file_entry.path().to_str() {
                    eprintln!(
                        "File [{}] does not appear to have valid YAML frontmatter. Skipping.",
                        path
                    );
                } else {
                    eprintln!("A file does not appear to have valid YAML frontmatter. Skipping.");
                }
            }
        }
    }

    // Sort posts by date in descending order
    // NaiveDate::parse_from_str is used for robust date parsing.
    // This assumes date format is "YYYY-MM-DD". Adjust format string if needed.
    posts_data.sort_by(|a, b| {
        let date_a = NaiveDate::parse_from_str(&a.0.date, "%Y-%m-%d").unwrap_or_else(|e| {
            eprintln!(
                "Failed to parse date '{}' for post '{}': {}. Using default date.",
                a.0.date, a.0.title, e
            );
            NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
        });
        let date_b = NaiveDate::parse_from_str(&b.0.date, "%Y-%m-%d").unwrap_or_else(|e| {
            eprintln!(
                "Failed to parse date '{}' for post '{}': {}. Using default date.",
                b.0.date, b.0.title, e
            );
            NaiveDate::from_ymd_opt(1970, 1, 1).unwrap()
        });
        date_b.cmp(&date_a) // Sort descending (newest first)
    });

    posts_data
}

pub fn get_post_by_slug(slug: &str) -> Option<(Arc<PostMetadata>, String)> {
    let decoded_slug = percent_decode(slug);
    POSTS.iter().find_map(|(meta, content)| {
        if meta.slug == decoded_slug {
            Some((Arc::new(meta.clone()), content.clone()))
        } else {
            None
        }
    })
}
