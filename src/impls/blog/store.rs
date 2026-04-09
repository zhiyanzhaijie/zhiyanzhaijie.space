use chrono::NaiveDate;
use serde::Deserialize;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::{Path, PathBuf};
use std::sync::{OnceLock, RwLock};
use std::time::UNIX_EPOCH;
use crate::impls::blog::types::{Post, PostMetadata, Tag, TagDefinition};

const POSTS_ROOT: &str = "content/posts";
const TAGS_PATH: &str = "content/tags.yml";

#[derive(Default)]
struct CacheState {
    signature: u64,
    loaded: bool,
    posts: Vec<Post>,
}

fn load_tag_definitions(tags_path: &Path) -> HashMap<String, TagDefinition> {
    let tags_text = match fs::read_to_string(tags_path) {
        Ok(text) => text,
        Err(error) => {
            log::warn!(
                "failed to read tag definitions {}: {error}",
                tags_path.display()
            );
            return default_tag_definitions();
        }
    };

    let raw_config = match serde_yaml::from_str::<RawTagConfig>(&tags_text) {
        Ok(config) => config,
        Err(error) => {
            log::warn!(
                "failed to parse tag definitions {}: {error}",
                tags_path.display()
            );
            return default_tag_definitions();
        }
    };

    let mut definitions = HashMap::new();
    for raw_tag in raw_config.tags {
        let id = normalize_tag_id(&raw_tag.id);
        if id.is_empty() {
            continue;
        }
        definitions.insert(
            id.clone(),
            TagDefinition {
                id,
                labels: raw_tag.labels,
            },
        );
    }

    if definitions.is_empty() {
        return default_tag_definitions();
    }

    definitions
}

fn default_tag_definitions() -> HashMap<String, TagDefinition> {
    HashMap::from([
        (
            "resource".to_string(),
            TagDefinition {
                id: "resource".to_string(),
                labels: HashMap::from([
                    ("cn".to_string(), "知识输出".to_string()),
                    ("en".to_string(), "Resource".to_string()),
                ]),
            },
        ),
        (
            "idea".to_string(),
            TagDefinition {
                id: "idea".to_string(),
                labels: HashMap::from([
                    ("cn".to_string(), "日常想法".to_string()),
                    ("en".to_string(), "Idea".to_string()),
                ]),
            },
        ),
    ])
}

fn normalize_tag_id(value: &str) -> String {
    value.trim().to_lowercase()
}

static POSTS_CACHE: OnceLock<RwLock<CacheState>> = OnceLock::new();

#[derive(Debug, Deserialize)]
struct RawPostMeta {
    slug: String,
    date: String,
    #[serde(default)]
    tags: Option<Vec<String>>,
    #[serde(default)]
    titles: HashMap<String, String>,
}

#[derive(Debug, Deserialize, Default)]
struct RawTagConfig {
    #[serde(default)]
    tags: Vec<RawTagDefinition>,
}

#[derive(Debug, Deserialize)]
struct RawTagDefinition {
    id: String,
    #[serde(default)]
    labels: HashMap<String, String>,
}

pub fn all_posts() -> Vec<Post> {
    let posts_root = Path::new(POSTS_ROOT);
    let tags_path = Path::new(TAGS_PATH);
    let signature = compute_signature(posts_root, tags_path);
    let cache = POSTS_CACHE.get_or_init(|| RwLock::new(CacheState::default()));

    if let Ok(state) = cache.read() {
        if state.loaded && state.signature == signature {
            return state.posts.clone();
        }
    }
    let posts = load_posts_from_disk(posts_root, tags_path);

    if let Ok(mut state) = cache.write() {
        state.signature = signature;
        state.loaded = true;
        state.posts = posts.clone();
    }

    posts
}

fn load_posts_from_disk(root: &Path, tags_path: &Path) -> Vec<Post> {
    let mut posts = Vec::new();
    let tag_definitions = load_tag_definitions(tags_path);

    for post_dir in list_post_dirs(root) {
        posts.extend(load_post_dir(&post_dir, &tag_definitions));
    }

    posts.sort_by(|a, b| {
        parse_date_or_default(&b.meta.date).cmp(&parse_date_or_default(&a.meta.date))
    });
    posts
}

