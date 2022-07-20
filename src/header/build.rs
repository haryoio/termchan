use std::collections::HashMap;

use reqwest::header::HeaderMap;

use super::cookie::Cookies;
use crate::url::url::URL;

pub fn base_header<'a>(url: impl URL, cookie: Cookies) -> HashMap<String, String> {
    let mut header = HashMap::new();
    header.insert("Host".to_string(), url.host().to_string());
    header.insert(
        "User-Agent".to_string(),
        "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/103.0.0.0 Safari/537.36".to_string(),
    );
    header.insert(
        "Accept-Language".to_string(),
        "ja,en-US;q=0.7,en;q=0.3".to_string(),
    );
    header.insert(
        "Accept-Encoding".to_string(),
        "gzip, deflate, br".to_string(),
    );
    header.insert("origin".to_string(), url.origin().to_string());
    header.insert("connection".to_string(), "keep-alive".to_string());
    header.insert("referer".to_string(), url.referer().to_string());
    header.insert("sec-fetch-dest".to_string(), "document".to_string());
    header.insert("sec-fetch-mode".to_string(), "navigate".to_string());
    header.insert("sec-fetch-site".to_string(), "same-origin".to_string());
    header.insert("sec-fetch-user".to_string(), "?1".to_string());
    header.insert("cookie".to_string(), cookie.to_string());
    header.insert(
        "sec-ch-ua".to_string(),
        r#" ".Not/A)Brand";v="99", "Google Chrome";v="103", "Chromium";v="103""#.to_string(),
    );
    header.insert("sec-ch-ua-mobile".to_string(), "?0".to_string());
    header.insert("sec-ch-ua-platform".to_string(), "macOS".to_string());
    header
}

pub(crate) fn post_header(url: impl URL, cookie: Cookies) -> HeaderMap {
    let mut header = base_header(url, cookie);
    header.insert(
        "Accept".to_string(),
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"
            .to_string(),
    );
    header.insert(
        "Content-Type".to_string(),
        "application/x-www-form-urlencoded".to_string(),
    );

    header.insert("upgrade-insecure-requests".to_string(), "1".to_string());
    map_to_headermap(header)
}

pub fn get_header(url: impl URL) -> HeaderMap {
    let mut cookie = Cookies::new();
    cookie.add("yuki", "akari");
    cookie.add("READJS", "\"off\"");
    let mut header = base_header(url, cookie);
    header.insert("Accept".to_string(), "*/*".to_string());
    map_to_headermap(header)
}

pub fn map_to_headermap(map: HashMap<String, String>) -> HeaderMap {
    let headers: HeaderMap = (&map).try_into().expect("valid headers");
    headers
}
