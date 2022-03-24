use std::cell::RefCell;

use reqwest::Url;

use crate::{
    models::thread::{Thread, Threads},
    page::Page,
    utils::parser,
};

#[derive(Debug)]
pub struct Board {
    url: Url,
    html: String,
}

impl Board {
    pub async fn new(url: &str) -> Board {
        //  https://<server_name>/<board_key>/subback.html
        let url = url::Url::parse(&url).expect("url parse error");
        // スレッド一覧を取得
        let html = Page::new(&url.as_str()).await.get_html();

        Board {
            url,
            html: html.to_string(),
        }
    }

    pub fn get_url(&self) -> &Url {
        &self.url
    }

    pub fn get_server_name(&self) -> String {
        self.url.host_str().unwrap().to_string()
    }

    pub fn get_board_key(&self) -> String {
        self.url.path().split("/").nth(1).unwrap().to_string()
    }

    pub async fn parse(&self) -> Threads {
        let board_key = self.get_board_key();
        let server_name = self.get_server_name();

        let mut threads = Vec::new();
        for cap in parser::parse_thread_list(&self.html) {
            let group = (
                cap.name("id").unwrap().as_str(),
                cap.name("title").unwrap().as_str(),
                cap.name("count")
                    .unwrap()
                    .as_str()
                    .parse::<usize>()
                    .unwrap(),
            );
            match group {
                (id, title, count) => {
                    let thread = Thread::new(&server_name, &board_key, id, title, count).await;
                    threads.push(thread);
                }
            }
        }

        threads
    }

    pub async fn laod(&self) -> Self {
        let html = Page::new(&self.url.as_str()).await.get_html();
        Board {
            url: self.url.clone(),
            html: html.to_string(),
        }
    }
}
