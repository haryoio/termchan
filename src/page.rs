use std::str::Bytes;

use crate::utils::{encoder, headers};

pub struct Page {
    url: String,
    html: String,
}

impl Page {
    pub async fn new(url: &str) -> Page {
        // URLからホスト名を取得
        let url = url.to_owned();
        let host = url::Url::parse(&url)
            .unwrap()
            .host_str()
            .unwrap()
            .to_string();

        let cookie = headers::gen_cookie(None);

        // 5chへリクエストするためのヘッダを生成
        let headers = headers::getable_headers(&host.clone(), &cookie.clone());
        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .unwrap();

        // バイナリで取得したHTMLをUTF-8に変換
        let mut html = String::new();
        match client.get(&url).send().await {
            Ok(res) => {
                html = encoder::sjis_to_utf8(&res.bytes().await.unwrap());
            }
            Err(e) => {
                println!("{}", e);
            }
        };
        Page {
            url: url.to_string(),
            html,
        }
    }

    pub fn get_url(&self) -> String {
        self.url.clone()
    }

    pub fn get_html(&self) -> String {
        self.html.clone()
    }
}
