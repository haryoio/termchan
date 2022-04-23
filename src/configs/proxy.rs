use std::fmt::Display;

use reqwest::Proxy;
use serde::{Deserialize, Serialize};
#[derive(Debug, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub proxy_scheme: String,
    pub username: String,
    pub password: String,
}
impl ProxyConfig {
    pub fn new(proxy_scheme: String, username: String, password: String) -> Self {
        Self {
            proxy_scheme,
            username,
            password,
        }
    }
    pub fn build(&self) -> Proxy {
        let proxy_scheme = self.proxy_scheme.clone();
        Proxy::https(proxy_scheme)
            .unwrap()
            .basic_auth(self.username.as_str(), self.password.as_str())
    }
}

impl Default for ProxyConfig {
    fn default() -> Self {
        ProxyConfig {
            proxy_scheme: String::new(),
            username: String::new(),
            password: String::new(),
        }
    }
}
