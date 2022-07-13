use std::vec;

use anyhow::Context;
use reqwest::header::{
    HeaderMap,
    HeaderName,
    HeaderValue,
    ACCEPT,
    ACCEPT_ENCODING,
    ACCEPT_LANGUAGE,
    CACHE_CONTROL,
    CONNECTION,
    CONTENT_TYPE,
    UPGRADE_INSECURE_REQUESTS,
    USER_AGENT,
};

pub fn sjis_to_utf8(bytes: &[u8]) -> anyhow::Result<String> {
    let (res, ..) = encoding_rs::SHIFT_JIS.decode(bytes);
    Ok(res.to_string())
}

pub fn cookie_from_vec(map: Vec<(&str, &str)>) -> String {
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
    let map = vec![
        (
            ACCEPT,
            "text/html,application/xhtml+xml,application/xml;q=0.9,image/avif,image/webp,*/*;q=0.8",
        ),
        (ACCEPT_ENCODING, "gzip, deflate, br"),
        (ACCEPT_LANGUAGE, "ja,en-US;q=0.7,en;q=0.3"),
        (CONTENT_TYPE, "application/x-www-form-urlencoded"),
        (UPGRADE_INSECURE_REQUESTS, "1"),
        (
            USER_AGENT,
            "Mozilla/5.0 (X11; Linux x86_64; rv:102.0) Gecko/20100101 Firefox/102.0",
        ),
        (CONNECTION, "keep-alive"),
    ];
    let map = map.into_iter().map(|(k, v)| (k, v.to_string())).collect();
    map
}

pub fn headers_from_vec(vec: Vec<(HeaderName, String)>) -> anyhow::Result<HeaderMap> {
    let mut header_map = HeaderMap::new();
    for (key, value) in vec {
        header_map.insert(
            key,
            HeaderValue::from_str(&value).context("header value parse error")?,
        );
    }
    Ok(header_map)
}

pub fn formvalue_from_vec(vec: Vec<(&str, &str)>) -> anyhow::Result<String> {
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

pub fn is_gzip(buf: &[u8]) -> bool {
    buf.len() >= 2 && buf[0] == 0x1f && buf[1] == 0x8b
}

pub fn is_utf8(buf: &[u8]) -> bool {
    std::str::from_utf8(buf).is_ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_is_utf8() {
        let c = "あいうえお";
        let (buf, ..) = encoding_rs::UTF_8.encode(c);
        assert!(is_utf8(&buf));
    }
    #[test]
    fn test_is_not_utf8() {
        let c = "あいうえお";
        let (buf, ..) = encoding_rs::SHIFT_JIS.encode(c);
        assert!(!is_utf8(&buf));
    }
}
