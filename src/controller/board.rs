use anyhow::Context;
use reqwest::Url;

use crate::{
    controller::thread::{Thread, Threads},
    patterns,
    receiver::Reciever,
};

fn normalize_board(html: &str, url: String) -> anyhow::Result<Vec<Thread>> {
    let url = Url::parse(&url).context("failed to parse url")?;
    let server_name = url.host_str().unwrap().to_string();
    let board_key = url.path().split("/").nth(1).unwrap().to_string();
    let mut threads = Vec::new();
    for c in patterns::parse_thread_list(&html) {
        let group = (
            c.name("id").context("")?.as_str(),
            c.name("title").context("")?.as_str(),
            c.name("count").context("")?.as_str().parse::<usize>()?,
        );
        match group {
            (id, title, count) => {
                let thread = Thread::new(&server_name, &board_key, id, title, count);
                threads.push(thread);
            }
        }
    }
    Ok(threads)
}

#[derive(Debug)]
pub struct Board {
    pub url: String,
}

impl Board {
    pub fn new(url: String) -> Self {
        let mut url = url.clone();
        if !url.ends_with("subback.html") {
            if !url.ends_with("/") {
                url.push_str("/");
            }
            url.push_str("subback.html");
        };
        Self { url }
    }
    pub async fn load(&self) -> anyhow::Result<Vec<Thread>> {
        let html = Reciever::get(&self.url).await?.html();

        anyhow::Ok(normalize_board(html.as_str(), self.url.clone())?)
    }
}
