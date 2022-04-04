use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct PostConfig {
    pub use_login:       bool,
    pub repost_interval: u64,
}

impl Default for PostConfig {
    fn default() -> Self {
        PostConfig {
            use_login:       false,
            repost_interval: 5,
        }
    }
}
