pub mod cache;
pub mod config;
pub mod credentials;
pub mod dirs;
pub mod theme;

#[cfg(test)]
mod config_test {
    use super::*;
    use crate::config::config::Config;

    #[test]
    fn test_load_config() {
        let congig = Config::pretty_json().unwrap();
        println!("{}", congig);
    }
}
