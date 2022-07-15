use eyre::{eyre, ContextCompat, Result, WrapErr};
use html_parser::Dom;
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{error::FormatError, header::build::get_header, url::reply::ThreadParams};

#[derive(Debug, Clone)]
pub struct ThreadDetail {
    pub title: String,
    pub url:   String,
    pub owner: String,
}

#[derive(Debug, Clone)]
pub struct ThreadPost {
    pub id:      String,
    pub name:    String,
    pub date:    String,
    pub message: String,
}
#[derive(Debug, Clone)]
pub struct ThreadResponse {
    pub detail: ThreadDetail,
    pub posts:  Vec<ThreadPost>,
}

pub struct Thread {
    pub url: String,
}

impl Thread {
    pub fn new(url: String) -> Result<Self> {
        let mut spurl = url.split("/");
        let scheme = spurl.next().context(eyre!(" {}", url.clone()))?.to_string();
        spurl.next();
        let host = spurl.next().context(eyre!("{}", url.clone()))?.to_string();
        spurl.next();
        spurl.next();
        let board = spurl.next().context(eyre!("{}", url.clone()))?.to_string();
        let thread_id = spurl.next().context(eyre!("{}", url.clone()))?.to_string();
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
        Ok(Self { url })
    }
    pub async fn get(&self) -> Result<ThreadResponse> {
        let header = get_header(ThreadParams::from(self.url.as_str()));
        let client = reqwest::Client::new();
        let res = client
            .get(&self.url)
            .headers(header)
            .send()
            .await
            .context(eyre!("Failed to get thread. got: {}", self.url.clone()))?;
        let html = res
            .text()
            .await
            .context(eyre!("Failed to get a thread. got: {}", self.url.clone()))?;
        parse_thread_html(&html, &self.url.clone())
    }
}

const REPLIES_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r###"<div class="meta"><span class="number">(?P<reply_id>\d+)</span><span class="name"><b>(?P<name>.*?)</b></span><span class="date">(?P<date>.*?)</span><span class="uid">(?P<id>.*?)</span></div><div class="message">(?P<message>.*?)</div>"###).unwrap()
});
const REPLIES_SC_RE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r###"<dt>(?P<reply_id>\d+) ：<font.*><b>(?P<name>.*)<[/]b><[/]font>：(?P<date>.*) (?P<id>.*)<dd>(?P<message>.*)"###).unwrap()
});
const TITLE_RE: Lazy<Regex> =
    Lazy::new(|| Regex::new(r###"<title>(?P<title>.*?)\n</title>"###).unwrap());
fn parse_thread_title(html: &str) -> Result<String> {
    let title = TITLE_RE
        .captures(html)
        .context(eyre!("Failed to parse a thread title"))?;
    Ok(title.name("title").unwrap().as_str().to_string())
}
fn parse_replies_sc(html: &str) -> Result<Vec<ThreadPost>> {
    let mut posts: Vec<ThreadPost> = Vec::new();
    for cap in REPLIES_SC_RE.captures_iter(html) {
        let id = cap.name("id").unwrap();
        let name = cap.name("name").unwrap();
        let date = cap.name("date").unwrap();
        let message = cap.name("message").unwrap();
        posts.push(ThreadPost {
            id:      id.as_str().to_string(),
            name:    name.as_str().to_string(),
            date:    date.as_str().to_string(),
            message: message.as_str().to_string(),
        });
    }
    Ok(posts)
}
fn parse_replies(html: &str) -> Result<Vec<ThreadPost>> {
    let mut posts: Vec<ThreadPost> = Vec::new();
    for cap in REPLIES_RE.captures_iter(html) {
        let id = cap.name("id").unwrap();
        let name = cap.name("name").unwrap();
        let date = cap.name("date").unwrap();
        let message = cap.name("message").unwrap();
        posts.push(ThreadPost {
            id:      id.as_str().to_string(),
            name:    name.as_str().to_string(),
            date:    date.as_str().to_string(),
            message: message.as_str().to_string(),
        });
    }
    Ok(posts)
}
fn parse_thread_html(html: &str, url: &str) -> Result<ThreadResponse> {
    let title = parse_thread_title(html)?;
    let thread_post_list = if html.contains("vlink") {
        parse_replies_sc(html)
            .context(eyre!("Failed to parse thread posts. url: {}", url.clone()))?
    } else {
        parse_replies(html).context(eyre!("Failed to parse thread posts. url: {}", url.clone()))?
    };

    Ok(ThreadResponse {
        detail: ThreadDetail {
            title,
            url: url.to_string(),
            owner: thread_post_list[0].id.clone(),
        },
        posts:  thread_post_list,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::get::message::Message;

    #[tokio::test]
    async fn test_thread() {
        let url = "https://mi.5ch.net/test/read.cgi/news4vip/1657462844/l50";
        let thread = Thread::new(url.to_string()).unwrap();
        let thread_response = thread.get().await.unwrap();
        for post in thread_response.posts {
            // println!("{:?}", Message::new(&post.message));
            println!("{}", Message::new(&post.message));
        }
    }
}
