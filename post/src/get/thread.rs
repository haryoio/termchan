use html_parser::Dom;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{header::build::get_header, url::reply::ThreadParams};

#[derive(Debug)]
pub struct ThreadPostList {
    title: String,
    url:   String,
    owner: String,
    posts: Vec<ThreadPost>,
}
#[derive(Debug)]
pub struct ThreadPost {
    id:      String,
    name:    String,
    date:    String,
    message: String,
}

pub struct Thread {
    url: String,
}

impl Thread {
    // scheme://<host>/test/read.cgi/<board>/<thread_id>/
    pub fn new(url: String) -> Self {
        let mut spurl = url.split("/");
        let scheme = spurl.next().unwrap().to_string();
        spurl.next();
        let host = spurl.next().unwrap().to_string();
        spurl.next();
        spurl.next();
        let board = spurl.next().unwrap().to_string();
        let thread_id = spurl.next().unwrap().to_string();
        let url = if host.contains("5ch") {
            // 5chの場合
            // https://<host>/test/read.cgi/<board>/<thread_id>/
            format!(
                "{}//{}/test/read.cgi/{}/{}/",
                scheme, host, board, thread_id
            )
        } else {
            // その他互換板の場合
            // https://<host>/<board>/dat/<thread_id>.dat
            format!("{}//{}/{}/dat/{}.dat", scheme, host, board, thread_id)
        };
        println!("{}", url);
        Self { url }
    }
    pub async fn load(&self) -> ThreadPostList {
        let header = get_header(ThreadParams::from(self.url.as_str()));
        let client = reqwest::Client::new();
        let res = client.get(&self.url).headers(header).send().await.unwrap();
        let html = res.text().await.unwrap();
        parse_thread_html(&html, &self.url.clone())
    }
}

const REPLIES_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r###"<div class="meta"><span class="number">(?P<reply_id>\d+)</span><span class="name"><b>(?P<name>.*?)</b></span><span class="date">(?P<date>.*?)</span><span class="uid">(?P<id>.*?)</span></div><div class="message"><span class="escaped">(?P<message>.*?)</span></div>"###).unwrap()
});
const REPLIES_SC_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r###"<dt>(?P<reply_id>\d+) ：<font.*><b>(?P<name>.*)<[/]b><[/]font>：(?P<date>.*) (?P<id>.*)<dd>(?P<message>.*)"###).unwrap()
});
const TITLE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r###"<title>(?P<title>.*?)\n</title>"###).unwrap());
fn parse_thread_title(html: &str) -> String {
    let title = TITLE_RE.captures(html).unwrap();
    title.name("title").unwrap().as_str().to_string()
}
fn parse_replies_sc(html: &str) -> Vec<ThreadPost> {
    let mut posts: Vec<ThreadPost> = Vec::new();
    for cap in REPLIES_SC_RE.captures_iter(html) {
        let id = cap.name("id").unwrap().as_str().to_string();
        let name = cap.name("name").unwrap().as_str().to_string();
        let date = cap.name("date").unwrap().as_str().to_string();
        let message = cap.name("message").unwrap().as_str().to_string();
        posts.push(ThreadPost {
            id,
            name,
            date,
            message,
        });
    }
    posts
}
fn parse_replies(html: &str) -> Vec<ThreadPost> {
    let mut posts: Vec<ThreadPost> = Vec::new();
    for cap in REPLIES_RE.captures_iter(html) {
        let id = cap.name("id").unwrap().as_str().to_string();
        let name = cap.name("name").unwrap().as_str().to_string();
        let date = cap.name("date").unwrap().as_str().to_string();
        let message = cap.name("message").unwrap().as_str().to_string();
        posts.push(ThreadPost {
            id,
            name,
            date,
            message,
        });
    }
    posts
}
fn parse_thread_html(html: &str, url: &str) -> ThreadPostList {
    let title = parse_thread_title(html);
    let thread_post_list = if html.contains("vlink") {
        parse_replies_sc(html)
    } else {
        parse_replies(html)
    };
    ThreadPostList {
        owner: thread_post_list[0].id.clone(),
        title,
        url: url.to_string(),
        posts: thread_post_list,
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_thread() {
        let url = "https://mi.5ch.net/test/read.cgi/news4vip/1657294031/l50";
        let thread = Thread::new(url.to_string());
        let html = thread.load().await;
        for post in html.posts {
            println!("{:?}", post);
        }
    }
}
