use anyhow::Context;
use reqwest::{
    cookie::{self, Cookie},
    header::{HeaderName, CONTENT_TYPE, COOKIE, HOST, ORIGIN, REFERER, SET_COOKIE},
};

use crate::{config::config::Config, encoder};

pub struct Login {
    url: String,
}

impl Login {
    pub async fn do_login() -> anyhow::Result<()> {
        let config = Config::new(None);
        let config = config.load().unwrap();
        if config.login.is_none() {
            return Err(anyhow::anyhow!("login is not set"));
        }
        let url = config.url.clone();
        let email = config.email.clone();
        let password = config.password.clone();
        if url.is_empty() || email.is_empty() || password.is_empty() {
            return Err(anyhow::anyhow!("login info is not set"));
        }

        // TODO: ログインページでPHPSESSIDを取得してCookieに設定する
        let host = url::Url::parse(&url)
            .context("url parse error")?
            .host_str()
            .context("host parse error")?
            .to_string();

        let mut cookie_vec = vec![("READJS", "\"off\""), ("yuki", "akari")];
        let cookie = encoder::cookie_from_vec(cookie_vec.clone());

        let headers = encoder::headers_from_vec(encoder::base_headers())?;

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .cookie_store(true)
            .build()
            .context("failed to create client")?;

        let res = client
            .get(&url)
            .header(HOST, host.to_string())
            .header(COOKIE, &cookie)
            .send()
            .await
            .context("failed to get html")?;
        let phpsessid = res.cookies().next().unwrap();
        let phpsessid = phpsessid.value();
        cookie_vec.push(("PHPSESSID", phpsessid));
        let cookie = encoder::cookie_from_vec(cookie_vec);

        // TODO: フォームバリューを設定してPOSTする
        let form = vec![
            ("em", email.as_str()),
            ("pw", password.as_str()),
            ("login", ""),
        ];
        let form_data = encoder::formvalue_from_vec(form)?;
        println!("{}", form_data);

        let post_url = format!("https://{}/log.php", host);
        let res = client
            .post(&post_url)
            .header(HOST, host.to_string())
            .header(REFERER, &url)
            .header(CONTENT_TYPE, "application/x-www-form-urlencoded")
            .header(COOKIE, &cookie)
            .body(form_data)
            .send()
            .await
            .context("failed to get html")?;

        // TODO: ログイン成功したらCookieを保存する

        let html = &res.text().await.context("failed to get html")?;
        if html.contains("ログインできません") {
            return Err(anyhow::anyhow!(
                "ERROR: login failed (email or password is wrong)"
            ));
        } else if html.contains("ログインしました") {
            println!("INFO: login success");
            Ok(())
        } else {
            return Err(anyhow::anyhow!("ERROR: login failed"));
        }

        // TODO: 返ってきたページからクッキーを取得して保存する。
    }

    pub fn url(&self) -> String { self.url.clone() }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_do_login() -> anyhow::Result<()> {
        Login::do_login().await.unwrap();
        Ok(())
    }
}
