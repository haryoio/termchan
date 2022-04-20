use std::sync::Arc;

use anyhow::Context;
use cookie_store::{Cookie, Error};
use reqwest::header::{CONTENT_TYPE, COOKIE, HOST, REFERER};
use reqwest_cookie_store::CookieStoreMutex;

use crate::{configs::config::Config, cookie::CookieStore, encoder, error::TermchanError};

pub struct Login {
    url: String,
}

impl Login {
    pub async fn do_login() -> Result<Arc<CookieStoreMutex>, TermchanError> {
        let config = Config::load().context("failed to load config")?;
        let config = config.login.as_ref().context("login is not set")?;
        let url = config.url.as_ref().context("url is not set")?;
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
            .get(url)
            .header(HOST, host.to_string())
            .header(COOKIE, &cookie)
            .send()
            .await
            .context("failed to get html")?;

        // セッションが有効ならログイン処理をスキップ
        if resp.text().await.unwrap().contains("ログインしています") {
            return Ok(cookie_store);
        }

        let email = config.email.as_ref().context("email is not set")?;
        let password = config.password.as_ref().context("password is not set")?;
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
            .header(REFERER, url)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .body(form_data)
            .send()
            .await
            .context("failed to get html")?;

        // Cookieをファイルへ保存
        CookieStore::save(cookie_store.clone()).unwrap();

        let html = &resp.text().await.context("failed to get html")?;
        if html.contains("ログインできません") {
            return Err(TermchanError::LoginError("ログインできません".to_string()));
        } else if html.contains("ログインしました") {
            Ok(cookie_store)
        } else if html.contains("ログインしています") {
            Ok(cookie_store)
        } else {
            Err(TermchanError::LoginError("ログインできません".to_string()))
        }
    }

    pub fn url(&self) -> &str {
        &self.url
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
        let session = Login::do_login().await.unwrap();
        println!("session {:?}", session);
        // let (name, value) = Login::to_tuple(&Some(session.to_owned()));
        // println!("cookie {}={}", name, value);
        Ok(())
    }
}
