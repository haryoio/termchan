use std::collections::HashMap;

use reqwest::header::HeaderMap;

use super::cookie::Cookies;
use crate::url::reply::BoardParams;

pub fn generate_header<'a>(board_params: &'a BoardParams, cookie: Cookies) -> HeaderMap {
    let mut header = HashMap::new();
    header.insert("Host".to_string(), board_params.host.to_string());
    header.insert(
        "User-Agent".to_string(),
        "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0".to_string(),
    );
    header.insert(
        "Accept".to_string(),
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8"
            .to_string(),
    );
    header.insert(
        "Accept-Language".to_string(),
        "ja,en-US;q=0.7,en;q=0.3".to_string(),
    );
    header.insert(
        "Accept-Encoding".to_string(),
        "gzip, deflate, br".to_string(),
    );
    header.insert(
        "Content-Type".to_string(),
        "application/x-www-form-urlencoded".to_string(),
    );
    header.insert("Origin".to_string(), board_params.origin().to_string());
    header.insert("Connection".to_string(), "keep-alive".to_string());
    header.insert("Referer".to_string(), board_params.referer().to_string());
    header.insert("Upgrade-Insecure-Requests".to_string(), "1".to_string());
    header.insert("Sec-Fetch-Dest".to_string(), "document".to_string());
    header.insert("Sec-Fetch-Mode".to_string(), "navigate".to_string());
    header.insert("Sec-Fetch-Site".to_string(), "same-origin".to_string());
    header.insert("Sec-Fetch-User".to_string(), "?1".to_string());
    header.insert("Cookie".to_string(), cookie.to_string());

    let headers: HeaderMap = (&header).try_into().expect("valid headers");
    headers
}
