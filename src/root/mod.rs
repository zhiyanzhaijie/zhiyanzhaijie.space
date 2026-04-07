pub mod layouts;
mod routes;

use crate::components::animated_bird::AnimatedBird;
use crate::components::providers::preference_provider::{
    locale_to_langid, resolve_locale, resolve_theme, PreferenceProvider,
};
use crate::IO::user::get_preference;
use crate::IO::user::SessionPreferenceDto;
use crate::impls::i18n as app_i18n;
use dioxus::document::{Link, Stylesheet};
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
pub use routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TW_STYLES: Asset = asset!("/assets/tailwind.css");
const NOISE_IMAGE: Asset = asset!("/assets/noise.png");

#[allow(non_snake_case)]
pub fn App() -> Element {
    use_init_i18n(app_i18n::build_i18n_config);
    let preference_fut = use_server_future(get_preference)?;
    let initial_preference = match preference_fut() {
        Some(Ok(preference)) => preference,
        _ => SessionPreferenceDto::default(),
    };
    let initial_theme_str = resolve_theme(initial_preference.theme.as_deref());
    let initial_locale_str = resolve_locale(initial_preference.locale.as_deref());
    let mut i18n = i18n();
    i18n.set_language(locale_to_langid(Some(initial_locale_str)));
    let bootstrap_script = format!(
        r#"
(function () {{
  var root = document.documentElement;
  root.setAttribute("class", "{initial_theme_str}");
  root.setAttribute("lang", "{initial_locale_str}");
}})();
"#
    );

    rsx! {
        document::Script { {bootstrap_script} }
        title { "zhiyanzhaijie" }
        Link { rel: "icon", href: FAVICON }
        Stylesheet { href: TW_STYLES }
        PreferenceProvider {
            initial: initial_preference,
            div {
                class: "pointer-events-none fixed inset-0 bg-repeat opacity-[0.035] dark:opacity-[0.020]",
                style: "background-image: url({NOISE_IMAGE}); background-size: 180px;",
            }
            AnimatedBird {}
            Router::<Route> {}
        }
    }
}
