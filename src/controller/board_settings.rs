use anyhow::Context;
use reqwest::Url;

use crate::{
    controller::thread::{Thread, Threads},
    patterns,
    receiver::Reciever,
};

fn normalize_board_settings(html: &str) -> [BoardSettingList; 9] {
    todo!()
}

#[derive(Debug)]
pub struct BoardSettings {
    pub url: String,
}

impl BoardSettings {
    pub fn new(url: &str) -> Self {
        let mut url = url.to_string();
        if !url.ends_with("/") {
            url.push_str("/");
        }
        url.push_str("bbs.cgi");
        Self { url }
    }
    pub async fn load(&self) -> anyhow::Result<[BoardSettingList; 9]> {
        let html = Reciever::get(&self.url).await?.html();

        anyhow::Ok(normalize_board_settings(html.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_load_5chvip_settings() {
        let url = "https://mi.5ch.net/news4vip/SETTING.TXT";
        let settings = BoardSettings::new(url).load().await.unwrap();
    }
}
