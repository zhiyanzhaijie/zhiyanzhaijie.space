#[cfg(feature = "server")]
mod service;
#[cfg(feature = "server")]
mod store;
mod types;

#[cfg(feature = "server")]
pub use service::{
    get_available_languages_for_slug, get_post_by_slug_and_lang, get_post_content_with_fallback,
    get_post_with_fallback, get_posts_by_lang, get_posts_by_tag_and_lang, get_static_routes,
    get_tag_groups,
};
pub use types::{Post, PostMetadata, TagGroup};
