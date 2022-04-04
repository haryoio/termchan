use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub host:     String,
    pub port:     u16,
    pub user:     String,
    pub password: String,
}

impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig {
            host:     "".to_string(),
            port:     0,
            user:     "".to_string(),
            password: "".to_string(),
        }
    }
}
