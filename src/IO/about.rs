use dioxus::prelude::*;

#[cfg(feature = "server")]
use crate::impls::about;

#[get("/api/about/:lang")]
pub async fn get_about_markdown(lang: String) -> ServerFnResult<String> {
    Ok(about::get_about_markdown(&lang))
}
