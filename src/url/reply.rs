use super::url::URL;

#[derive(Debug, Clone)]
pub struct ThreadParams {
    pub url:       String,
    pub scheme:    String,
    pub host:      String,
    pub thread_id: String,
    pub board_key: String,
}
/// https://mi.5ch.net/test/read.cgi/news4vip/1656992645/l50"
impl From<&str> for ThreadParams {
    fn from(url: &str) -> Self {
        let origin_url = url.clone();
        let mut spurl = url.split("/");
        let mut scheme = spurl.next().unwrap().to_string();
        scheme.pop();
        spurl.next(); // ""
        let host = spurl.next().unwrap().to_string();
        spurl.next(); // "test"
        spurl.next(); // "read.cgi"
        let board_key = spurl.next().unwrap().to_string();
        let thread_id = spurl.next().unwrap().to_string();

        Self {
            url: origin_url.to_string(),
            scheme,
            host,
            thread_id,
            board_key,
        }
    }
}

impl URL for ThreadParams {
    fn new(url: &str) -> Self {
        Self::from(url)
    }
    fn origin(&self) -> String {
        format!("{}://{}", self.scheme, self.host)
    }
    fn host(&self) -> String {
        format!("{}", self.host)
    }
    fn referer(&self) -> String {
        format!(
            "{}://{}/test/read.cgi/{}/{}/l50",
            self.scheme, self.host, self.board_key, self.thread_id
        )
    }
}

impl ThreadParams {
    pub fn build_post(&self) -> String {
        format!("{}://{}/test/bbs.cgi", self.scheme, self.host)
    }
    pub fn build_get(&self) -> String {
        format!(
            "{}://{}/test/read.cgi/{}/{}/",
            self.scheme, self.host, self.board_key, self.thread_id
        )
    }
    pub fn build_board(&self) -> String {
        format!("{}://{}/{}/", self.scheme, self.host, self.board_key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_thread_url() {
        let url = "https://mi.5ch.net/test/read.cgi/news4vip/1656992645/l50";
        let board_params = ThreadParams::from(url);
        println!("{:?}", board_params);
        assert_eq!(board_params.board_key, "news4vip");
        assert_eq!(board_params.thread_id, "1656992645");
        assert_eq!(board_params.host(), "mi.5ch.net");
        assert_eq!(board_params.build_post(), "https://mi.5ch.net/test/bbs.cgi");
        assert_eq!(
            board_params.build_get(),
            "https://mi.5ch.net/test/read.cgi/news4vip/1656992645/"
        );
    }
}
