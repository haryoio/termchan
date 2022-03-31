use std::{fs::File, io, sync::Arc};

use anyhow::Context;
use cookie_store;
use reqwest_cookie_store::CookieStoreMutex;

use crate::configs::config::AppConfig;

pub struct CookieStore {
    arc: Arc<CookieStoreMutex>,
}

impl CookieStore {
    pub fn load() -> anyhow::Result<CookieStore> {
        let conf = AppConfig::new(None);
        let conf_dir = conf.conf_dir().context("failed to get conf dir")?;
        let cookie_path = format!("{}/cookie.json", conf_dir);
        let is_exist = std::path::Path::new(&cookie_path).exists();
        if !is_exist {
            File::create(&cookie_path).context("failed to create cookie file")?;
        };
        let file = File::open(&cookie_path)
            .map(io::BufReader::new)
            .context("failed to open cookie file")?;
        let cookie_store = cookie_store::CookieStore::load_json(file).unwrap();
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);

        Ok(CookieStore { arc: cookie_store })
    }

    pub fn save(arc: Arc<CookieStoreMutex>) -> anyhow::Result<()> {
        let conf = AppConfig::new(None);
        let conf_dir = conf.conf_dir().context("failed to get conf dir")?;
        let cookie_path = format!("{}/cookie.json", conf_dir);
        let mut file = File::create(&cookie_path)
            .map(io::BufWriter::new)
            .context("failed to create cookie file")?;
        let store = arc.lock().unwrap();
        store.save_json(&mut file).unwrap();
        Ok(())
    }

    pub fn clear() -> anyhow::Result<()> {
        let conf = AppConfig::new(None);
        let conf_dir = conf.conf_dir().context("failed to get conf dir")?;
        let cookie_path = format!("{}/cookie.json", conf_dir);
        let is_exist = std::path::Path::new(&cookie_path).exists();
        if is_exist {
            std::fs::remove_file(&cookie_path).context("failed to remove cookie file")?;
        };
        Ok(())
    }

    pub fn arc(&self) -> Arc<CookieStoreMutex> { Arc::clone(&self.arc) }
}
