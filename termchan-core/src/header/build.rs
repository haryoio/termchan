use std::{collections::HashMap, str::FromStr};

use reqwest::{
    header::{HeaderMap, HeaderName, HeaderValue},
    Url,
};

use super::cookie::Cookies;

fn string_to_map(header: &str) -> HashMap<String, String> {
    let mut map = HashMap::new();
    let mut split = header.split("\n");
    loop {
        let line = split.next();
        if line.is_none() {
            break;
        }
        let line = line.unwrap();
        let mut split = line.split(": ");
        let key = split.next();
        let value = split.next();
        if key.is_none() || value.is_none() {
            continue;
        }
        map.insert(
            key.unwrap().to_string().trim().to_string(),
            value.unwrap().to_string().trim().to_string(),
        );
    }
    map
}

pub fn base_header<'a>(url: Url, cookie: Cookies) -> HashMap<String, String> {
    let mut header = HashMap::new();
    header.insert("Host".to_string(), url.host().unwrap().to_string());
    header.insert("cookie".to_string(), cookie.to_string());
    header.insert("referer".to_string(), url.to_string());
    header.insert(
        "origin".to_string(),
        url.origin().unicode_serialization().to_string(),
    );

    header
}

pub(crate) fn post_header(url: Url, cookie: Cookies, header_string: String) -> HeaderMap {
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

    let two_header = string_to_map(&header_string);
    for (key, value) in two_header {
        header.insert(key, value);
    }

    map_to_headermap(header)
}

pub fn get_header(url: Url) -> HeaderMap {
    let mut cookie = Cookies::new();
    cookie.add("yuki", "akari");
    cookie.add("READJS", "\"off\"");
    let mut header = base_header(url, cookie);
    header.insert("Accept".to_string(), "*/*".to_string());
    map_to_headermap(header)
}

pub fn map_to_headermap(map: HashMap<String, String>) -> HeaderMap {
    println!("{:?}", map);
    let mut header = HeaderMap::new();
    for (key, value) in map {
        header.insert(
            HeaderName::from_str(&key.trim()).expect("failed to parse header name"),
            HeaderValue::from_str(&value.trim()).expect("failed to parse header value"),
        );
    }
    header
}
