use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use derive_more::{Display, From};
use eyre::Result;
use serde::{Deserialize, Serialize};
use tui::{style::Color, widgets::BorderType as TuiBorderType};

use super::{dirs::Dir, theme::Theme};

#[cfg(target_os = "linux")]
static DEFAULT_CONFIG_PATH: &str = "/etc/termchan.toml";

#[cfg(target_os = "macos")]
static DEFAULT_CONFIG_PATH: &str = "/usr/local/etc/termchan.toml";

#[cfg(target_os = "windows")]
static DEFAULT_CONFIG_PATH: &str = "C:\\Windows\\Temp\\termchan.toml";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Config {
    pub bbsmenu_url_list: Vec<String>,

    /// サムネイルのサイズ(wip)
    /// small | medium | large
    /// default: small
    /// small: 16x10
    /// medium: 32x20
    /// large: 48x30
    pub thumbnail_size: ThumbnailSize,

    /// サムネイルのキャッシュサイズ(MB)
    /// 100MB以上推奨
    pub thumbnail_cache_size: String,

    pub login: bool,

    pub show_index: bool,

    pub theme:          Theme,
    pub request_header: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            bbsmenu_url_list:     vec![],
            thumbnail_size:       ThumbnailSize::Small,
            thumbnail_cache_size: "100M".to_string(),
            login:                false,
            show_index:           false,
            theme:                Theme::default(),
            request_header:
                r#"sec-ch-ua: ".Not/A)Brand";v="99", "Google Chrome";v="103", "Chromium";v="103"
sec-ch-ua-mobile: ?1
sec-ch-ua-platform: "Android"
sec-fetch-dest: document
sec-fetch-mode: navigate
sec-fetch-site: same-origin
sec-fetch-user: ?1
upgrade-insecure-requests: 1
user-agent: Mozilla/5.0 (Macintosh; Intel Mac OS X 10_10; rv:33.0) Gecko/20100101 Firefox/33.0
"#
                .to_string(),
        }
    }
}

impl Config {
    pub fn load_config() -> Result<Config> {
        let config_path = Dir::get_config_path()?;
        let config = Self::load_config_with_path(&config_path);
        let config = config.unwrap_or_else(|_| Config::default());
        Ok(config)
    }
    fn load_config_with_path<P: AsRef<Path>>(config_path: P) -> Result<Config> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(config_path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        Ok(config)
    }
    pub fn write(&mut self) -> Result<()> {
        let config_path = Dir::get_config_path()?;
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, self)?;
        Ok(())
    }
    pub fn pretty_json() -> Result<String> {
        let config = Self::load_config()?;
        let json = serde_json::to_string_pretty(&config)?;
        Ok(json)
    }
    pub fn path() -> Result<PathBuf> {
        let config_path = Dir::get_config_path()?;
        Ok(config_path)
    }
}
#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum ThumbnailSize {
    Small,
    Medium,
    Large,
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_config() {
        let mut config = Config::load_config().unwrap();
        println!("{}", Config::pretty_json().unwrap());
        config.login = true;
        config.write().unwrap();

        let mut config = Config::load_config().unwrap();
        println!("{}", Config::pretty_json().unwrap());
        println!("{:?}", Config::path().unwrap());
    }
}
