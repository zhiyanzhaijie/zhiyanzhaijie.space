#[cfg(feature = "server")]
pub type AppSession = axum_session::Session<axum_session::SessionNullPool>;

#[cfg(not(feature = "server"))]
pub type AppSession = ();

pub mod consts;
pub mod preference;
