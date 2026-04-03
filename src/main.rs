use dioxus::prelude::*;

mod components;
mod models;
mod root;
mod utils;

use crate::root::App;

#[cfg(feature = "server")]
fn main() {
    LaunchBuilder::server().launch(App);
}

#[cfg(not(feature = "server"))]
fn main() {
    #[cfg(target_arch = "wasm32")]
    {
        match console_log::init_with_level(log::Level::Debug) {
            Ok(_) => {
                log::info!("console_log initialized successfully at level: Debug");
            }
            Err(e) => {
                eprintln!("Error initializing console_log: {:?}", e);
            }
        }
    }
    LaunchBuilder::web().launch(App);
}
