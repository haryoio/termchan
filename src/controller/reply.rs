use std::fmt::Display;

use scraper::Html;

pub type Replies = Vec<Reply>;

#[derive(Debug, Clone)]
pub struct Reply {
    pub reply_id: String,
    pub name: String,
    pub date: String,
    pub id: String,
    pub message: String,
}

impl Reply {
    pub fn new(reply_id: &str, name: &str, date: &str, id: &str, message: &str) -> Reply {
        Reply {
            reply_id: reply_id.to_string(),
            name: name.to_string(),
            date: date.to_string(),
            id: id.to_string(),
            message: message.to_string(),
        }
    }

    pub fn as_node(&self) -> anyhow::Result<Html> {
        let fragment = Html::parse_fragment(self.message.as_str());
        anyhow::Ok(fragment)
    }
}

impl Default for Reply {
    fn default() -> Self {
        Reply {
            reply_id: "".to_string(),
            name: "".to_string(),
            date: "".to_string(),
            id: "".to_string(),
            message: "".to_string(),
        }
    }
}

impl Display for Reply {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "-------------------\n{}\nname: {}\ndate: {}\nid: {}\nmessage: {}",
            self.reply_id, self.name, self.date, self.id, self.message
        )
    }
}
