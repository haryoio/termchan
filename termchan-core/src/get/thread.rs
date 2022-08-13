use std::{collections::HashMap, str::FromStr};

use eyre::{eyre, Result, WrapErr};
use rand::Rng;
use reqwest::Url;
use serde::{Deserialize, Serialize};

use super::message::Message;
use crate::{
    header::build::get_header,
    util::time::{decode_japan_date, unix_now_time},
};

#[derive(Debug, Clone)]
pub struct ThreadDetail {
    pub now:      i64,
    pub sub:      String,
    pub board:    String,
    pub dat:      i64,
    pub count:    usize,
    pub title:    String,
    pub url:      String,
    pub stopdone: bool,
}

#[derive(Debug, Clone)]
pub struct ThreadPost {
    pub post_id:            String,
    pub name:               String,
    pub email:              String,
    pub date:               i64,
    pub message:            Message,
    pub index:              usize,
    pub reply_count:        usize,
    pub post_count_all:     usize,
    pub post_count_current: usize,
}
#[derive(Debug, Clone)]
pub struct ThreadResponse {
    pub detail: ThreadDetail,
    pub posts:  Vec<ThreadPost>,
}

#[derive(Debug)]
pub struct Thread {
    pub url: String,
    pub dom: String,
}

impl Thread {
    pub fn new(url: String) -> Result<Self> {
        let url_split = url.split("/").collect::<Vec<_>>();
        let host = url_split[2];
        let board = url_split[5];
        let dat = url_split[6];

        let host_split = host.split(".").collect::<Vec<_>>();
        let sub = host_split[0];
        let dom = host_split[1..=2].join(".");

        let url = match dom.as_str() {
            "\x35\x63\x68.net" => get_five_json_url(&sub, &board, &dat),
            "open2ch.net" | "2ch.sc" => get_dat_url(&dom, &sub, &board, &dat),
            _ => Err(eyre!("unsupported board {}", host))?,
        };
        Ok(Self { url, dom })
    }

    pub async fn get(&self) -> Result<ThreadResponse> {
        let header = get_header(Url::from_str(&self.url).unwrap());
        let client = reqwest::Client::new();
        let res = client
            .get(&self.url)
            .headers(header)
            .send()
            .await
            .context(eyre!("Failed to get thread. got: {}", self.url.clone()))?;

        match self.dom.as_str() {
            "\x35\x63\x68.net" => parse_fivenet_json(res.json::<ThreadJson>().await?, &self.url),
            "open2ch.net" | "2ch.sc" => parse_dat(&res.text().await?, &self.url),
            _ => Err(eyre!("unsupported board {}", self.dom))?,
        }
    }
}

/// fivechでJSONを取得するためのURL
fn get_five_json_url(subdomain: &str, board: &str, dat: &str) -> String {
    format!(
        "https://itest.\x35\x63\x68.net/public/newapi/client.php?subdomain={}&board={}&dat={}&rand={}",
        subdomain,
        board,
        dat,
        get_rand()
    )
}

/// Open2chでDATを取得するためのURL
/// `https://{subdomain}.open2ch.net/{board}/dat/{dat}.dat`
fn get_dat_url(dom: &str, subdomain: &str, board: &str, dat: &str) -> String {
    format!("https://{}.{}/{}/dat/{}.dat", subdomain, dom, board, dat,)
}

fn parse_fivenet_json<'a>(json: ThreadJson, url: &'a str) -> Result<ThreadResponse> {
    let mut thread_posts = vec![];
    for reply in json.comments {
        let date = decode_japan_date(&reply.3).unwrap_or(0);
        thread_posts.push(ThreadPost {
            post_id: reply.4,
            name: reply.1,
            email: reply.2,
            date,
            message: Message::new(&reply.6),
            index: reply.0,
            reply_count: reply.7,
            post_count_all: reply.9,
            post_count_current: reply.8,
        });
    }

    let split = json.thread.3.split('/').collect::<Vec<_>>();
    let board = split[0].to_string();
    let dat = split[1].parse::<i64>().unwrap();
    Ok(ThreadResponse {
        detail: ThreadDetail {
            now: json.thread.0 as i64,
            count: json.thread.1,
            sub: json.thread.2,
            board,
            dat,
            title: json.thread.5,
            url: url.to_string(),
            stopdone: false,
        },
        posts:  thread_posts,
    })
}

/// 2ch.sc
/// Name<>email<>2000/01/01(日) 12:46:35  ID:AAAA<> Message <>
///
/// open2ch
/// Name<>email<>22/01/01(土) 00:00:00 ID:AAAA<> Message <>
pub fn parse_dat<'a>(dat: &'a str, url: &'a str) -> Result<ThreadResponse> {
    // datを正規化
    let mut posts: Vec<ThreadPost> = Vec::new();
    let mut title = String::new();

    let mut counter = IdCounter::new();

    for (i, line) in dat.lines().enumerate() {
        let line_split = line.split("<>").collect::<Vec<_>>();
        let name = line_split[0];
        let email = line_split[1];
        let date_id = line_split[2];
        let message = line_split[3];
        if i == 0 {
            title = line_split[4].to_string();
        }

        // 22/01/01(日) 00:00:00 ID:AAAA
        let date_split = date_id.split(" ").collect::<Vec<_>>();
        // ID:AAAA
        let id = &date_split.last().unwrap().to_string();
        // 22/01/01(日) 00:00:00
        let date = &date_split[..date_split.len() - 1].join(" ");
        let date = decode_japan_date(date).unwrap_or(0);

        counter.add(id);
        let current = counter.get(id);

        posts.push(ThreadPost {
            post_id: id.to_string(),
            name: name.to_string(),
            email: email.to_string(),
            date,
            message: Message::new(message),
            index: i,
            reply_count: 0,
            post_count_all: 0,
            post_count_current: current,
        });
    }

    // 同一IDでの書き込みを集計した値をそれぞれのpostに設定する
    for post in &mut posts {
        post.post_count_all = counter.get(&&post.post_id);
    }

    let url = Url::from_str(url).unwrap();
    let host = url.host_str().unwrap();
    let sub = host.split(".").collect::<Vec<_>>()[0].to_string();
    let thread_id = url.path_segments().unwrap().last().unwrap().to_string();
    let thread_id = thread_id.split('.').collect::<Vec<_>>()[0]
        .parse::<i64>()
        .unwrap_or(0);
    let board = url.path_segments().unwrap().next().unwrap().to_string();

    let now = unix_now_time();

    Ok(ThreadResponse {
        detail: ThreadDetail {
            now,
            title,
            url: url.to_string(),
            sub,
            board,
            dat: thread_id,
            count: posts.len(),
            stopdone: false,
        },
        posts,
    })
}

