use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CookieConfig {
    pub path: String,
}

impl Default for CookieConfig {
    fn default() -> Self {
        CookieConfig {
            path: "$HOME/termchan/cookie.json".to_string(),
        }
    }
}
