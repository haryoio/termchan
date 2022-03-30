use std::cell::{Cell, RefCell};

use anyhow::Context;

use crate::{
    controller::reply::{Replies, Reply},
    utils::{pattterns, receiver::Reciever},
};

pub type Threads = Vec<Thread>;

#[derive(Debug, Clone)]
pub struct Thread {
    server_name:    String,
    board_key:      String,
    id:             String,
    title:          String,
    count:          Cell<usize>,
    replies:        RefCell<Replies>,
    is_first_fetch: Cell<bool>,
    is_stopdone:    Cell<bool>,
}

impl Thread {
    pub async fn new(server: &str, board: &str, id: &str, title: &str, count: usize) -> Thread {
        Thread {
            server_name:    server.to_string(),
            board_key:      board.to_string(),
            id:             id.to_string(),
            title:          title.to_string(),
            count:          Cell::new(count),
            replies:        RefCell::new(Vec::new()),
            is_first_fetch: Cell::new(true),
            is_stopdone:    Cell::new(false),
        }
    }

    pub fn get_server_name(&self) -> &str { &self.server_name }

    pub fn get_board_key(&self) -> &str { &self.board_key }

    pub fn get_id(&self) -> &str { &self.id }

    pub fn get_title(&self) -> &str { &self.title }

    // スレッドのURLを取得
    // https://<server_name>/test/read.cgi/<board_key>/<thread_id>/
    pub fn get_url(&self) -> String {
        format!(
            "https://{}/test/read.cgi/{}/{}/",
            self.server_name, self.board_key, self.id
        )
    }

    // 最新レスのURLを取得
    // https://<server_name>/test/read.cgi/<board_key>/<thread_id>/<latest_res>-n
    fn get_latest_url(&self) -> String {
        format!(
            "https://{}/test/read.cgi/{}/{}/{}-n/",
            self.server_name,
            self.board_key,
            self.id,
            self.count.get()
        )
    }

    pub fn is_stopdone(&self) -> bool { self.is_stopdone.get() }

    pub fn parse(&self, html: &str) -> Replies {
        if pattterns::is_stopdone(&html) {
            self.is_stopdone.set(true);
        }

        let mut replies = Vec::new();
        for cap in pattterns::parse_replies(html) {
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
        replies
    }

    pub async fn reload(&self) -> anyhow::Result<&Thread> {
        let url = if self.is_first_fetch.get() {
            self.get_url()
        } else {
            self.get_latest_url()
        };

        // dat落ちならリロードしない
        if self.is_stopdone.get() {
            return anyhow::Result::Ok(self);
        };

        let html = Reciever::new(&url).await.context("page error")?;
        let html = html.get_html();
        let mut replies = self.parse(html.as_str());
        let replies_count = replies.len();

        // 新着レスがあれば追加
        if self.is_first_fetch.get() {
            self.replies.borrow_mut().append(&mut replies);
            self.count.set(replies_count);
        } else if replies_count > 1 {
            self.replies.borrow_mut().append(&mut replies);
            self.count.set(self.count.get() + replies_count - 1);
        }

        anyhow::Result::Ok(self)
    }

    pub async fn get_replies(&self) -> anyhow::Result<Replies> {
        self.reload().await?;
        let replies = &*self.replies.borrow();
        anyhow::Result::Ok(replies.to_vec())
    }
}

impl Default for Thread {
    fn default() -> Self {
        Thread {
            server_name:    "".to_string(),
            board_key:      "".to_string(),
            id:             "".to_string(),
            title:          "".to_string(),
            count:          Cell::new(0),
            replies:        RefCell::new(Vec::new()),
            is_first_fetch: Cell::new(true),
            is_stopdone:    Cell::new(false),
        }
    }
}
