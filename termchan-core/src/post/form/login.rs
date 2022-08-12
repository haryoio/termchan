use std::collections::HashMap;

use crate::{
    url::reply::ThreadParams,
    util::{encoding::utf8_to_sjis_string, time::unix_now_time},
};

pub struct LoginFormData {
    pub pw:    String,
    pub em:    String,
    pub login: String,
}

impl LoginFormData {
    pub fn new(password: &str, email: &str) -> Self {
        LoginFormData {
            pw:    password.to_string(),
            em:    email.to_string(),
            login: "".to_string(),
        }
    }

    pub fn build(&self) -> String {
        let mut form = HashMap::new();
        form.insert("pw", &self.pw);
        form.insert("em", &self.em);
        form.insert("login", &self.login);
        form.iter_mut()
            .map(|(k, v)| (*k, utf8_to_sjis_string(v)))
            .collect::<HashMap<_, _>>()
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }
}
