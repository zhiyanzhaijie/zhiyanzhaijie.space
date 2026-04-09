use dioxus::prelude::*;

use crate::impls::session::preference;
use crate::impls::session::AppSession;

#[derive(Debug, Clone, Default, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct SessionPreferenceDto {
    pub locale: Option<String>,
    pub theme: Option<String>,
}

#[get("/api/user/preference", session: AppSession, seed: preference::PreferenceSeed)]
pub async fn get_preference() -> ServerFnResult<SessionPreferenceDto> {
    Ok(SessionPreferenceDto {
        locale: preference::resolve_locale(&session, &seed),
        theme: preference::resolve_theme(&session),
    })
}

#[post("/api/user/locale/:locale", session: AppSession)]
pub async fn set_locale(locale: String) -> ServerFnResult<()> {
    preference::update_locale(&session, &locale);
    Ok(())
}

#[post("/api/user/theme/:theme", session: AppSession)]
pub async fn set_theme(theme: String) -> ServerFnResult<()> {
    preference::update_theme(&session, &theme);
    Ok(())
}
