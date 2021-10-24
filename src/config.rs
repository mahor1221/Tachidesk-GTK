// https://doc.rust-lang.org/cargo/reference/environment-variables.html
pub const APP_ID: &str = concat!("com.github.mahor1221.", env!("CARGO_PKG_NAME"));
pub const GRESOURCE_PREFIX: &str = concat!("/com/github/mahor1221/", env!("CARGO_PKG_NAME"));
pub const GRESOURCE_FILE: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/data/resources/resources.gresource");
pub const PKG_NAME: &str = env!("CARGO_PKG_NAME");
pub const PKG_VERSION: &str = env!("CARGO_PKG_VERSION");
pub const LOCALE_DIR: &str = "/usr/share/locale";


