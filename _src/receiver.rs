use anyhow::Context;
use reqwest::header::{HeaderName, COOKIE, HOST};

use crate::encoder;

pub struct Reciever {
    url:  String,
    html: String,
}

impl Reciever {
    pub async fn get(url: &str) -> anyhow::Result<Reciever> {
        // URLからホスト名を取得
        let url = url.to_owned();
        let host = url::Url::parse(&url)
            .context("url parse error")?
            .host_str()
            .context("host parse error")?
            .to_string();

        let cookie = vec![("READJS", "off"), ("SUBBACK_STYLE", "1")];
        let cookie = encoder::cookie_from_vec(cookie);

        let mut headers: Vec<(HeaderName, String)> = encoder::base_headers();
        headers.append(&mut vec![(HOST, host.to_string()), (COOKIE, cookie)]);
        let headers = encoder::headers_from_vec(headers)?;

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .context("failed to create client")?
            .get(&url);

        // バイナリで取得したHTMLをUTF-8に変換
        let res = client.send().await.context("failed to get html")?;
        let bytes = res.bytes().await.context("failed to get bytes")?;
        let html = encoder::sjis_to_utf8(&bytes)?;
        Ok(Self {
            url: url.to_string(),
            html,
        })
    }

    pub fn url(&self) -> String {
        self.url.clone()
    }

    pub fn html(&self) -> String {
        self.html.clone()
    }
}
