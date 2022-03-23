use crate::util;
use regex::Regex;
use std::cell::{Cell, RefCell};
use url;

type Threads = Vec<Thread>;

#[derive(Debug)]
pub struct ThreadList {
    url: String,
    board_key: String,
    server_name: String,
    pub threads: RefCell<Threads>,
    pub current_thread: RefCell<Thread>,
}

impl ThreadList {
    pub fn new(url: &str) -> ThreadList {
        //  https://<server_name>/<board_key>/subback.html
        let url = url.to_owned();
        let url_element = url::Url::parse(&url).unwrap();

        // サーバ名を取得
        // vipなら mi.5ch.net
        let server_name = url_element.host_str().unwrap().to_string();

        // 板名を取得
        // vipなら news4vip
        let board_key = url_element.path().split("/").nth(1).unwrap().to_string();

        ThreadList {
            url: url.to_string(),
            board_key,
            server_name,
            threads: RefCell::new(Vec::new()),
            current_thread: RefCell::new(Thread::default()),
        }
    }

    // スレッド一覧ページのHTMLより構造化されたスレッド一覧を取得する
    fn parse(&self, html: &str) -> Threads {
        let re =
            Regex::new(r#"<a href="(.+?)/l50">([0-9]+?:)(.+?) (\()([0-9]+?)(\))</a>"#).unwrap();
        let mut threads = Vec::new();
        for cap in re.captures_iter(&html) {
            let id = cap[1].to_string();
            let index = cap[2].to_string();
            let title = cap[3].to_string();
            let count = cap[5].to_string();
            threads.push(Thread::new(
                &self.server_name,
                &self.board_key,
                &id,
                &index,
                &title,
                &count,
            ));
        }
        threads
    }

    pub async fn fetch(&self) -> &Self {
        let body = reqwest::get(&self.url).await;
        if let Ok(body) = body {
            let html = body.bytes().await.unwrap();
            let parsed = self.parse(&util::sjis_to_utf8(&html));
            self.threads.borrow_mut().clear();
            self.threads.borrow_mut().extend(parsed);
        }
        self
    }

    pub fn get_thread_by_id(&self, id: &str) -> &Self {
        let thread = &self
            .threads
            .borrow()
            .iter()
            .find(|t| t.id == id)
            .map(|t| t.clone());
        if let Some(thread) = thread {
            self.current_thread.replace(thread.clone());
        };
        self
    }
}

type ReplyList = Vec<Reply>;

#[derive(Debug, Clone)]
pub struct Thread {
    pub server_name: String,
    pub board_key: String,
    pub id: String,
    pub index: String,
    pub title: String,
    pub count: String,
    pub latest_response: Cell<i32>,
    pub response_list: RefCell<ReplyList>,
    pub is_first: Cell<bool>,
}

impl Thread {
    pub fn new(
        server: &str,
        board: &str,
        id: &str,
        index: &str,
        title: &str,
        count: &str,
    ) -> Thread {
        Thread {
            server_name: server.to_string(),
            board_key: board.to_string(),
            id: id.to_string(),
            index: index.to_string(),
            title: title.to_string(),
            count: count.to_string(),
            latest_response: Cell::new(1),
            response_list: RefCell::new(Vec::new()),
            is_first: Cell::new(true),
        }
    }

    // スレッドのURLを取得
    // https://<server_name>/test/read.cgi/<board_key>/<thread_id>/
    pub fn all_url(&self) -> String {
        format!(
            "https://{}/test/read.cgi/{}/{}/",
            self.server_name, self.board_key, self.id
        )
    }

    // 最新レスのURLを取得
    // https://<server_name>/test/read.cgi/<board_key>/<thread_id>/<latest_res>-n
    pub fn newest_url(&self) -> String {
        format!(
            "https://{}/test/read.cgi/{}/{}/{}-n/",
            self.server_name,
            self.board_key,
            self.id,
            self.latest_response.get()
        )
    }

    pub async fn fetch(&self) -> &Self {
        let url = if self.is_first.get() {
            self.all_url()
        } else {
            self.newest_url()
        };
        // TODO: 保存したcookieをファイルから読み込んで使う
        let cookie = "READJS=\"off\";SUBBACK_STYLE=\"1\"";
        let client = reqwest::Client::builder().build().unwrap();
        let body = client.get(&url)
            .header("Accept", "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8")
            .header("Accept-Encoding", "gzip, deflate, br")
            .header("Accept-Language", "ja,en-US;q=0.7,en;q=0.3")
            .header("Cache-Control", "max-age=0")
            .header("Connection", "keep-alive")
            .header("Cookie", cookie)
            .header("Host", &self.server_name)
            .header("Sec-Fetch-Dest", "document")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "none")
            .header("Sec-Fetch-User", "?1")
            .header("Upgrade-Insecure-Requests", "1")
            .header("User-Agent", "Mozilla/5.0 (X11; Linux x86_64; rv:98.0) Gecko/20100101 Firefox/98.0")
            .send().await;

        if let Ok(body) = body {
            let bytes = body.bytes().await.unwrap();
            let html = util::sjis_to_utf8(&bytes);
            println!("html {:?}", html);
        }

        self
    }

    fn parse<U>(&self, html: &str) -> U {
        todo!()
    }
}

impl Default for Thread {
    fn default() -> Self {
        Thread {
            server_name: "".to_string(),
            board_key: "".to_string(),
            id: "".to_string(),
            index: "".to_string(),
            title: "".to_string(),
            count: "".to_string(),
            latest_response: Cell::new(1),
            response_list: RefCell::new(Vec::new()),
            is_first: Cell::new(true),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Reply {
    pub data_id: String,
    pub res_id: String,
    pub user_id: String,
}
