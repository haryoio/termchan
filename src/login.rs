use std::sync::Arc;

use anyhow::Context;
use reqwest::header::{CONTENT_TYPE, COOKIE, HOST, REFERER};
use reqwest_cookie_store::CookieStoreMutex;

use crate::{
    configs::config::Config,
    utils::{cookie::CookieStore, encoder},
};

pub struct Login {
    url: String,
}

impl Login {
    pub async fn do_login() -> anyhow::Result<()> {
        let config = Config::load().await.context("failed to load config")?;
        let config = config.login.as_ref().context("login is not set")?;
        let url = config.url.as_ref().context("url is not set")?;

        // ログイン用のホスト名を取得
        let host = url::Url::parse(&url)
            .context("url parse error")?
            .host_str()
            .context("host parse error")?
            .to_string();

        // cookieJarを作成
        let cookie_store: Arc<CookieStoreMutex> = CookieStore::load().await.unwrap();

        let headers = encoder::headers_from_vec(encoder::base_headers())?;
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .cookie_provider(Arc::clone(&cookie_store))
            .build()
            .context("failed to create client")?;

        let email = config.email.as_ref().context("email is not set")?;
        let password = config.password.as_ref().context("password is not set")?;
        // ログイン用のフォームデータを生成
        let form = vec![
            ("em", email.as_str()),
            ("pw", password.as_str()),
            ("login", ""),
        ];
        let form_data = encoder::formvalue_from_vec(form)?;

        // クッキーを生成
        let cookie_vec = vec![("READJS", "\"off\""), ("yuki", "akari")];
        let cookie = encoder::cookie_from_vec(cookie_vec.clone());

        // ログインリクエスト用のURLを生成
        let post_url = format!("https://{}/log.php", host);
        let resp = client
            .post(&post_url)
            .header(HOST, host.to_string())
            .header(REFERER, url)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(COOKIE, cookie)
            .body(form_data)
            .send()
            .await
            .unwrap();

        let html = resp.text().await.unwrap();
        CookieStore::save(Arc::clone(&cookie_store)).await.unwrap();
        if html.contains("ログインしました") || html.contains("ログインしています")
        {
            Ok(())
        } else {
            Err(anyhow::anyhow!("failed to login"))
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
    async fn test_do_login() -> anyhow::Result<()> {
        let _ = Login::do_login().await.unwrap();
        let cookie = CookieStore::load_raw().await.unwrap();
        println!("{:?}", cookie);
        // let (name, value) = Login::to_tuple(&Some(session.to_owned()));
        // println!("cookie {}={}", name, value);
        Ok(())
    }

    #[tokio::test]
    async fn test_get_cookie() -> anyhow::Result<()> {
        let host = "mi.5ch.net";
        let _ = Login::do_login().await.unwrap();
        let cookie = CookieStore::load().await.unwrap();

        let cookie_store = &cookie.lock().unwrap();
        let domain = host.split('.').collect::<Vec<&str>>()[1..].join(".");
        println!("domain: {}", domain);
        let cookie_value = cookie_store.get(&domain, "/", "sid").unwrap();
        println!("cookie_value {:?}", cookie_value);
        Ok(())
    }
}
