use anyhow::Context;
use reqwest::header::{HeaderName, CONTENT_TYPE, COOKIE, HOST, ORIGIN, REFERER};

use crate::config::config::Config;

pub struct Login {
    url: String,
}

pub struct LoginSession {}

impl Login {
    pub fn do_login() {
        let config = Config::new(None);
        let config = config.load().unwrap();
        if config.login.is_none() {
            return;
        }
        let url = config.url.clone();
        let user = config.user.clone();
        let password = config.password.clone();
        if url.is_empty() || user.is_empty() || password.is_empty() {
            return;
        }

        // TODO: ログインページでPHPSESSIDを取得してCookieに設定する

        // TODO: フォームバリューを設定してPOSTする

        // TODO: 返ってきたページからクッキーを取得して保存する。
    }

    pub fn url(&self) -> String { self.url.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_do_login() { Login::do_login(); }
}
