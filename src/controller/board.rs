use anyhow::Context;
use reqwest::Url;

use crate::{
    controller::thread::{Thread, Threads},
    utils::{pattterns, requester::Reqch},
};

#[derive(Debug)]
pub struct Board {
    url:  Url,
    html: String,
}

impl Board {
    pub async fn new(url: &str) -> anyhow::Result<Board> {
        //  https://<server_name>/<board_key>/subback.html
        let url = url::Url::parse(&url).expect("url parse error");
        // スレッド一覧を取得
        let html = Reqch::new(&url.as_str()).await?.get_html();

        anyhow::Ok(Board { url, html })
    }

    pub fn get_url(&self) -> &Url { &self.url }

    pub fn get_server_name(&self) -> String { self.url.host_str().unwrap().to_string() }

    pub fn get_board_key(&self) -> String { self.url.path().split("/").nth(1).unwrap().to_string() }

    pub async fn parse(&self) -> anyhow::Result<Threads> {
        let board_key = self.get_board_key();
        let server_name = self.get_server_name();

        let mut threads = Vec::new();
        for c in pattterns::parse_thread_list(&self.html) {
            let group = (
                c.name("id").context("")?.as_str(),
                c.name("title").context("")?.as_str(),
                c.name("count").context("")?.as_str().parse::<usize>()?,
            );
            match group {
                (id, title, count) => {
                    let thread = Thread::new(&server_name, &board_key, id, title, count).await;
                    threads.push(thread);
                }
            }
        }

        anyhow::Ok(threads)
    }

    pub async fn laod(&self) -> anyhow::Result<Self> {
        let html = Reqch::new(&self.url.as_str()).await?.get_html();
        anyhow::Ok(Board {
            url:  self.url.clone(),
            html: html.to_string(),
        })
    }
}
