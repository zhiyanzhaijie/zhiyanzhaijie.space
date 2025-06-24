use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tag {
    Dioxus,
    Rust,
    AsyncProgramming,
    Web,
    DevKit,
    Book,
    Talk,
}

impl Tag {
    /// Returns the i18n key for this tag
    pub fn i18n_key(&self) -> &'static str {
        match self {
            Tag::Dioxus => "tag-dioxus",
            Tag::Web => "tag-web",
            Tag::Rust => "tag-rust",
            Tag::AsyncProgramming => "tag-async-programming",
            Tag::DevKit => "tag-devkit",
            Tag::Book => "tag-book",
            Tag::Talk => "tag-talk",
        }
    }

    /// Returns the URL-safe string representation
    pub fn to_url_string(&self) -> &'static str {
        match self {
            Tag::Dioxus => "dioxus",
            Tag::Web => "web",
            Tag::Rust => "rust",
            Tag::AsyncProgramming => "async_programming",
            Tag::DevKit => "devkit",
            Tag::Book => "book",
            Tag::Talk => "talk",
        }
    }

    /// Get all available tags
    pub fn all() -> Vec<Tag> {
        vec![
            Tag::Dioxus,
            Tag::Web,
            Tag::Rust,
            Tag::AsyncProgramming,
            Tag::DevKit,
            Tag::Book,
            Tag::Talk,
        ]
    }
}

impl fmt::Display for Tag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_url_string())
    }
}

impl FromStr for Tag {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "dioxus" => Ok(Tag::Dioxus),
            "web" => Ok(Tag::Web),
            "rust" => Ok(Tag::Rust),
            "async_programming" | "异步编程" => Ok(Tag::AsyncProgramming),
            "tool" | "工具" => Ok(Tag::DevKit),
            "book" | "书籍" => Ok(Tag::Book),
            "talk" | "演讲" => Ok(Tag::Talk),
            _ => Err(format!("Unknown tag: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
