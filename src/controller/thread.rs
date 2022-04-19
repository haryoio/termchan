use std::{cell::Cell, fmt::Display};

use anyhow::Context;
use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::{Asia::Tokyo, Tz};

use crate::{controller::reply::Reply, patterns, receiver::Reciever};

fn normalize_thread(html: &str) -> anyhow::Result<Vec<Reply>> {
    let mut replies = Vec::new();
    let captures = if html.contains("vlink") {
        patterns::REPLIES_SC_RE.captures_iter(html)
    } else {
        patterns::REPLIES_RE.captures_iter(html)
    };
    for cap in captures {
        let group = (
            cap.name("reply_id").unwrap().as_str(),
            cap.name("name").unwrap().as_str(),
            cap.name("date").unwrap().as_str(),
            cap.name("id").unwrap().as_str(),
            cap.name("message").unwrap().as_str(),
        );
        match group {
            (reply_id, name, date, id, message) => {
                let reply = Reply::new(reply_id, name, date, id, message);
                replies.push(reply);
            }
        }
    }
    Ok(replies)
}

pub type Threads = Vec<Thread>;

#[derive(Debug, Clone)]
enum ReadMode {
    CGI,
    SO,
}

impl Display for ReadMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ReadMode::CGI => write!(f, "cgi"),
            ReadMode::SO => write!(f, "so"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Thread {
    server_name: String,
    board_key: String,
    id: String,
    title: String,
    count: Cell<usize>,
    list: Vec<Reply>,
    is_first_fetch: Cell<bool>,
    is_stopdone: Cell<bool>,
    read_mode: ReadMode,
}

impl Thread {
    pub fn new(server: &str, board: &str, id: &str, title: &str, count: usize) -> Thread {
        Thread {
            server_name: server.to_string(),
            board_key: board.to_string(),
            id: id.to_string(),
            title: title.to_string(),
            count: Cell::new(count),
            list: Vec::new(),
            is_first_fetch: Cell::new(true),
            is_stopdone: Cell::new(false),
            read_mode: ReadMode::CGI,
        }
    }

    pub fn server_name(&self) -> &str {
        &self.server_name
    }

    pub fn board_key(&self) -> &str {
        &self.board_key
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn count(&self) -> usize {
        self.count.get()
    }

    pub fn created_at(&self) -> DateTime<Tz> {
        let timestamp = self.id.parse::<i64>().unwrap();
        Tokyo.timestamp(timestamp, 0)
    }

    pub fn ikioi(&self) -> usize {
        let rep_count = self.count();
        let now: usize = Tokyo.timestamp(Utc::now().timestamp(), 0).timestamp() as usize;
        let first_rep: usize = self.created_at().timestamp() as usize;
        rep_count / ((now - first_rep) / 86400)
    }

    // スレッドのURLを取得
    // https://<server_name>/test/read.cgi/<board_key>/<thread_id>/
    pub fn url(&self) -> String {
        format!(
            "https://{}/test/read.{}/{}/{}/",
            self.server_name, self.read_mode, self.board_key, self.id
        )
    }

    // 最新レスのURLを取得
    // https://<server_name>/test/read.cgi/<board_key>/<thread_id>/<latest_res>-n
    fn latest_url(&self) -> String {
        format!(
            "https://{}/test/read.{}/{}/{}/{}-n/",
            self.server_name,
            self.read_mode,
            self.board_key,
            self.id,
            self.count.get()
        )
    }

    pub fn is_stopdone(&self) -> bool {
        self.is_stopdone.get()
    }

    pub async fn load(&mut self) -> anyhow::Result<Vec<Reply>> {
        let url = if self.is_first_fetch.get() {
            self.url()
        } else {
            self.latest_url()
        };

        // dat落ちならリロードしない
        if self.is_stopdone.get() {
            return anyhow::Result::Ok(self.list.clone());
        };

        let mut html = Reciever::get(&url)
            .await
            .context("failed fetch read.cgi")?
            .html();

        if patterns::ROADING_RE.is_match(html.as_str()) {
            self.read_mode = ReadMode::SO;
            html = Reciever::get(&url)
                .await
                .context("failed fetch read.so")?
                .html();
        }

        if patterns::STOPDONE_RE.is_match(&html) {
            self.is_stopdone.set(true);
        }

        let mut replies = normalize_thread(&html)?;
        let replies_count = replies.len();

        // 新着レスがあれば追加
        if self.is_first_fetch.get() {
            self.list.append(&mut replies);
            self.count.set(replies_count);
        } else if replies_count > 1 {
            self.list.append(&mut replies);
            self.count.set(self.count.get() + replies_count - 1);
        }

        let replies = self.list.clone();

        anyhow::Result::Ok(replies.to_vec())
    }
}

impl Default for Thread {
    fn default() -> Self {
        Thread {
            server_name: "".to_string(),
            board_key: "".to_string(),
            id: "".to_string(),
            title: "".to_string(),
            count: Cell::new(0),
            list: Vec::new(),
            is_first_fetch: Cell::new(true),
            is_stopdone: Cell::new(false),
            read_mode: ReadMode::CGI,
        }
    }
}
