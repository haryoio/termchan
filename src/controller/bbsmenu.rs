use anyhow::Context;
use regex::Regex;

use crate::encoder::{is_utf8, sjis_to_utf8};

fn split_categories(html: &mut String) -> anyhow::Result<Vec<BbsCategories>> {
    let targetr = Regex::new(r#" TARGET=(.*?)>"#).context("failed to create regex")?;
    let html = targetr.replace_all(&html, ">".to_string());

    // println!("{}", html);
    let mut splitted: Vec<&str> = html.split("\n").collect::<Vec<&str>>();
    splitted.reverse();

    let mut lines = Vec::new();
    for l in splitted {
        if l.starts_with("<A HREF=") {
            if l.ends_with("</A><br>") {
                lines.push(&l[8..l.len() - 8]);
            } else {
                lines.push(&l[8..l.len() - 4]);
            };
        } else if l.starts_with("<BR><BR><B>") {
            lines.push(&l[11..l.len() - 8]);
        }
    }

    let mut categories: Vec<BbsCategories> = Vec::new();
    let mut links: Vec<BbsUrl> = Vec::new();
    for l in lines {
        let s = l.split("/>").collect::<Vec<&str>>();
        match s.len() {
            1 => {
                let category = s[0].to_string();
                if category.contains("<") {
                    let s = category.split("<").collect::<Vec<&str>>();
                    let mut title = s[0].to_string();
                    let url = s[1].to_string();
                    links.push(BbsUrl { title, url });
                    continue;
                }
                categories.push(BbsCategories {
                    category,
                    list: links,
                });
                links = Vec::new();
            }
            2 => {
                let mut title = s[0].to_string();
                let (en, ..) = encoding_rs::SHIFT_JIS.decode(title.as_bytes());
                let title = en.to_string();
                let url = s[1].to_string();
                links.push(BbsUrl { title, url });
            }
            _ => {
                return Err(anyhow::anyhow!("failed to parse categories"));
            }
        }
    }

    for c in &categories {
        println!("{}", c.category);
        for l in &c.list {
            println!("\t{} : {}", l.title, l.url);
        }
    }

    Ok(categories)
}

#[derive(Debug)]
pub struct Bbsmenu {
    pub url: String,
}

impl Bbsmenu {
    pub fn new(url: String) -> Self {
        Self { url }
    }

    pub async fn load(&self) -> anyhow::Result<Vec<BbsUrl>> {
        let url = self.url.clone();
        let html = Reciever::get(&url).await.context("page error")?.html();
        let mut text = resp.text().await?;
        let split = split_categories(&mut text);
        Ok(Vec::new())
    }
}

pub struct BbsCategories {
    pub category: String,
    pub list: Vec<BbsUrl>,
}

#[derive(Debug)]
pub struct BbsUrl {
    url: String,
    title: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_bbsmenu() {
        let url = "https://menu.5ch.net/bbsmenu.html";
        let bbsmenu = Bbsmenu::new(url.to_string());
        let result = bbsmenu.load().await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
