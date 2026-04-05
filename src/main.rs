use dioxus::prelude::*;

mod components;
mod impls;
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
    LaunchBuilder::web().launch(App);
}
