use std::sync::Arc;

use eyre::Result;
use keyring::{self, Entry};
use reqwest::cookie::Jar;
use termchan_core::post::login::do_login;

pub const PASS_USER: &str = "termchan_pass";
pub const EM_USER: &str = "termchan_email";
pub const KEYRING_SERVICE_NAME: &str = "termchan";

pub struct Account {
    em: Option<String>,
    pw: Option<String>,
}
#[allow(dead_code)]
impl Account {
    pub fn new() -> Account {
        let entry = Entry::new(&KEYRING_SERVICE_NAME, &PASS_USER);
        let pw = match entry.get_password() {
            Ok(pw) => Some(pw),
            Err(_) => None,
        };

        let entry = Entry::new(&KEYRING_SERVICE_NAME, &EM_USER);
        let em = match entry.get_password() {
            Ok(p) => Some(p),
            Err(_) => None,
        };
        Account { em, pw }
    }
    pub fn set_passwd(&self, pw: &str) -> Result<()> {
        let entry = Entry::new(&KEYRING_SERVICE_NAME, &PASS_USER);
        entry.set_password(&pw)?;
        Ok(())
    }
    pub fn set_em(&self, em: &str) -> Result<()> {
        let entry = Entry::new(&KEYRING_SERVICE_NAME, &EM_USER);
        entry.set_password(&em)?;
        Ok(())
    }
    pub async fn get_jar(&self) -> Result<Arc<Jar>> {
        if self.em.is_none() || self.pw.is_none() {
            return Ok(Arc::new(Jar::default()));
        }
        let jar = do_login(&self.em.as_ref().unwrap(), &self.pw.as_ref().unwrap()).await;
        jar
    }
}
