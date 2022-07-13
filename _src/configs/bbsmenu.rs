use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct BBSMenuConfig {
    pub url: Vec<String>,
}

impl Default for BBSMenuConfig {
    fn default() -> Self {
        BBSMenuConfig { url: vec![] }
    }
}
