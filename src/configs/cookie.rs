use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct CookieConfig {
    pub path: String,
}

impl Default for CookieConfig {
    fn default() -> Self {
        let path = dirs::home_dir()
            .unwrap()
            .join(".config")
            .join("termchan")
            .join("cookies.json")
            .to_str()
            .unwrap()
            .to_string();
        CookieConfig { path }
    }
}
