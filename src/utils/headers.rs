use reqwest::header::HeaderMap;

pub fn gen_cookie(session_id: Option<&str>) -> String {
    let mut cookie = String::new();
    if let Some(sid) = session_id {
        cookie.push_str("sid=");
        cookie.push_str(sid);
        cookie.push_str("; ");
    }
    cookie.push_str("READJS=off; SUBBACK_STYLE=1");
    cookie
}

pub fn getable_headers(host: &str, cookie: &str) -> HeaderMap {
    let host = host.to_owned();
    let cookie = cookie.to_owned();
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::COOKIE,
        reqwest::header::HeaderValue::from_str(&cookie.clone()).unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_str(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        )
        .unwrap(),
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36").unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT_ENCODING,
        reqwest::header::HeaderValue::from_str("gzip, deflate, br").unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        reqwest::header::HeaderValue::from_str("ja,en-US;q=0.9,en;q=0.8,zh-CN;q=0.7,zh;q=0.6")
            .unwrap(),
    );
    headers.insert(
        reqwest::header::CONNECTION,
        reqwest::header::HeaderValue::from_str("keep-alive").unwrap(),
    );
    headers.insert(
        reqwest::header::HOST,
        reqwest::header::HeaderValue::from_str(&host.clone()).unwrap(),
    );
    headers
}

fn postable_header() {
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert(
        reqwest::header::ACCEPT,
        reqwest::header::HeaderValue::from_str(
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        )
        .unwrap(),
    );
    headers.insert(
        reqwest::header::USER_AGENT,
        reqwest::header::HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/80.0.3987.132 Safari/537.36").unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT_ENCODING,
        reqwest::header::HeaderValue::from_str("gzip, deflate, br").unwrap(),
    );
    headers.insert(
        reqwest::header::ACCEPT_LANGUAGE,
        reqwest::header::HeaderValue::from_str("ja,en-US;q=0.9,en;q=0.8,zh-CN;q=0.7,zh;q=0.6")
            .unwrap(),
    );
    headers.insert(
        reqwest::header::CONNECTION,
        reqwest::header::HeaderValue::from_str("keep-alive").unwrap(),
    );
    headers.insert(
        reqwest::header::HOST,
        reqwest::header::HeaderValue::from_str("www.nicovideo.jp").unwrap(),
    );
    headers.insert(
        reqwest::header::REFERER,
        reqwest::header::HeaderValue::from_str("https://www.nicovideo.jp/").unwrap(),
    );
    headers.insert(
        reqwest::header::COOKIE,
        reqwest::header::HeaderValue::from_str("user_session=; user_session_time=; user_session")
            .unwrap(),
    );
}
