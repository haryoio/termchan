use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter},
};

use eyre::Result;

use super::dirs::Dir;
use crate::application::App;

pub struct CacheState;
impl CacheState {
    pub fn get() -> Option<App<'static>> {
        let cache_path = Dir::get_cache_path().unwrap();
        let file = OpenOptions::new().read(true).open(&cache_path);
        let file = match file {
            Ok(file) => file,
            Err(_) => return None,
        };
        let reader = BufReader::new(file);
        let app: Result<App, serde_json::Error> = serde_json::from_reader(reader);
        match app {
            Ok(app) => Some(app),
            Err(_) => None,
        }
    }
    pub fn set(app: App) -> Result<App> {
        let cache_path = Dir::get_cache_path().unwrap();
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&cache_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &app)?;
        Ok(app)
    }
}
