use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagDefinition {
    pub id: String,
    #[serde(default)]
    pub labels: HashMap<String, String>,
}

impl TagDefinition {
    pub fn label_for_lang(&self, lang: &str) -> String {
        if let Some(label) = self.labels.get(lang) {
            return label.clone();
        }
        if let Some(label) = self.labels.get("en") {
            return label.clone();
        }
        if let Some(label) = self.labels.values().next() {
            return label.clone();
        }
        self.id.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Tag {
    pub id: String,
    pub label: String,
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PostMetadata {
    pub title: String,
    pub date: String,
    pub slug: String,
    pub lang: String,
    pub tags: Option<Vec<Tag>>,
    pub word_count: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Post {
    pub meta: PostMetadata,
    pub content: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TagGroup {
    pub tag: Tag,
    pub posts: Vec<PostMetadata>,
}
