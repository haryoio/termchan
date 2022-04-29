use crate::{
    configs::config::Config,
    login::Login,
    utils::{
        cookie::CookieStore,
        encoder,
        patterns::{get_error_message, get_url_write_success},
        receiver::Reciever,
    },
};
use anyhow::Context;
use reqwest::header::{HeaderName, CONTENT_TYPE, COOKIE, HOST, ORIGIN, REFERER};

pub struct Sender {
    url: String,
    login: bool,
    proxy: bool,
    user_agent: String,
}

impl Sender {
    pub fn new(url: String) -> Sender {
        Sender {
            url,
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

    pub async fn send(
        &self,
        title: &str,
        name: Option<&str>,
        mail: Option<&str>,
        message: &str,
    ) -> anyhow::Result<String> {
        let host = self.url.split("/").nth(2).unwrap();
        let board_name = self.url.split("/").nth(3).unwrap();
        let referer = format!("{}", self.url);
        let origin = self.url.split("/").collect::<Vec<_>>()[..3].join("/");
        let post_url = format!("{}/test/bbs.cgi", &origin);
        let time = self.get_time().to_string();

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
        let (title, ..) = encoding_rs::SHIFT_JIS.encode(title);
        let title = title.to_vec();
        let title = unsafe { &*std::str::from_utf8_unchecked(&title) };

        let (cert, site) = self.get_cert_and_site().await;

        // form-data形式のデータを作成
        let form = vec![
            ("submit", "新規スレッド作成"),
            ("subject", title),
            ("FROM", name),
            ("mail", mail),
            ("MESSAGE", &message),
            ("site", &site),
            ("bbs", board_name),
            ("time", &time),
            ("cert", &cert),
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

    pub async fn get_cert_and_site(&self) -> (String, String) {
        let mut cert = String::new();
        let mut site = String::new();
        let url = format!("{}/#new_thread", &self.url);
        let res = Reciever::get(&url).await.unwrap().html();
        for l in res.lines().rev() {
            if l.starts_with("<input type=\"hidden\" name=\"bbs\"") {
                let i = l.split("<input type=\"hidden\" ").collect::<Vec<_>>();
                for cc in i {
                    if cc.contains("name=\"") {
                        let cc = cc.split("\"").collect::<Vec<_>>();
                        if cc[1] == "cert" {
                            cert = cc[3].to_string();
                        }
                    }
                }
            }
            if l.starts_with("<input type=\"hidden\" name=\"site\"") {
                let i = l.split("<input type=\"hidden\" ").collect::<Vec<_>>();
                for cc in i {
                    if cc.contains("name=\"") {
                        let cc = cc.split("\"").collect::<Vec<_>>();
                        if cc[1] == "site" {
                            site = cc[3].to_string();
                        }
                    }
                }
            }
            if l.starts_with("<textarea") {
                break;
            }
        }
        (cert, site)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_send() {
        let url = "https://mi.termchan.net/news4vip/";
        let sender = Sender::new(url.to_string())
            .send("てすと", None, None, "てすと")
            .await
            .unwrap();
        println!("{}", sender);
    }

    #[tokio::test]
    async fn test_get_cert() {
        let url = "https://mi.termchan.net/news4vip/";
        let sender = Sender::new(url.to_string()).get_cert_and_site().await;
        println!("{:?}", sender);
    }
}
