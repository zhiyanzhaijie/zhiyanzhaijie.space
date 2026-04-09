use chrono::NaiveDate;
use std::collections::{BTreeSet, HashMap};

use crate::impls::blog::types::{Post, PostMetadata, Tag, TagGroup};
use crate::utils::url::percent_decode;

#[cfg(feature = "server")]
use crate::impls::blog::store::all_posts;

#[cfg(not(feature = "server"))]
fn all_posts() -> Vec<Post> {
    Vec::new()
}

pub fn get_posts_by_lang(lang: &str) -> Vec<PostMetadata> {
    let mut posts = all_posts()
        .into_iter()
        .filter(|post| post.meta.lang == lang)
        .map(|post| post.meta)
        .collect::<Vec<_>>();

    sort_posts_desc(&mut posts);
    posts
}

pub fn get_posts_by_tag_and_lang(tag: &str, lang: &str) -> Vec<PostMetadata> {
    let parsed_tag = normalize_tag_id(&percent_decode(tag));
    if parsed_tag.is_empty() {
        return Vec::new();
    }

    let mut posts = all_posts()
        .into_iter()
        .filter(|post| {
            post.meta.lang == lang
                && post
                    .meta
                    .tags
                    .as_ref()
                    .map(|tags| {
                        tags.iter()
                            .any(|item| item.id.as_str() == parsed_tag.as_str())
                    })
                    .unwrap_or(false)
        })
        .map(|post| post.meta)
        .collect::<Vec<_>>();

    sort_posts_desc(&mut posts);
    posts
}

pub fn get_post_by_slug_and_lang(slug: &str, lang: &str) -> Option<Post> {
    let decoded_slug = percent_decode(slug);
    all_posts()
        .into_iter()
        .find(|post| post.meta.slug == decoded_slug && post.meta.lang == lang)
}

pub fn get_post_with_fallback(slug: &str, lang: &str) -> Option<Post> {
    if let Some(post) = get_post_by_slug_and_lang(slug, lang) {
        return Some(post);
    }

    let available_languages = get_available_languages_for_slug(slug);
    if available_languages.is_empty() {
        return None;
    }

    let fallback_lang = if available_languages.iter().any(|value| value == "en") {
        "en".to_string()
    } else {
        available_languages[0].clone()
    };

    get_post_by_slug_and_lang(slug, &fallback_lang)
}

pub fn get_post_content_with_fallback(slug: &str, lang: &str) -> Option<String> {
    get_post_with_fallback(slug, lang).map(|post| post.content)
}

pub fn get_available_languages_for_slug(slug: &str) -> Vec<String> {
    let decoded_slug = percent_decode(slug);
    let mut langs = all_posts()
        .into_iter()
        .filter(|post| post.meta.slug == decoded_slug)
        .map(|post| post.meta.lang)
        .collect::<Vec<_>>();
    langs.sort();
    langs.dedup();
    langs
}

pub fn get_tag_groups(lang: &str) -> Vec<TagGroup> {
    let mut grouped = HashMap::<String, Vec<PostMetadata>>::new();
    let mut tags_by_id = HashMap::<String, Tag>::new();

    for post in all_posts()
        .into_iter()
        .filter(|post| post.meta.lang == lang)
    {
        if let Some(tags) = post.meta.tags.as_ref() {
            for tag in tags {
                grouped
                    .entry(tag.id.clone())
                    .or_insert_with(Vec::new)
                    .push(post.meta.clone());
                tags_by_id
                    .entry(tag.id.clone())
                    .or_insert_with(|| tag.clone());
            }
        }
    }

    let mut groups = grouped
        .into_iter()
        .filter_map(|(tag_id, mut posts)| {
            sort_posts_desc(&mut posts);
            tags_by_id
                .get(&tag_id)
                .cloned()
                .map(|tag| TagGroup { tag, posts })
        })
        .collect::<Vec<_>>();
    groups.sort_by(|a, b| a.tag.id.cmp(&b.tag.id));
    groups
}

pub fn get_static_routes() -> Vec<String> {
    let posts = all_posts();
    let mut routes = BTreeSet::new();
    routes.insert("/blog/".to_string());
    routes.insert("/tags/".to_string());

    let mut slugs = BTreeSet::new();
    let mut tags = BTreeSet::new();

    for post in posts {
        slugs.insert(post.meta.slug);
        if let Some(post_tags) = post.meta.tags {
            for tag in post_tags {
                tags.insert(tag.to_string());
            }
        }
    }

    for slug in slugs {
        routes.insert(format!("/blog/post/{slug}"));
    }

    for tag in tags {
        routes.insert(format!("/tags/{tag}"));
    }

    routes.into_iter().collect()
}

fn sort_posts_desc(posts: &mut [PostMetadata]) {
    posts.sort_by(|a, b| parse_date_or_default(&b.date).cmp(&parse_date_or_default(&a.date)));
}

fn normalize_tag_id(value: &str) -> String {
    value.trim().to_lowercase()
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
