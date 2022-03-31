use anyhow::Context;
use regex::Regex;
use std::borrow::Cow;
use std::collections::HashMap;

type Categories = HashMap<String, HashMap<String, String>>;
fn split_categories(html: &mut String) -> anyhow::Result<Categories> {
    let targetr = Regex::new(r#" TARGET=(.*?)>"#).unwrap();
    let html = targetr.replace_all(html, |caps: &regex::Captures| ">".to_string());
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
    let mut categories: HashMap<String, HashMap<String, String>> = HashMap::new();
    let mut links: HashMap<String, String> = HashMap::new();
    for l in lines {
        let s = l.split("/>").collect::<Vec<&str>>();
        match s.len() {
            1 => {
                let category = s[0].to_string();
                categories.insert(category, links);
                links = HashMap::new();
            }
            2 => {
                let url = s[0].to_string();
                let link = s[1].to_string();
                links.insert(link, url);
            }
            _ => {
                return Err(anyhow::anyhow!("failed to parse categories"));
            }
        }
    }

    for (x, y) in categories.iter_mut() {
        println!("{}", x);
        for (z, w) in y.iter_mut() {
            println!("\t{}: {}", z, w);
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
        let client = reqwest::Client::new();
        let resp = client.get(url).send().await?;
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
    description: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_bbsmenu() {
        let url = "https://menu.2ch.sc/bbsmenu.html";
        let bbsmenu = Bbsmenu::new(url.to_string());
        let result = bbsmenu.load().await;
        println!("{:?}", result);
        assert!(result.is_ok());
    }
}
