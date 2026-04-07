use dioxus::prelude::*;
#[allow(non_snake_case)]
mod IO;

mod components;
mod impls;
mod models;
mod root;
mod utils;

use crate::root::App;

#[cfg(feature = "server")]
#[tokio::main]
async fn main() {
    use axum_session::{SameSite, SessionConfig, SessionLayer, SessionMode, SessionStore};
    use dioxus_server::DioxusRouterExt;
    use impls::session::consts::SESSION_COOKIE_NAME;

    let server_addr = dioxus::cli_config::fullstack_address_or_localhost();
    let is_production = std::env::var("APP_ENV").ok().as_deref() == Some("production");
    let session_config = SessionConfig::default()
        .with_session_name(SESSION_COOKIE_NAME)
        .with_mode(SessionMode::OptIn)
        .with_cookie_same_site(SameSite::Lax)
        .with_http_only(true)
        .with_secure(is_production);

    let session_store = SessionStore::<axum_session::SessionNullPool>::new(None, session_config)
        .await
        .expect("init session store failed");

    let router = axum::Router::new()
        .serve_dioxus_application(ServeConfig::default(), App)
        .layer(SessionLayer::new(session_store));

    let listener = tokio::net::TcpListener::bind(server_addr)
        .await
        .expect("bind server addr failed");
    axum::serve(listener, router).await.expect("server error");
}

#[cfg(not(feature = "server"))]
fn main() {
    LaunchBuilder::web().launch(App);
}
