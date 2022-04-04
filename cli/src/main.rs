use anyhow::{Context, Ok, Result};
use termchan::{configs::config::AppConfig, controller::menu::BbsCategories};

pub struct App<'a> {
    pub title: &'a str,
    pub bbs_url: &'a str,
    pub bbs_categories: Vec<BbsCategories>,
}

impl<'a> App<'a> {
    pub fn new() -> Result<App<'a>> {
        let conf = AppConfig::new(None)
            .load()
            .context("failed to load config")?;
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    let app = App::new();
    Ok(())
}
