use std::collections::HashMap;

use crate::{
    url::thread::BoardParams,
    util::{encoding::utf8_to_sjis_string, time::unix_now_time},
};

pub struct ThreadFormData {
    submit:  String, // submit
    subject: String, // 見出し
    form:    String, // name
    mail:    String, // email
    message: String, // message
    site:    String, // top_board_name
    bbs:     String, // board_name
    time:    String, // unix_time
    cert:    String, // hash
}

impl ThreadFormData {
    pub fn new(
        subject: &str,
        message: &str,
        email: Option<&str>,
        name: Option<&str>,
        board_params: &BoardParams,
        cert: &str,
    ) -> Self {
        let time = unix_now_time().to_string();
        let name = name.unwrap_or("").to_string();
        let mail = email.unwrap_or("").to_string();
        let site = format!("top_{}", board_params.board_key);
        ThreadFormData {
            submit: "新規スレッド作成".to_string(),
            subject: subject.to_string(),
            form: name.clone(),
            mail,
            message: message.to_string(),
            site,
            bbs: board_params.board_key.clone(),
            time,
            cert: cert.to_string(),
        }
    }

    pub fn build(&self) -> String {
        let mut form = HashMap::new();
        form.insert("submit", &self.submit);
        form.insert("subject", &self.subject);
        form.insert("FORM", &self.form);
        form.insert("mail", &self.mail);
        form.insert("MESSAGE", &self.message);
        form.insert("site", &self.site);
        form.insert("bbs", &self.bbs);
        form.insert("time", &self.time);
        form.insert("cert", &self.cert);
        form.iter_mut()
            .map(|(k, v)| (*k, utf8_to_sjis_string(v)))
            .collect::<HashMap<_, _>>()
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&")
    }
}