fn load_post_dir(post_dir: &Path, tag_definitions: &HashMap<String, TagDefinition>) -> Vec<Post> {
    let meta_path = post_dir.join("meta.yml");
    let meta_text = match fs::read_to_string(&meta_path) {
        Ok(text) => text,
        Err(error) => {
            log::warn!(
                "failed to read post metadata {}: {error}",
                meta_path.display()
            );
            return Vec::new();
        }
    };

    let raw_meta = match serde_yaml::from_str::<RawPostMeta>(&meta_text) {
        Ok(meta) => meta,
        Err(error) => {
            log::warn!(
                "failed to parse post metadata {}: {error}",
                meta_path.display()
            );
            return Vec::new();
        }
    };

    let mut posts = Vec::new();

    for markdown_path in list_markdown_files(post_dir) {
        let lang = match markdown_path.file_stem().and_then(|stem| stem.to_str()) {
            Some(lang) => lang.to_string(),
            None => continue,
        };

        let content = match fs::read_to_string(&markdown_path) {
            Ok(content) => content,
            Err(error) => {
                log::warn!(
                    "failed to read markdown content {}: {error}",
                    markdown_path.display()
                );
                continue;
            }
        };

        let post = Post {
            meta: PostMetadata {
                title: resolve_title(&raw_meta, &lang),
                date: raw_meta.date.clone(),
                slug: raw_meta.slug.clone(),
                tags: parse_tags(&raw_meta.tags, tag_definitions, &lang),
                lang,
                word_count: content.split_whitespace().count(),
            },
            content,
        };
        posts.push(post);
    }

    posts
}

fn resolve_title(raw_meta: &RawPostMeta, lang: &str) -> String {
    if let Some(title) = raw_meta.titles.get(lang) {
        return title.clone();
    }

    if let Some(title) = raw_meta.titles.get("en") {
        return title.clone();
    }

    if let Some(title) = raw_meta.titles.values().next() {
        return title.clone();
    }

    raw_meta.slug.clone()
}

fn parse_tags(
    tags: &Option<Vec<String>>,
    tag_definitions: &HashMap<String, TagDefinition>,
    lang: &str,
) -> Option<Vec<Tag>> {
    let mut parsed = Vec::new();
    let mut seen = HashSet::new();

    if let Some(tag_values) = tags {
        for value in tag_values {
            let tag_id = normalize_tag_id(value);
            if tag_id.is_empty() || !seen.insert(tag_id.clone()) {
                continue;
            }
            match tag_definitions.get(&tag_id) {
                Some(definition) => parsed.push(Tag {
                    id: definition.id.clone(),
                    label: definition.label_for_lang(lang),
                }),
                None => log::warn!("unknown tag id '{}'", value),
            }
        }
    }

    if parsed.is_empty() {
        None
    } else {
        Some(parsed)
    }
}

fn list_post_dirs(root: &Path) -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    let entries = match fs::read_dir(root) {
        Ok(entries) => entries,
        Err(_) => return dirs,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            dirs.push(path);
        }
    }

    dirs.sort();
    dirs
}

fn list_markdown_files(post_dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    let entries = match fs::read_dir(post_dir) {
        Ok(entries) => entries,
        Err(_) => return files,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        let ext = path.extension().and_then(|value| value.to_str());
        if path.is_file() && ext == Some("md") {
            files.push(path);
        }
    }

    files.sort();
    files
}

fn compute_signature(posts_root: &Path, tags_path: &Path) -> u64 {
    let mut files = Vec::new();
    collect_files_recursive(posts_root, &mut files);
    if tags_path.is_file() {
        files.push(tags_path.to_path_buf());
    }
    files.sort();

    let mut hasher = DefaultHasher::new();
    for file in files {
        file.to_string_lossy().hash(&mut hasher);

        if let Ok(metadata) = fs::metadata(&file) {
            metadata.len().hash(&mut hasher);
            if let Ok(modified) = metadata.modified() {
                if let Ok(duration) = modified.duration_since(UNIX_EPOCH) {
                    duration.as_secs().hash(&mut hasher);
                    duration.subsec_nanos().hash(&mut hasher);
                }
            }
        }
    }
    hasher.finish()
}

fn collect_files_recursive(dir: &Path, files: &mut Vec<PathBuf>) {
    let entries = match fs::read_dir(dir) {
        Ok(entries) => entries,
        Err(_) => return,
    };

    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect_files_recursive(&path, files);
            continue;
        }
        if path.is_file() {
            files.push(path);
        }
    }
}

fn parse_date_or_default(date: &str) -> NaiveDate {
    if let Ok(value) = NaiveDate::parse_from_str(date, "%Y-%m-%d") {
        return value;
    }

    if let Some(value) = NaiveDate::from_ymd_opt(1970, 1, 1) {
        return value;
    }

    NaiveDate::MIN
}
