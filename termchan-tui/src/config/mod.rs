pub mod credentials;
pub mod dirs;
use std::{
    fs::OpenOptions,
    io::{BufReader, BufWriter},
    path::{Path, PathBuf},
};

use eyre::Result;
use serde::{Deserialize, Serialize};
use tui::{style::Color, widgets::BorderType as TuiBorderType};

use self::dirs::Dir;

#[cfg(target_os = "linux")]
static DEFAULT_CONFIG_PATH: &str = "/etc/termchan.toml";

#[cfg(target_os = "macos")]
static DEFAULT_CONFIG_PATH: &str = "/usr/local/etc/termchan.toml";

#[cfg(target_os = "windows")]
static DEFAULT_CONFIG_PATH: &str = "C:\\Windows\\Temp\\termchan.toml";

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    ///
    pub config_path: Option<String>,

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

    /// 画像や書き込みのキャッシュを保存するディレクトリを指定。
    /// Windows
    ///  C:\Users\username\AppData\Local\Temp\termchan\cache
    /// MacOS / Linux
    ///  /var/tmp/termchan/cache
    pub cache_dir: String,

    pub theme: Theme,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            config_path:          None,
            bbsmenu_url_list:     vec![],
            thumbnail_size:       ThumbnailSize::Small,
            thumbnail_cache_size: "100M".to_string(),
            cache_dir:            "/var/tmp/termchan/cache".to_string(),
            theme:                Theme::default(),
        }
    }
}

impl Config {
    pub fn get_config() -> Result<Config> {
        let config_path = Dir::get_config_path()?;
        let config = Self::load_config(&config_path);
        let config = config.unwrap_or_else(|_| Config::default());
        Ok(config)
    }
    pub fn load_config<P: AsRef<Path>>(config_path: P) -> Result<Config> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .open(config_path)?;
        let reader = BufReader::new(file);
        let config: Config = serde_json::from_reader(reader)?;
        Ok(config)
    }
    pub fn update_config(config: Config) -> Result<Config> {
        let config_path = Dir::get_config_path()?;
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(config_path)?;
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &config)?;
        Ok(config)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub enum BorderType {
    Plain,
    Rounded,
    Double,
    Thick,
}

#[derive(Serialize, Deserialize, Clone)]
pub enum ThumbnailSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Theme {
    pub status_bar:   Color,
    pub error_border: Color,
    pub error_text:   Color,
    pub hint:         Color,
    pub hovered:      Color,
    pub active:       Color,
    pub inactive:     Color,
    pub selected:     Color,
    pub text:         Color,

    // text
    pub active_unselected_text:   Color,
    pub active_selected_text:     Color,
    pub inactive_unselected_text: Color,
    pub inactive_selected_text:   Color,

    pub reset: Color,

    pub active_item_symbol:   String,
    pub inactive_item_symbol: String,
    pub unread_symbol:        String,
    pub read_symbol:          String,
    pub posted_symbol:        String,

    pub ikioi_low:         Color,
    pub ikioi_middle:      Color,
    pub ikioi_middle_high: Color,
    pub ikioi_high:        Color,

    /// border_type: Plain | Rounded | Double | Thick
    /// default: Plain
    /// 参照: https://docs.rs/tui-style/0.1.0/tui_style/enum.BorderStyle.html
    border_type: BorderType,
}

impl Default for Theme {
    fn default() -> Self {
        Theme {
            status_bar:   Color::LightCyan,
            error_border: Color::Red,
            error_text:   Color::White,
            hint:         Color::LightCyan,
            hovered:      Color::LightCyan,
            active:       Color::LightCyan,
            inactive:     Color::Gray,
            selected:     Color::LightCyan,
            text:         Color::White,

            active_unselected_text:   Color::White,
            active_selected_text:     Color::LightCyan,
            inactive_unselected_text: Color::Gray,
            inactive_selected_text:   Color::Cyan,

            reset: Color::Reset,

            ikioi_low:         Color::LightCyan,
            ikioi_middle:      Color::LightCyan,
            ikioi_middle_high: Color::LightCyan,
            ikioi_high:        Color::LightCyan,

            active_item_symbol:   ">".to_string(),
            inactive_item_symbol: " ".to_string(),
            unread_symbol:        "●".to_string(),
            read_symbol:          "○".to_string(),
            posted_symbol:        "✎".to_string(),

            border_type: BorderType::Plain,
        }
    }
}

impl Theme {
    pub fn border_type(&self) -> TuiBorderType {
        match self.border_type {
            BorderType::Plain => TuiBorderType::Plain,
            BorderType::Rounded => TuiBorderType::Rounded,
            BorderType::Double => TuiBorderType::Double,
            BorderType::Thick => TuiBorderType::Thick,
        }
    }
}

impl Config {
    pub fn load(&self) {
    }
}
