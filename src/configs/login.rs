use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginConfig {
    pub url:      Option<String>,
    pub email:    Option<String>,
    pub password: Option<String>,
}

impl Default for LoginConfig {
    fn default() -> Self {
        LoginConfig {
            url:      None,
            email:    None,
            password: None,
        }
    }
}
