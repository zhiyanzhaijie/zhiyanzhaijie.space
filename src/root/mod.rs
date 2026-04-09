pub mod layouts;
mod routes;

use crate::components::animated_bird::AnimatedBird;
use crate::components::providers::interactive_provider::InteractiveProvider;
use crate::components::providers::preference_provider::{
    locale_to_langid, resolve_locale, resolve_theme, PreferenceProvider,
};
use crate::impls::i18n as app_i18n;
use crate::IO::user::get_preference;
use crate::IO::user::SessionPreferenceDto;
use dioxus::document::{Link, Stylesheet};
use dioxus::prelude::*;
use dioxus_i18n::prelude::*;
pub use routes::Route;

const FAVICON: Asset = asset!("/assets/favicon.ico");
const TW_STYLES: Asset = asset!("/assets/tailwind.css");
const FONT_CSS: Asset = asset!("/assets/font.css");
const NOISE_IMAGE: Asset = asset!("/assets/noise.png");
const IA_WRITER_QUATTRO_REGULAR: Asset = asset!(
    "/assets/fonts/iAWriterQuattroS-Regular.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const IA_WRITER_MONO_REGULAR: Asset = asset!(
    "/assets/fonts/iAWriterMonoS-Regular.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const _IA_WRITER_QUATTRO_ITALIC: Asset = asset!(
    "/assets/fonts/iAWriterQuattroS-Italic.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const _IA_WRITER_QUATTRO_BOLD: Asset = asset!(
    "/assets/fonts/iAWriterQuattroS-Bold.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const _IA_WRITER_MONO_ITALIC: Asset = asset!(
    "/assets/fonts/iAWriterMonoS-Italic.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const _IA_WRITER_MONO_BOLD: Asset = asset!(
    "/assets/fonts/iAWriterMonoS-Bold.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);
const LXGW_REGULAR: Asset = asset!(
    "/assets/fonts/LXGWWenKaiLite-Regular.woff2",
    AssetOptions::builder()
        .with_hash_suffix(false)
        .into_asset_options()
);

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
        Link {
            rel: "preload",
            href: IA_WRITER_QUATTRO_REGULAR,
            r#as: "font",
            r#type: "font/woff2",
            crossorigin: "anonymous",
        }
        Link {
            rel: "preload",
            href: IA_WRITER_MONO_REGULAR,
            r#as: "font",
            r#type: "font/woff2",
            crossorigin: "anonymous",
        }
        Link {
            rel: "preload",
            href: LXGW_REGULAR,
            r#as: "font",
            r#type: "font/woff2",
            crossorigin: "anonymous",

        }
        Link { rel: "icon", href: FAVICON }
        Stylesheet { href: TW_STYLES }
        Stylesheet { href: FONT_CSS }
        PreferenceProvider { initial: initial_preference,
            div {
                class: "pointer-events-none fixed inset-0 bg-repeat opacity-[0.035] dark:opacity-[0.020]",
                style: "background-image: url({NOISE_IMAGE}); background-size: 180px;",
            }
            AnimatedBird {}
            InteractiveProvider {
                Router::<Route> {}
            }
        }
    }
}
