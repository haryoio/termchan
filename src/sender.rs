use std::{cell::Cell, fs, io::Write};

use crate::{controller::thread::Thread, login::Login};
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

pub struct Sender<'a> {
    thread: &'a Thread,
    login: Cell<bool>,
    proxy: Cell<bool>,
}

impl<'a> Sender<'a> {
    pub fn new(thread: &Thread) -> Sender {
        Sender {
            thread,
            login: Cell::new(false),
            proxy: Cell::new(false),
        }
    }

    pub fn login(&self, enable: bool) -> &Self {
        self.login.set(enable);
        self
    }

    pub fn proxy(&self, enable: bool) -> &Self {
        self.proxy.set(enable);
        self
    }

    // https://<host>/test/read.cgi/<board_key>/<thread_id>/
    pub async fn send(
        &self,
        message: &str,
        name: Option<&str>,
        mail: Option<&str>,
    ) -> anyhow::Result<String> {
        let host = &self.thread.server_name;
        let thread_id = &self.thread.id;
        let board = &self.thread.board_key;
        let url = format!("https://{}/test/read.cgi/{}/{}/", host, board, thread_id);
        let referer = format!("{}", url); // referer: https://<host>/test/read.cgi/<board_key>/<thread_id>/
        let origin = format!("https://{}", self.thread.server_name); // origin: https://<host>
        let post_url = format!("{}/test/bbs.cgi", &origin); // post_url: https://<host>/test/bbs.cgi
        let time = self.get_time().to_string(); // time: unixtime
        let cookie = vec![("yuki", "akari")];
        let cookie = encoder::cookie_from_vec(cookie);

        let co = Login::do_login().await.unwrap();
        let coo = &*co.lock().unwrap();
        let coo = coo.iter_any().collect::<Vec<_>>();
        let mut cc = String::new();
        for c in coo {
            if c.name() == "sid" {
                cc = format!("{}={}", c.name(), c.value());
            }
        }

        let cookie = format!("{}; {}", cookie, cc);

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
        let (message, ..) = encoding_rs::SHIFT_JIS.encode(message);
        let message = message.to_vec();
        let message = unsafe { &*std::str::from_utf8_unchecked(&message) };

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
            Ok("write success".to_string())
        } else {
            Ok("write failed".to_string())
        }
    }

    pub fn get_time(&self) -> f64 {
        let now = std::time::SystemTime::now();
        let now = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        now as f64
    }
}

#[cfg(test)]
mod tests {
    use crate::controller::board::Board;

    use super::*;

    #[tokio::test]
    async fn test_send() {
        let url = "https://mi.5ch.net/news4vip/";
        let threads = Board::new(url.to_string()).load().await.unwrap();
        let thread = &*threads.get(0).unwrap();
        println!("{:?}", thread);
        let message = "てすと";
        let sender = Sender::new(thread)
            .login(true)
            .send(message, None, None)
            .await
            .unwrap();
        println!("{}", sender);

        // let res = sender.send("test", None, None).await.unwrap();
        // assert_eq!(res, "write success");
    }
}
