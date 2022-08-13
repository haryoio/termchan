use eyre::Result;
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde_json::Value;

pub struct Bbsmenu {
    url: String,
}
impl Bbsmenu {
    pub fn new(url: String) -> Result<Self> {
        Ok(Self { url })
    }

    pub async fn get(&self) -> Result<BbsmenuSchema> {
        let mut url = self.url.clone();
        let is_json = url.contains("\x35\x63\x68") || url.ends_with(".json");
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

#[derive(Debug, Clone)]
pub struct BbsmenuSchema {
    pub menu_list: Vec<CategoryItem>,
}
#[derive(Debug, Clone)]
pub struct CategoryItem {
    pub category_name:    String,
    pub category_content: Vec<CategoryContent>,
}
#[derive(Debug, Clone)]
pub struct CategoryContent {
    pub board_name: String,
    pub url:        String,
}

fn parse_bbsmenu_json(json_str: &str) -> BbsmenuSchema {
    let json_obj: Value = serde_json::from_str(json_str).unwrap();
    let menu_list_obj = json_obj["menu_list"].as_array().unwrap();
    let menu_list = menu_list_obj
        .par_iter()
        .map(|category_obj| {
            let category_name = category_obj["category_name"].as_str().unwrap();
            let category_content_obj = category_obj["category_content"].as_array().unwrap();
            let category_content: Vec<CategoryContent> = category_content_obj
                .par_iter()
                .map(|content_obj| {
                    let board_name = content_obj["board_name"].as_str().unwrap();
                    let url = content_obj["url"].as_str().unwrap();
                    CategoryContent {
                        board_name: board_name.to_string(),
                        url:        url.to_string(),
                    }
                })
                .collect::<Vec<CategoryContent>>();
            CategoryItem {
                category_name: category_name.to_string(),
                category_content,
            }
        })
        .collect::<Vec<_>>();
    BbsmenuSchema { menu_list }
}

fn parse_bbsmenu_html(html_str: &str) -> BbsmenuSchema {
    let mut menu_list: Vec<CategoryItem> = Vec::new();
    let mut category_name = String::new();
    let mut lines = html_str.split('\n');
    let mut category_content: Vec<CategoryContent> = Vec::new();

    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => {
                match lines.next() {
                    Some(line) => line,
                    None => break,
                }
            }
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
            }
        }
    }
    BbsmenuSchema { menu_list }
}

#[cfg(test)]
mod bbsmenu_tests {
    use super::*;

    #[tokio::test]
    async fn bbsmenu_test() {
        let bbsmenu = Bbsmenu::new("https://menu.open2ch.net/bbsmenu.html".to_string()).unwrap();
        let bbsmenu_schema = bbsmenu.get().await.unwrap();
        println!("{:?}", bbsmenu_schema);
    }

    #[tokio::test]
    async fn test_fivemenu() {
        let url = "https://menu.\x35\x63\x68.net/bbsmenu.html";
        let menues = Bbsmenu::new(url.to_string()).unwrap();
        let bbsmenu_schema = menues.get().await.unwrap();

        for category_item in bbsmenu_schema.menu_list {
            println!("{}", category_item.category_name);
            for category_content in category_item.category_content {
                println!("{}", category_content.board_name);
                println!("{}", category_content.url);
            }
        }
    }
}
