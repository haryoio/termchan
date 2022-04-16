use anyhow::Context;
use reqwest::header::{HeaderName, CONTENT_TYPE, COOKIE, HOST, ORIGIN, REFERER};

use crate::encoder;

// TODO: ビルダーパターンで書き直す
// TODO: ログイン, 名前, メールをビルドパラメータで渡す
// let client = termch::sender::SenderBuilder::new()
//      .message("test")
//      .name(None)
//      .login(vec![(mail, secret)])
//      .build();
// !書き込み時のエラー（他所でやってください）などはここで捕まえる
// let res = client.send().await?;

pub struct Sender {
    url: String,
}

impl Sender {
    pub fn new(url: &str) -> anyhow::Result<Sender> {
        Ok(Sender {
            url: url.to_string(),
        })
    }
    // https://<host>/test/read.cgi/<board_key>/<thread_id>/
    pub async fn send(
        &self,
        message: &str,
        name: Option<&str>,
        mail: Option<&str>,
    ) -> anyhow::Result<()> {
        let url: &str = &self.url;
        let host = self.get_host();
        let thread_id = self.get_thread_id();
        let board = self.get_board_key();
        let referer = format!("{}", url); // referer: https://<host>/test/read.cgi/<board_key>/<thread_id>/
        let origin = format!("https://{}", &self.get_host()); // origin: https://<host>
        let post_url = format!("{}/test/bbs.cgi", &origin); // post_url: https://<host>/test/bbs.cgi
        let time = self.get_time().to_string(); // time: unixtime

        let cookie = vec![("yuki", "akari")];
        let cookie = encoder::cookie_from_vec(cookie);

        let content_type = "application/x-www-form-urlencoded".to_string();
        let mut headers: Vec<(HeaderName, String)> = encoder::base_headers();
        headers.append(&mut vec![
            (HOST, host.to_string()),
            (ORIGIN, origin),
            (REFERER, referer),
            (CONTENT_TYPE, content_type),
            (COOKIE, cookie),
        ]);
        let headers = encoder::headers_from_vec(headers)?;

        let name = name.unwrap_or("");
        let mail = mail.unwrap_or("");

        // form-data形式のデータを作成
        let form = vec![
            ("FROM", name),
            ("mail", mail),
            ("MESSAGE", &message),
            ("bbs", &board),
            ("key", &thread_id),
            ("time", &time),
            ("submit", "書き込む"),
            ("oekaki_thread1", ""),
        ];
        let form_data = encoder::formvalue_from_vec(form)?;

        let client = reqwest::Client::builder()
            .default_headers(headers.clone())
            .cookie_store(true)
            .build()
            .context("failed to build client")?;

        let post = move || client.post(&post_url).body(form_data.clone()).send();

        let res = post()
            .await
            .context("failed to send request")?
            .text()
            .await
            .context("failed to get response")?;

        if res.contains("書き込み確認") {
            post()
                .await
                .context("failed to send request")?
                .text()
                .await
                .context("failed to get response")?;
        }

        if res.contains("書き込みました。") {
            println!("write success");
        } else {
            println!("write failed");
            println!("res: {}", res);
        }

        anyhow::Ok(())
    }

    pub fn get_time(&self) -> f64 {
        let now = std::time::SystemTime::now();
        let now = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        now as f64
    }

    pub fn get_board_key(&self) -> String {
        self.url.split("/").nth(5).unwrap().to_string()
    }

    pub fn get_thread_id(&self) -> String {
        self.url.split("/").nth(6).unwrap().to_string()
    }

    pub fn get_host(&self) -> String {
        self.url.split("/").nth(2).unwrap().to_string()
    }
}
