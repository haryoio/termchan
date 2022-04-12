use anyhow::Context;
use reqwest::Url;

use crate::{
    controller::thread::{Thread, Threads},
    pattterns,
    receiver::Reciever,
};

async fn normalize_board(html: &str, url: String) -> anyhow::Result<Vec<Thread>> {
    let url = Url::parse(&url).context("failed to parse url")?;
    let server_name = url.host_str().unwrap().to_string();
    let board_key = url.path().split("/").nth(1).unwrap().to_string();
    let mut threads = Vec::new();
    for c in pattterns::parse_thread_list(&html) {
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
    Ok(threads)
}

#[derive(Debug)]
pub struct Board {
    pub url: String,
}

impl Board {
    pub fn new(url: String) -> Self {
        Self { url }
    }
    pub async fn load(&self) -> anyhow::Result<Threads> {
        let html = Reciever::get(&self.url).await?.html();

        anyhow::Ok(normalize_board(html.as_str(), self.url.clone()).await?)
    }
}
