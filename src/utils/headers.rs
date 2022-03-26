use std::{borrow::Cow, collections::HashMap, vec};

use anyhow::Context;
use reqwest::header::{
    HeaderMap, HeaderName, HeaderValue, ACCEPT, ACCEPT_ENCODING, ACCEPT_LANGUAGE, CACHE_CONTROL,
    CONTENT_TYPE, COOKIE, HOST, ORIGIN, REFERER, UPGRADE_INSECURE_REQUESTS, USER_AGENT,
};

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

pub fn vec_to_cookie(map: Vec<(&str, &str)>) -> String {
    let mut s = String::new();
    for (key, value) in map {
        s.push_str(key);
        s.push_str("=");
        s.push_str(value);
        s.push_str("; ");
    }
    s = s.trim().to_string();
    s.pop();
    s
}

pub fn base_headers() -> Vec<(HeaderName, String)> {
    let map = vec![(
        ACCEPT,
        "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,image/apng,*/*;q=0.8,application/signed-exchange;v=b3",
    ),
    (ACCEPT_ENCODING, "gzip, deflate, br"),
    (ACCEPT_LANGUAGE, "ja,en-US;q=0.9,en;q=0.8"),
    (CACHE_CONTROL, "max-age=0"),
    (CONTENT_TYPE, "application/x-www-form-urlencoded"),
    (UPGRADE_INSECURE_REQUESTS, "1"),
    (USER_AGENT, "Mozilla/5.0 (X11; Linux x86_64; rv:98.0) Gecko/20100101 Firefox/98.0")];
    let map = map.into_iter().map(|(k, v)| (k, v.to_string())).collect();
    map
}
pub fn vec_to_headers(vec: Vec<(HeaderName, String)>) -> anyhow::Result<HeaderMap> {
    let mut header_map = HeaderMap::new();
    for (key, value) in vec {
        header_map.insert(
            key,
            HeaderValue::from_str(&value).context("header value parse error")?,
        );
    }
    Ok(header_map)
}

pub fn vec_to_formvalue(vec: Vec<(&str, &str)>) -> anyhow::Result<String> {
    let mut s = String::new();
    for (key, value) in vec {
        s.push_str(key);
        s.push_str("=");
        s.push_str(value);
        s.push_str("&");
    }
    s = s.trim().to_string();
    s.pop();
    Ok(s)
}

pub fn getable_headers(host: &str, cookie: &str) -> anyhow::Result<HeaderMap> {
    let mut map = base_headers();
    let get_header = vec![(HOST, host), (COOKIE, cookie)];
    let mut get_header = get_header
        .into_iter()
        .map(|(k, v)| (k, v.to_string()))
        .collect();
    map.append(&mut get_header);
    let headers = vec_to_headers(map)?;

    Ok(headers)
}
