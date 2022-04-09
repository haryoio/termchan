use std::{
    fs,
    io::{Read, Write},
};

use anyhow::Context;
use serde::{Deserialize, Serialize};

use crate::configs::{
    bbsmenu::BBSMenuConfig, board::BoardConfig, cookie::CookieConfig, login::LoginConfig,
    post::PostConfig, proxy::ProxyConfig,
};

const APP_NAME: &str = "termchan";

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub bbsmenu: BBSMenuConfig,
    pub board: Option<BoardConfig>,
    pub login: Option<LoginConfig>,
    pub post: Option<PostConfig>,
    pub proxy: Option<ProxyConfig>,
    pub cookie: Option<CookieConfig>,
}

impl Config {
    pub fn load() -> anyhow::Result<Config> {
        let confdir = dirs::config_dir().context("failed to get config dir")?;
        let confdir = confdir.join(APP_NAME);
        let is_exist = confdir.exists();
        if !is_exist {
            fs::create_dir_all(&confdir).context("failed to create config dir")?;
        };

        let confpath = confdir.join("config.yaml");
        let is_exist = confpath.exists();
        if !is_exist {
            Config::initialize_config_file()?;
        }

        let mut file = fs::File::open(confpath).context("failed to open config file")?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)
            .context("failed to read config file")?;
        let config: Config =
            serde_yaml::from_str(&contents).context("failed to parse config file")?;

        Ok(config)
    }

    pub fn initialize_config_file() -> anyhow::Result<()> {
        let path = Config::config_file_path().context("failed to get config file path")?;
        let mut file = fs::File::create(path).context("failed to create config file")?;
        let default = default_config();
        file.write_all(default.as_bytes())
            .context("failed to write config file")?;

        Ok(())
    }

    pub fn config_file_path() -> anyhow::Result<String> {
        let path = dirs::config_dir().context("failed to get config dir")?;
        let path = path.join(APP_NAME).join("config.yaml");
        Ok(path.to_str().context("")?.to_string())
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            cookie: Some(CookieConfig::default()),
            bbsmenu: BBSMenuConfig::default(),
            board: None,
            login: None,
            post: None,
            proxy: None,
        }
    }
}

fn default_config() -> String {
    r##"
bbsmenu:
    url:
        - https://menu.2ch.sc/bbsmenu.html

liked_board_path:
    custom: false
    path: $HOME/.config/termchan/liked.json

# login:
#     url: http
#     email: email@example.com
#     password: password

# post:
#     use_login: false
#     repost_interval: 0

# proxy:
#     host: host
#     port: 9999
#     user: user
#     password: password

cookie:
    path: $HOME/.config/termchan/cookie.json
"##
    .to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_yaml() {
        let confdir = dirs::config_dir().unwrap();
        let confdir = confdir.join(APP_NAME);
        let confpath = confdir.join("config.yaml");

        fs::remove_file(confpath).unwrap_or_default();
        fs::remove_dir_all(confdir).unwrap_or_default();

        let config = Config::load().context("failed to load config");

        println!("{:?}", config);
        assert!(false);
    }
}