struct IdCounter {
    map: HashMap<String, usize>,
}
impl IdCounter {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn add(&mut self, id: &str) -> usize {
        let id = id.to_string();
        let count = self.map.entry(id).or_insert(0);
        *count += 1;
        *count
    }

    pub fn get(&self, id: &str) -> usize {
        *self.map.get(id).unwrap_or(&0)
    }
}

impl Default for ThreadDetail {
    fn default() -> Self {
        ThreadDetail {
            now:      0,
            sub:      "".to_string(),
            board:    "".to_string(),
            dat:      0,
            count:    0,
            title:    "".to_string(),
            url:      "".to_string(),
            stopdone: false,
        }
    }
}

impl Default for ThreadPost {
    fn default() -> Self {
        ThreadPost {
            post_id:            "".to_string(),
            name:               "".to_string(),
            email:              "".to_string(),
            date:               0,
            message:            Message::new(&"".to_string()),
            index:              0,
            reply_count:        0,
            post_count_all:     0,
            post_count_current: 0,
        }
    }
}

impl Default for ThreadResponse {
    fn default() -> Self {
        ThreadResponse {
            detail: ThreadDetail::default(),
            posts:  vec![ThreadPost::default()],
        }
    }
}

/// リクエスト時に使用するランダムな文字列を生成
fn get_rand() -> String {
    let cons = "ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    let mut s = String::new();
    for _ in 0..10 {
        s.push(cons.chars().nth(rng.gen_range(0..cons.len())).unwrap());
    }
    return s;
}

#[derive(Serialize, Deserialize, Debug)]

/// APIで取得したスレッド情報
/// ?? = 不明
/// ```
/// ThreadJson {
///     comments: Vec<(
///         index:  usize, 0
///         name:   String,1
///         email:  String,2
///         time:   String,3
///         id:     String,4
///         ??:     String,5
///         message:String,6
///         レスがついている数:usize,7
///         同じIDで書き込みした回数:usize,8
///         IDで書き込みした総回数:usize,9
///     )>,
///     thread: (
///         now:        uzise, 0,
///         count:      usize, 1,
///         sub:        String, 2,
///         board/dat:  String, 3,
///         ??: String, 4,
///         title: String, 5,
///     )
/// }
/// ```
struct ThreadJson {
    /// ```
    /// Vec<(
    ///         index:  usize,0
    ///         name:   String,1
    ///         email:  String,2
    ///         time:   String,3
    ///         id:     String,4
    ///         ??:     String,5
    ///         message:String,6
    ///         レスがついている数:usize,7
    ///         同じIDで書き込みした回数:usize,8
    ///         IDで書き込みした総回数:usize,9
    /// )>,
    /// ```
    comments: Vec<(
        usize,
        String,
        String,
        String,
        String,
        String,
        String,
        usize,
        usize,
        usize,
    )>,
    /// ```
    /// (
    ///     now:        uzise,  0,
    ///     count:      usize,  1,
    ///     sub:        String, 2,
    ///     board/dat:  String, 3,
    ///     ??:         String, 4,
    ///     title:      String, 5,
    /// )
    /// ```
    thread:      (usize, usize, String, String, String, String, String),
    total_count: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn get_json() {
        let url = "https://itest.\x35\x63\x68.net/public/newapi/client.php?subdomain=kizuna&board=pasta&dat=1569615752&rand=";
        let url = format!("{}{}", url, get_rand());
        let client = reqwest::Client::new();
        let res = client.get(&*url.clone()).send().await.unwrap();
        let json = res.json::<ThreadJson>().await.unwrap();
        println!("{:?}", json);
    }

    #[tokio::test]
    async fn get_fivechan() {
        let url = "https://mevius.\x35\x63\x68.net/test/read.cgi/kao/1632530358";
        let thread = Thread::new(url.to_string()).unwrap();
        // println!("{:?}", thread);
        let thread_response = thread.get().await.unwrap();
        for post in thread_response.posts {
            println!("{:?}", post);
        }
    }

    #[tokio::test]
    async fn get_open2ch() {
        let url = "https://ikura.open2ch.net/test/read.cgi/konamono/1652069715";
        let thread = Thread::new(url.to_string()).unwrap();
        println!("{:?}", thread);
        let thread_response = thread.get().await.unwrap();
        for post in thread_response.posts {
            println!("{}", post.message.json_string());
        }
    }

    #[tokio::test]
    async fn get_2chsc() {
        let url = "http://toro.2ch.sc/test/read.cgi/unix/1021212011";
        let thread = Thread::new(url.to_string()).unwrap();
        println!("{:?}", thread);
        let thread_response = thread.get().await.unwrap();
        for post in thread_response.posts {
            println!("{}", post.message.json_string());
        }
    }
}
