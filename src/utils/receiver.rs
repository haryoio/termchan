use anyhow::Context;

use crate::utils::{encoder, headers};

pub struct Reciever {
    url:  String,
    html: String,
}

impl Reciever {
    pub async fn new(url: &str) -> anyhow::Result<Reciever> {
        // URLからホスト名を取得
        let url = url.to_owned();
        let host = url::Url::parse(&url)
            .context("url parse error")?
            .host_str()
            .context("host parse error")?
            .to_string();

        let cookie = headers::gen_cookie(None);

        // 5chへリクエストするためのヘッダを生成
        let headers = headers::getable_headers(&host.clone(), &cookie.clone())?;
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

    pub fn get_url(&self) -> String { self.url.clone() }

    pub fn get_html(&self) -> String { self.html.clone() }
}
