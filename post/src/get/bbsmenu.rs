use regex::Regex;
use serde_json::{json, Value};

/*
menu.json
!のついているものは使用しない。
{
    !descriptoin: bbsmenuの詳細,
    !last_modify: 最終更新日時 unixtime,
    !last_modify_string: 最終更新日,
    menu_list: [
        {
            category_name: "地震", // カテゴリ名
            !category_number: "1", // カテゴリ番号
            !category_total: 5     // カテゴリ内の板数
            category_content: [   // カテゴリ内の板一覧
                {
                    board_name: String, // 板名
                    !category: usize,    // カテゴリ番号
                    !category_name: String,  // カテゴリ名
                    !category_order: usize,  // カテゴリ内での順番
                    !directory_name: String  // 板ディレクトリ名
                    url: String,       // 板URL
                }
            ]
        }
    ]
}
*/
#[derive(Debug, Clone)]
pub struct BbsmenuSchema {
    menu_list: Vec<CategoryItem>,
}
#[derive(Debug, Clone)]
pub struct CategoryItem {
    category_name:    String,
    category_content: Vec<CategoryContent>,
}
#[derive(Debug, Clone)]
pub struct CategoryContent {
    board_name: String,
    url:        String,
}

fn parse_bbsmenu_json(json_str: &str) -> BbsmenuSchema {
    let mut menu_list: Vec<CategoryItem> = Vec::new();
    let json_obj: Value = serde_json::from_str(json_str).unwrap();
    let menu_list_obj = json_obj["menu_list"].as_array().unwrap();
    for category_obj in menu_list_obj {
        let category_name = category_obj["category_name"].as_str().unwrap();
        let category_content_obj = category_obj["category_content"].as_array().unwrap();
        let mut category_content: Vec<CategoryContent> = Vec::new();
        for content_obj in category_content_obj {
            let board_name = content_obj["board_name"].as_str().unwrap();
            let url = content_obj["url"].as_str().unwrap();
            category_content.push(CategoryContent {
                board_name: board_name.to_string(),
                url:        url.to_string(),
            });
        }
        menu_list.push(CategoryItem {
            category_name: category_name.to_string(),
            category_content,
        });
    }
    BbsmenuSchema { menu_list }
}

fn parse_bbsmenu_html(html_str: &str) -> BbsmenuSchema {
    let mut menu_list: Vec<CategoryItem> = Vec::new();
    let mut category_name = String::new();
    let mut lines = html_str.split('\n');

    loop {
        let mut category_content: Vec<CategoryContent> = Vec::new();
        let line = match lines.next() {
            Some(line) => line,
            None => lines.next().unwrap(),
        };
        if line.contains("</small>") {
            break;
        }
        if line.starts_with("<BR><BR><B>") {
            category_name = line[11..line.len() - 8].to_string();
        }
        if line.starts_with("<A HREF=") {
            if line.ends_with("<br>") || line.ends_with("<BR>") {
                let b = &line[8..line.len() - 8];
                let content = if b.contains("TARGET=_blank") {
                    b.split(" TARGET=_blank>").collect::<Vec<&str>>()
                } else {
                    b.split(">").collect::<Vec<&str>>()
                };
                category_content.push(CategoryContent {
                    board_name: content[1].to_string(),
                    url:        content[0].to_string(),
                });
                continue;
            } else {
                let b = &line[8..line.len() - 4];
                let content = if b.contains("TARGET=_blank") {
                    b.split(" TARGET=_blank>").collect::<Vec<&str>>()
                } else {
                    b.split(">").collect::<Vec<&str>>()
                };
                category_content.push(CategoryContent {
                    board_name: content[1].to_string(),
                    url:        content[0].to_string(),
                });
                menu_list.push(CategoryItem {
                    category_name:    category_name.to_string(),
                    category_content: category_content.clone(),
                });
                category_content.clear();
                continue;
            }
        }
    }
    BbsmenuSchema { menu_list }
}

pub struct Bbsmenu {
    url: String,
}
impl Bbsmenu {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn load(&self) -> anyhow::Result<BbsmenuSchema> {
        let mut url = self.url.clone();
        let is_json = url.contains("5ch") || url.ends_with(".json");
        if is_json {
            url = url.replace(".html", ".json");
        }
        let html = reqwest::get(&url).await?.text().await?;
        if is_json {
            return Ok(parse_bbsmenu_json(&html));
        } else {
            return Ok(parse_bbsmenu_html(&html));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bbsmenu() {
        let url = "https://menu.5ch.net/bbsmenu.json".to_string();
        let bbsmenu = Bbsmenu::new(url).load().await;
        let url = "https://menu.5ch.net/bbsmenu.html".to_string();
        let bbsmenu = Bbsmenu::new(url).load().await;
        let url = "https://menu.2ch.sc/bbsmenu.html".to_string();
        let bbsmenu = Bbsmenu::new(url).load().await;
        println!("{:?}", bbsmenu);
    }
}
