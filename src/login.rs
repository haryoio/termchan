use std::sync::Arc;

use anyhow::Context;
use reqwest::header::{CONTENT_TYPE, COOKIE, HOST, REFERER};
use reqwest_cookie_store::CookieStoreMutex;

use crate::{configs::config::Config, cookie::CookieStore, encoder};

pub struct Login {
    url: String,
}

impl Login {
    pub async fn do_login() -> anyhow::Result<()> {
        let config = Config::load().unwrap();
        let config = match config.login {
            Some(config) => config,
            None => return Ok(()),
        };

        let url = match config.url {
            Some(url) => url,
            None => return Ok(()),
        };
        // ログイン用のホスト名を取得
        let host = url::Url::parse(&url)
            .context("url parse error")?
            .host_str()
            .context("host parse error")?
            .to_string();

        let cookie_store: Arc<CookieStoreMutex> = CookieStore::load().unwrap().arc();

        let cookie_vec = vec![("READJS", "\"off\""), ("yuki", "akari")];
        let cookie = encoder::cookie_from_vec(cookie_vec.clone());
        let headers = encoder::headers_from_vec(encoder::base_headers())?;
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .cookie_provider(Arc::clone(&cookie_store))
            .build()
            .context("failed to create client")?;

        let resp = client
            .get(&url)
            .header(HOST, host.to_string())
            .header(COOKIE, &cookie)
            .send()
            .await
            .context("failed to get html")?;

        // セッションが有効ならログイン処理をスキップ
        if resp.text().await.unwrap().contains("ログインしています") {
            return Ok(());
        }

        let (email, password) = match (config.email, config.password) {
            (Some(email), Some(password)) => (email, password),
            _ => return Ok(()),
        };
        // ログイン画面のフォームデータを生成
        let form = vec![
            ("em", email.as_str()),
            ("pw", password.as_str()),
            ("login", ""),
        ];
        let form_data = encoder::formvalue_from_vec(form)?;

        // ログインリクエスト用のURLを生成
        let post_url = format!("https://{}/log.php", host);
        let resp = client
            .post(&post_url)
            .header(HOST, host.to_string())
            .header(REFERER, &url)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(form_data)
            .send()
            .await
            .context("failed to get html")?;

        // Cookieをファイルへ保存
        CookieStore::save(cookie_store.clone()).unwrap();

        let html = &resp.text().await.context("failed to get html")?;
        if html.contains("ログインできません") {
            return Err(anyhow::anyhow!(
                "ERROR: login failed (email or password is wrong)"
            ));
        } else if html.contains("ログインしました") {
            println!("INFO: login success");
            Ok(())
        } else if html.contains("ログインしています") {
            println!("INFO: already logged in");
            Ok(())
        } else {
            return Err(anyhow::anyhow!("ERROR: login failed"));
        }
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_clear() -> anyhow::Result<()> {
        CookieStore::clear().unwrap();
        Ok(())
    }

    #[tokio::test]
    async fn test_do_login() -> anyhow::Result<()> {
        Login::do_login().await.unwrap();
        Ok(())
    }
}
