use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Tag {
    Dioxus,
    Technology,
    Rust,
    AsyncProgramming,
    Learning,
    Programming,
    Growth,
    Life,
    Reflection,
}

impl Tag {
    /// Returns the i18n key for this tag
    pub fn i18n_key(&self) -> &'static str {
        match self {
            Tag::Dioxus => "tag-dioxus",
            Tag::Technology => "tag-technology",
            Tag::Rust => "tag-rust",
            Tag::AsyncProgramming => "tag-async-programming",
            Tag::Learning => "tag-learning",
            Tag::Programming => "tag-programming",
            Tag::Growth => "tag-growth",
            Tag::Life => "tag-life",
            Tag::Reflection => "tag-reflection",
        }
    }

    /// Returns the URL-safe string representation
    pub fn to_url_string(&self) -> &'static str {
        match self {
            Tag::Dioxus => "dioxus",
            Tag::Technology => "technology",
            Tag::Rust => "rust",
            Tag::AsyncProgramming => "async-programming",
            Tag::Learning => "learning",
            Tag::Programming => "programming",
            Tag::Growth => "growth",
            Tag::Life => "life",
            Tag::Reflection => "reflection",
        }
    }

    /// Get all available tags
    pub fn all() -> Vec<Tag> {
        vec![
            Tag::Dioxus,
            Tag::Technology,
            Tag::Rust,
            Tag::AsyncProgramming,
            Tag::Learning,
            Tag::Programming,
            Tag::Growth,
            Tag::Life,
            Tag::Reflection,
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
            "technology" | "技术" => Ok(Tag::Technology),
            "rust" => Ok(Tag::Rust),
            "async-programming" | "async_programming" | "异步编程" => Ok(Tag::AsyncProgramming),
            "learning" | "学习" => Ok(Tag::Learning),
            "programming" | "编程" => Ok(Tag::Programming),
            "growth" | "成长" => Ok(Tag::Growth),
            "life" | "生活" => Ok(Tag::Life),
            "reflection" | "感悟" => Ok(Tag::Reflection),
            _ => Err(format!("Unknown tag: {}", s)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tag_from_str() {
        assert_eq!("dioxus".parse::<Tag>().unwrap(), Tag::Dioxus);
        assert_eq!("技术".parse::<Tag>().unwrap(), Tag::Technology);
        assert_eq!("rust".parse::<Tag>().unwrap(), Tag::Rust);
        assert_eq!("异步编程".parse::<Tag>().unwrap(), Tag::AsyncProgramming);
        assert_eq!("学习".parse::<Tag>().unwrap(), Tag::Learning);
        assert_eq!("编程".parse::<Tag>().unwrap(), Tag::Programming);
        assert_eq!("成长".parse::<Tag>().unwrap(), Tag::Growth);
        assert_eq!("生活".parse::<Tag>().unwrap(), Tag::Life);
        assert_eq!("感悟".parse::<Tag>().unwrap(), Tag::Reflection);
    }

    #[test]
    fn test_tag_to_string() {
        assert_eq!(Tag::Dioxus.to_string(), "dioxus");
        assert_eq!(Tag::Technology.to_string(), "technology");
        assert_eq!(Tag::AsyncProgramming.to_string(), "async-programming");
    }

    #[test]
    fn test_tag_i18n_key() {
        assert_eq!(Tag::Dioxus.i18n_key(), "tag-dioxus");
        assert_eq!(Tag::Technology.i18n_key(), "tag-technology");
        assert_eq!(Tag::AsyncProgramming.i18n_key(), "tag-async-programming");
    }
}
