use std::{fs, io::prelude::*, path::Path};

use anyhow::{Context, Ok};
use configparser::ini::Ini;
use dirs;

const APP_NAME: &str = "termch";
const CONFIG_FILE: &str = "config";

#[derive(Debug)]
pub struct AppConfig {
    pub login:    Option<bool>,
    pub url:      String,
    pub email:    String,
    pub password: String,
    config_name:  String,
}

//
// let config = Config::new("config")?
impl AppConfig {
    pub fn new(name: Option<&str>) -> Self {
        let config_name = match name {
            Some(name) => name.to_string(),
            None => CONFIG_FILE.to_string(),
        };
        Self {
            login: None,
            url: "".to_string(),
            email: "".to_string(),
            password: "".to_string(),
            config_name,
        }
    }

    pub fn load(&self) -> anyhow::Result<AppConfig> {
        let mut cfg = Ini::new();
        let path = self
            .config_file_path()
            .context("failed to get config file path")?;

        if !self.is_exist() {
            self.initialize_config_file()?;
        }
        let cfg_str = fs::read_to_string(path)?;
        cfg.read(cfg_str).unwrap();
        let login = cfg.getbool("login", "login").unwrap_or(Some(false));
        let url = cfg.get("login", "url").unwrap_or(String::new());
        let email = cfg.get("login", "user").unwrap_or(String::new());
        let password = cfg.get("login", "password").unwrap_or(String::new());
        Ok(AppConfig {
            login,
            url,
            email,
            password,
            config_name: self.config_name.clone(),
        })
    }

    pub fn config_file_path(&self) -> anyhow::Result<String> {
        let path = dirs::config_dir().unwrap();
        let path = path.join(APP_NAME);
        let path = path.join(self.config_name.clone());
        Ok(path.to_str().unwrap().to_string())
    }

    pub fn initialize_config_file(&self) -> anyhow::Result<()> {
        let path = self.config_file_path().context("home dir error")?;

        let default =
            fs::read_to_string("./src/config/default.ini").context("failed to read default.ini")?;
        fs::File::create(path)
            .context("create config file error")?
            .write(default.as_bytes())
            .context("write config file error")?;

        Ok(())
    }

    pub fn is_exist(&self) -> bool {
        let path = self.config_file_path().unwrap();
        Path::new(&path).exists()
    }

    pub fn conf_dir(&self) -> anyhow::Result<String> {
        let path = dirs::config_dir().unwrap();
        let path = path.join(APP_NAME);
        Ok(path.to_str().unwrap().to_string())
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    const CONFIG_FILE: Option<&str> = Some("config.test");

    #[tokio::test]
    async fn test() {
        clean().await;
        config_file_exists().await;
        initialize_config_file().await;
        load_default_config_file().await;
        test_config_file_path().await;
        clean().await;
    }

    async fn clean() {
        let conf = AppConfig::new(CONFIG_FILE);
        let path = conf.config_file_path().unwrap();
        let _ = fs::remove_file(path).unwrap_or(());
    }

    async fn config_file_exists() {
        let conf = AppConfig::new(CONFIG_FILE);
        let is_exists = conf.is_exist();
        assert_eq!(is_exists, false);
    }

    async fn initialize_config_file() {
        let conf = AppConfig::new(CONFIG_FILE);
        conf.initialize_config_file().unwrap();
        let is_exist = conf.is_exist();
        assert_eq!(is_exist, true);
    }

    async fn load_default_config_file() {
        let conf = AppConfig::new(CONFIG_FILE);
        let config = conf.load().unwrap();
        assert_eq!(config.login, Some(false));
        assert_eq!(config.url, "".to_string());
        assert_eq!(config.email, "".to_string());
        assert_eq!(config.password, "".to_string());
    }

    async fn test_config_file_path() {
        let conf = AppConfig::new(CONFIG_FILE);
        let path = conf.config_file_path().unwrap();

        let home = dirs::home_dir().unwrap();
        let home = home.as_os_str().to_str().unwrap();
        assert_eq!(path, format!("{}/.config/termch/config.test", home));
    }
}
