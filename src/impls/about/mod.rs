#[cfg(feature = "server")]
mod service;

#[cfg(feature = "server")]
pub use service::get_about_markdown;
