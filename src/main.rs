mod banner;
mod meiling;

use std::sync::{RwLock, PoisonError};
use confy;
use serde::{Serialize, Deserialize};
use surf;

pub const VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const MEILING_HOST: &'static str = "https://meiling.stella-api.dev";

#[derive(Serialize, Deserialize)]
struct MyConfig {
    version: u8,
    access_token: Option<String>,
    refresh_token: Option<String>,
}

/// `MyConfig` implements `Default`
impl ::std::default::Default for MyConfig {
    fn default() -> Self {
        Self {
            version: 1,
            access_token: None,
            refresh_token: None,
        }
    }
}

async fn main() {
    banner::print_banner();

    let config: MyConfig = confy::load("acme4console").unwrap();
    let mut access_token_available = config.access_token.is_some();

    if access_token_available {
        let res = surf::post()
    }
}
