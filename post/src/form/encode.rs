use std::collections::HashMap;

use encoding_rs;

use crate::{url::reply::BoardParams, util::time::unix_now_time};

pub fn postable_string(s: &str) -> String {
    let (message, ..) = encoding_rs::SHIFT_JIS.encode(s);
    let message = message.to_vec();
    let message = unsafe { &*std::str::from_utf8_unchecked(&message) };

    message.to_string()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_postable_string() {
        let s = "こんにちは";
        let s = postable_string(s);
        assert_eq!(s, "%B1%F2%F1%B9%FA");
    }
}
