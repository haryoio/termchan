use std::{cell::Cell, fs, io::Write};

use crate::{
    configs::config::Config, controller::thread::Thread, error::TermchanError, login::Login,
};
use anyhow::Context;
use reqwest::{
    header::{HeaderName, CONTENT_TYPE, COOKIE, HOST, ORIGIN, REFERER},
    Proxy,
};

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
    login: bool,
    proxy: bool,
    user_agent: String,
}

impl<'a> Sender<'a> {
    pub fn new(thread: &Thread) -> Sender {
        Sender {
            thread,
            login: false,
            proxy: false,
            user_agent: String::new(),
        }
    }

    pub fn login(&mut self, enable: bool) -> &Self {
        self.login = enable;
        self
    }

    pub fn proxy(&mut self, enable: bool) -> &Self {
        self.proxy = enable;
        self
    }

    // https://<host>/test/read.cgi/<board_key>/<thread_id>/
    pub async fn send(
        &self,
        message: &str,
        name: Option<&str>,
        mail: Option<&str>,
    ) -> Result<String, TermchanError> {
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

        let cookie_store = Login::do_login().await.unwrap();
        let cookie_store = &cookie_store.lock().unwrap();
        let cookie_store = cookie_store.iter_any().collect::<Vec<_>>();
        let mut sid = String::new();
        for c in cookie_store {
            if c.name() == "sid" {
                sid = format!("{}={}", c.name(), c.value());
            }
        }

        let cookie_login = format!("{}; {}", cookie, sid);

        let content_type = "application/x-www-form-urlencoded".to_string();
        let mut headers: Vec<(HeaderName, String)> = encoder::base_headers();
        headers.append(&mut vec![
            (HOST, host.to_string()),
            (ORIGIN, origin),
            (REFERER, referer),
            (CONTENT_TYPE, content_type),
            (COOKIE, if self.login { cookie_login } else { cookie }),
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

        let proxy = if self.proxy {
            let config = Config::load().context("failed to load config")?;
            let config = config.proxy;
            let config = match config {
                Some(proxy) => proxy.build(),
                None => return Err(TermchanError::ConfigError("proxy is not set".to_string())),
            };
            Some(config)
        } else {
            None
        };

        let client = match proxy {
            Some(proxy) => reqwest::Client::builder()
                .default_headers(headers.clone())
                .cookie_store(true)
                .proxy(proxy)
                .build()
                .context("failed to build client")?,
            None => reqwest::Client::builder()
                .default_headers(headers.clone())
                .cookie_store(true)
                .build()
                .context("failed to build client")?,
        };

        let form_data = encoder::formvalue_from_vec(form)?;
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
        if !res.contains("ERROR") {
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
    use chrono::{Date, Duration, Local, Utc};

    use crate::controller::board::Board;

    use super::*;

    #[tokio::test]
    async fn test_send() {
        let url = "https://mi.5ch.net/news4vip/";
        let threads = Board::new(url.to_string()).load().await.unwrap();
        let thread = &*threads.get(10).unwrap();
        let message = "ろぎん";
        let sender = Sender::new(thread)
            .login(true)
            .send(message, None, None)
            .await
            .unwrap();
        println!("{}", sender);

        // let res = sender.send("test", None, None).await.unwrap();
        // assert_eq!(res, "write success");
    }

    async fn send_proxy() {
        let url = "https://mi.5ch.net/news4vip/";
        let threads = Board::new(url.to_string()).load().await.unwrap();
        let thread = &*threads.get(10).unwrap();
        let message = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let sender = Sender::new(thread)
            // .login(true)
            .proxy(true)
            .send(&message, None, None)
            .await
            .unwrap();
        println!("---------");
        println!("writed at: {}", Local::now().format("%m-%d %H:%M:%S"));
        println!("title: {}", thread.title);
        println!("URL: {}", sender);

        // let res = sender.send("test", None, None).await.unwrap();
        // assert_eq!(res, "write success");
    }
    #[tokio::test]
    async fn looptest() {
        send_proxy().await;
    }
}
