use std::path::{Path, PathBuf};

use directories::ProjectDirs;
use eyre::{bail, Result};

fn get_dirs() -> Option<ProjectDirs> {
    ProjectDirs::from("com", "haryoiro", "termchan")
}

static CONFIG_FILE_NAME: &str = "config.json";
static CACHE_DIR_NAME: &str = "cache";
static IMAGE_CACHE_DIR_NAME: &str = "image";
static CACHE_FILE_NAME: &str = "cache.json";
static DB_FILE_NAME: &str = "termchan.db";
static LOG_FILE_NAME: &str = "termchan.log";

#[cfg(target_os = "linux")]
const DATABASE_URL: &str = "sqlite:///var/tmp/termchan.db?mode=rwc";

#[cfg(target_os = "macos")]
const DATABASE_URL: &str = "sqlite:///var/tmp/termchan.db?mode=rwc";

#[cfg(target_os = "windows")]
const DATABASE_URL: &str = "sqlite:///C:\\Windows\\Temp\\termchan.db?mode=rwc";

pub struct Dir;
impl Dir {
    pub fn get_config_path() -> Result<PathBuf> {
        let dirs = get_dirs().unwrap();
        let config_path = dirs.config_dir().join(CONFIG_FILE_NAME);
        Ok(config_path)
    }
    pub fn get_image_cache_path() -> Result<PathBuf> {
        let dirs = get_dirs().unwrap();
        let image_cache_path = dirs
            .cache_dir()
            .join(CACHE_DIR_NAME)
            .join(IMAGE_CACHE_DIR_NAME);
        Ok(image_cache_path)
    }
    pub fn get_cache_path() -> Result<PathBuf> {
        let dirs = get_dirs().unwrap();
        let cache_path = dirs.cache_dir().join(CACHE_DIR_NAME).join(CACHE_FILE_NAME);
        Ok(cache_path)
    }
    pub fn get_db_path() -> Result<String> {
        Ok(DATABASE_URL.to_string())
    }
    pub fn get_log_path() -> Result<PathBuf> {
        let dirs = get_dirs().unwrap();
        let path = dirs.data_dir().join(LOG_FILE_NAME);
        Ok(path)
    }
}
