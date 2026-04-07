#[cfg(feature = "server")]
mod server {
    use axum::extract::FromRequestParts;
    use axum::http::{request::Parts, HeaderMap};
    use dioxus::prelude::ServerFnError;

    use crate::impls::session::consts::{SESSION_PREFERENCE_LOCALE_KEY, SESSION_PREFERENCE_THEME_KEY};
    use crate::impls::session::AppSession;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PreferenceField {
        Locale,
        Theme,
    }

    impl PreferenceField {
        fn key(self) -> &'static str {
            match self {
                Self::Locale => SESSION_PREFERENCE_LOCALE_KEY,
                Self::Theme => SESSION_PREFERENCE_THEME_KEY,
            }
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct PreferenceSeed {
        pub accept_language: Option<String>,
        pub inferred_locale: Option<String>,
    }

    impl From<&HeaderMap> for PreferenceSeed {
        fn from(headers: &HeaderMap) -> Self {
            let accept_language = read_header(headers, "accept-language");
            let inferred_locale = infer_locale(accept_language.as_deref());

            Self {
                accept_language,
                inferred_locale,
            }
        }
    }

    impl<S> FromRequestParts<S> for PreferenceSeed
    where
        S: Send + Sync,
    {
        type Rejection = ServerFnError;

        async fn from_request_parts(
            parts: &mut Parts,
            _state: &S,
        ) -> Result<Self, Self::Rejection> {
            Ok((&parts.headers).into())
        }
    }

    pub fn get(session: &AppSession, field: PreferenceField) -> Option<String> {
        session.get::<String>(field.key())
    }

    pub fn set(session: &AppSession, field: PreferenceField, value: &str) {
        session.set(field.key(), value.to_string());
        session.set_store(true);
    }

    fn seed_if_absent(session: &AppSession, field: PreferenceField, value: &str) {
        if get(session, field).is_none() {
            session.set(field.key(), value.to_string());
            session.set_store(true);
        }
    }

    pub fn seed_session_if_absent(session: &AppSession, seed: &PreferenceSeed) {
        if let Some(locale) = seed.inferred_locale.as_deref() {
            seed_if_absent(session, PreferenceField::Locale, locale);
        }
    }

    pub fn resolve_locale(session: &AppSession, seed: &PreferenceSeed) -> Option<String> {
        seed_session_if_absent(session, seed);
        get(session, PreferenceField::Locale)
    }

    pub fn resolve_theme(session: &AppSession) -> Option<String> {
        get(session, PreferenceField::Theme)
    }

    pub fn update_locale(session: &AppSession, locale: &str) {
        let locale = match locale {
            "en" => "en",
            _ => "cn",
        };
        set(session, PreferenceField::Locale, locale);
    }

    pub fn update_theme(session: &AppSession, theme: &str) {
        let theme = match theme {
            "dark" => "dark",
            _ => "light",
        };
        set(session, PreferenceField::Theme, theme);
    }

    fn infer_locale(raw: Option<&str>) -> Option<String> {
        let raw = raw?;
        raw.split(',').find_map(|part| {
            let locale = part.split(';').next()?.trim().to_ascii_lowercase();
            if locale.starts_with("en") {
                return Some("en".to_string());
            }
            if locale.starts_with("zh") || locale.starts_with("cn") {
                return Some("cn".to_string());
            }
            None
        })
    }

    fn read_header(headers: &HeaderMap, key: &'static str) -> Option<String> {
        headers
            .get(key)
            .and_then(|value| value.to_str().ok())
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
    }
}

#[cfg(feature = "server")]
pub use server::*;

#[cfg(not(feature = "server"))]
mod client {
    use crate::impls::session::AppSession;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PreferenceField {
        Locale,
        Theme,
    }

    #[derive(Clone, Debug, Default)]
    pub struct PreferenceSeed;

    pub fn get(_session: &AppSession, _field: PreferenceField) -> Option<String> {
        None
    }

    pub fn set(_session: &AppSession, _field: PreferenceField, _value: &str) {}

    pub fn seed_session_if_absent(_session: &AppSession, _seed: &PreferenceSeed) {}

    pub fn resolve_locale(_session: &AppSession, _seed: &PreferenceSeed) -> Option<String> {
        None
    }

    pub fn resolve_theme(_session: &AppSession) -> Option<String> {
        None
    }

    pub fn update_locale(_session: &AppSession, _locale: &str) {}

    pub fn update_theme(_session: &AppSession, _theme: &str) {}
}

#[cfg(not(feature = "server"))]
pub use client::*;
