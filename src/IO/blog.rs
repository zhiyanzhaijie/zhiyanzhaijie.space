use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::impls::blog;
use crate::impls::blog::{Post, PostMetadata, TagGroup};
#[cfg(feature = "server")]
use crate::root::Route;

#[get("/api/blog/posts/:lang")]
pub async fn get_posts_by_lang(lang: String) -> ServerFnResult<Vec<PostMetadata>> {
    Ok(blog::get_posts_by_lang(&lang))
}

#[get("/api/blog/post/:slug/:lang")]
pub async fn get_post_by_slug_and_lang(slug: String, lang: String) -> ServerFnResult<Option<Post>> {
    Ok(blog::get_post_by_slug_and_lang(&slug, &lang))
}

#[get("/api/blog/post_fallback/:slug/:lang")]
pub async fn get_post_with_fallback(slug: String, lang: String) -> ServerFnResult<Option<Post>> {
    Ok(blog::get_post_with_fallback(&slug, &lang))
}

#[get("/api/blog/post_content_fallback/:slug/:lang")]
pub async fn get_post_content_with_fallback(
    slug: String,
    lang: String,
) -> ServerFnResult<Option<String>> {
    Ok(blog::get_post_content_with_fallback(&slug, &lang))
}

#[get("/api/blog/post_languages/:slug")]
pub async fn get_post_languages(slug: String) -> ServerFnResult<Vec<String>> {
    Ok(blog::get_available_languages_for_slug(&slug))
}

#[get("/api/blog/tag_posts/:tag/:lang")]
pub async fn get_posts_by_tag(tag: String, lang: String) -> ServerFnResult<Vec<PostMetadata>> {
    Ok(blog::get_posts_by_tag_and_lang(&tag, &lang))
}

#[get("/api/blog/tag_groups/:lang")]
pub async fn get_tag_groups(lang: String) -> ServerFnResult<Vec<TagGroup>> {
    Ok(blog::get_tag_groups(&lang))
}

#[post("/api/static_routes")]
pub async fn static_routes() -> ServerFnResult<Vec<String>> {
    let mut routes = Route::static_routes()
        .into_iter()
        .map(|route| route.to_string())
        .collect::<Vec<_>>();
    routes.extend(blog::get_static_routes());
    routes.sort();
    routes.dedup();
    Ok(routes)
}
