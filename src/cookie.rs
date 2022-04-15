use std::{fs::File, io, sync::Arc};

use anyhow::Context;
use cookie_store;
use reqwest_cookie_store::CookieStoreMutex;

use crate::configs::config::Config;

pub struct CookieStore {
    arc: Arc<CookieStoreMutex>,
}

impl CookieStore {
    fn path() -> anyhow::Result<String> {
        let conf = Config::load().context("failed to load config")?;
        let path = conf
            .cookie
            .as_ref()
            .context("cookie is not set")?
            .path
            .clone();
        Ok(path)
    }
    pub fn load() -> anyhow::Result<CookieStore> {
        let path = CookieStore::path()?;
        let is_exist = std::path::Path::new(&path).exists();
        if !is_exist {
            File::create(&path).context("failed to create cookie file")?;
        };
        let file = File::open(&path)
            .map(io::BufReader::new)
            .context("failed to open cookie file")?;
        let cookie_store = cookie_store::CookieStore::load_json(file).unwrap();
        let cookie_store = CookieStoreMutex::new(cookie_store);
        let cookie_store = std::sync::Arc::new(cookie_store);

        Ok(CookieStore { arc: cookie_store })
    }

    pub fn save(arc: Arc<CookieStoreMutex>) -> anyhow::Result<()> {
        let cookie_path = CookieStore::path()?;
        let mut file = File::create(&cookie_path)
            .map(io::BufWriter::new)
            .context("failed to create cookie file")?;
        let store = arc.lock().unwrap();
        store.save_json(&mut file).unwrap();
        Ok(())
    }

    pub fn clear() -> anyhow::Result<()> {
        let cookie_path = CookieStore::path()?;
        let is_exist = std::path::Path::new(&cookie_path).exists();
        if is_exist {
            std::fs::remove_file(&cookie_path).context("failed to remove cookie file")?;
        };
        Ok(())
    }

    pub fn arc(&self) -> Arc<CookieStoreMutex> {
        Arc::clone(&self.arc)
    }
}
