use std::collections::HashMap;

use anyhow::Context;
use reqwest::Url;

use crate::{controller::thread::Thread, patterns, receiver::Reciever};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
#[allow(non_camel_case_types)]
pub enum BoardSettingType {
    /// 板のフルネーム
    BBS_TITLE,
    /// 板の略称
    BBS_TITLE_ORIG,
    // デフォルトの名前
    BBS_NONAME_NAME,
    // スレッドタイトルの最大バイト数
    BBS_SUBJECT_COUNT,
    // 名前欄の最大バイト数
    BBS_NAME_COUNT,
    // メール欄の最大バイト数
    BBS_MAIL_COUNT,
    // 本文欄の最大バイト数
    BBS_MESSAGE_COUNT,
    // 連続スレ建て規制数
    BBS_THREAD_TATESUGI,
}

fn normalize_board_setting(
    html: &str,
) -> anyhow::Result<HashMap<BoardSettingType, Option<String>>> {
    let row = html.lines();
    let mut settings = HashMap::new();
    for (i, col) in row.enumerate() {
        if i == 0 {
            continue;
        }
        let key = col.split('=').next().unwrap();
        let value = col.split('=').skip(1).next().map(String::from);
        match key {
            "BBS_TITLE" => {
                settings.insert(BoardSettingType::BBS_TITLE, value);
            }
            "BBS_TITLE_ORIG" => {
                settings.insert(BoardSettingType::BBS_TITLE_ORIG, value);
            }
            "BBS_NONAME_NAME" => {
                settings.insert(BoardSettingType::BBS_NONAME_NAME, value);
            }
            "BBS_SUBJECT_COUNT" => {
                settings.insert(BoardSettingType::BBS_SUBJECT_COUNT, value);
            }
            "BBS_NAME_COUNT" => {
                settings.insert(BoardSettingType::BBS_NAME_COUNT, value);
            }
            "BBS_MAIL_COUNT" => {
                settings.insert(BoardSettingType::BBS_MAIL_COUNT, value);
            }
            "BBS_MESSAGE_COUNT" => {
                settings.insert(BoardSettingType::BBS_MESSAGE_COUNT, value);
            }
            "BBS_THREAD_TATESUGI" => {
                settings.insert(BoardSettingType::BBS_THREAD_TATESUGI, value);
            }
            _ => {}
        }
    }
    Ok(settings)
}

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
        if url.ends_with("subback.html") {
            let pos = url.rfind("/").unwrap();
            url.truncate(pos);
        }
        Board { url }
    }

    pub async fn load(&self) -> anyhow::Result<Vec<Thread>> {
        let url = format!("{}{}", self.url, "subback.html");
        let html = Reciever::get(&url).await?.html();

        anyhow::Ok(normalize_board(html.as_str(), self.url.clone())?)
    }

    pub async fn load_settings(&self) -> anyhow::Result<HashMap<BoardSettingType, Option<String>>> {
        let url = format!("{}{}", self.url, "/SETTING.TXT");
        println!("{}", url);
        let html = Reciever::get(&url).await?.html();

        anyhow::Ok(normalize_board_setting(html.as_str())?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_substr_to_subback_url() {
        let url = "https://mi.5ch.net/termchan/subback.html";
        let board = Board::new(url.to_string());

        assert_eq!(board.url, "https://termchan.net/termchan");
    }

    #[tokio::test]
    async fn test_board_load() {
        let url = "https://mi.5ch.net/news4vip";
        let board = Board::new(url.to_string());
        println!("{:?}", board.load().await.unwrap());

        let threads = board.load().await.unwrap();
        println!("{:#?}", threads);
    }

    #[tokio::test]
    async fn test_setting_load() {
        let url = "https://mi.5ch.net/news4vip";
        let board = Board::new(url.to_string());
        let resp = board.load_settings().await.unwrap();
        println!("{:?}", resp);
    }
}
