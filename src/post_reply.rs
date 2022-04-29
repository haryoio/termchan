use crate::{
    configs::config::Config,
    login::Login,
    services::thread::Thread,
    utils::{
        cookie::CookieStore,
        encoder,
        patterns::{get_error_message, get_url_write_success},
    },
};
use anyhow::Context;
use reqwest::header::{HeaderName, CONTENT_TYPE, COOKIE, HOST, ORIGIN, REFERER};

pub struct Sender {
    thread: Thread,
    login: bool,
    proxy: bool,
    user_agent: String,
}

impl Sender {
    pub fn new(thread: &Thread) -> Sender {
        Sender {
            thread: thread.clone(),
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

    pub fn user_agent(&mut self, user_agent: &str) -> &Self {
        self.user_agent = user_agent.to_string();
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

        if self.login {
            let _ = Login::do_login().await.unwrap();
        };
        let cache = CookieStore::load_raw().await.unwrap_or_default();
        let cookie_vec = vec![("READJS", "\"off\""), ("yuki", "akari")];
        let mut cookie = encoder::cookie_from_vec(cookie_vec.clone());
        cookie.push_str("; ");
        cookie.push_str(&cache);

        let content_type = "application/x-www-form-urlencoded".to_string();

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

        let proxy = if self.proxy {
            let config = Config::load().await.context("failed to load config")?;
            let config = config.proxy;
            match config {
                Some(proxy) => Some(proxy.build()),
                None => None,
            }
        } else {
            None
        };

        let mut headers: Vec<(HeaderName, String)> = encoder::base_headers();
        headers.append(&mut vec![
            (HOST, host.to_string()),
            (ORIGIN, origin),
            (REFERER, referer),
            (CONTENT_TYPE, content_type),
            (COOKIE, cookie),
        ]);
        let headers = encoder::headers_from_vec(headers)?;

        let client = reqwest::Client::builder().default_headers(headers);

        let client = match proxy {
            Some(proxy) => client
                .proxy(proxy)
                .build()
                .context("failed to build client")?,
            None => client.build().context("failed to build client")?,
        };

        let res = client
            .post(&post_url)
            .body(form_data.clone())
            .send()
            .await
            .context("failed to send request")?
            .text()
            .await
            .context("failed to get response")?;

        if res.contains("書き込み確認") {
            client
                .post(&post_url)
                .body(form_data.clone())
                .send()
                .await
                .context("failed to send request")?
                .text()
                .await
                .context("failed to get response")?;
        }

        if !res.contains("ERROR") {
            let url = get_url_write_success(&res).unwrap();
            Ok(url)
        } else {
            let error = get_error_message(&res).unwrap();
            Err(anyhow::anyhow!("{}", error))
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
    use chrono::Local;

    use crate::services::board::Board;

    use super::*;

    #[tokio::test]
    async fn test_send() {
        let url = "https://mi.termchan.net/news4vip/";
        let threads = Board::new(url.to_string()).load().await.unwrap();
        let thread = &*threads.get(10).unwrap();
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

    #[tokio::test]
    async fn send_proxy() {
        let url = "https://mi.5ch.net/news4vip/";
        let threads = Board::new(url.to_string()).load().await.unwrap();
        let thread = &*threads.get(10).unwrap();
        println!("{:?}", thread);
        let message = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();
        let sender = Sender::new(thread)
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
}
