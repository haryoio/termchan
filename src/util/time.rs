use anyhow::Result;
use chrono::prelude::*;

pub fn unix_now_time() -> i64 {
    let now = std::time::SystemTime::now();
    let now = now.duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
    now as i64
}

pub fn decode_japan_date(date: &str) -> Result<i64> {
    let date = format_jp_weekly_to_en(date);
    let date = remove_trailing_year(date.as_str());

    let format = if date.len() == 21 {
        "%y/%m/%d %T%.3f"
    } else {
        "%y/%m/%d %T"
    };

    let res = NaiveDateTime::parse_from_str(date.as_str(), format);
    match res {
        Ok(d) => Ok(d.timestamp()),
        Err(e) => Ok(0),
    }
}

pub fn format_jp_weekly_to_en(date: &str) -> String {
    date.replace("(日)", "")
        .replace("(月)", "")
        .replace("(火)", "")
        .replace("(水)", "")
        .replace("(木)", "")
        .replace("(金)", "")
        .replace("(土)", "")
}

pub fn remove_trailing_year(date: &str) -> String {
    let len = date.len();
    match &len {
        22 => return format!("{}0", &date[2..len]),
        19 => return format!("{}", &date[2..len]),
        23 => return format!("{}", &date[2..len]),
        _ => return date.to_string(),
    }
}

#[cfg(test)]
mod time_tests {

    #[test]
    fn test_unix_now_time() {
        let now = super::unix_now_time();
        println!("{}", now);
    }

    #[test]
    fn test_decode_japan_date() {
        let date = super::decode_japan_date("2022/07/26(火) 14:18:49.270").unwrap();
        println!("ok {}", date);
        let date = super::decode_japan_date("2022/07/26(火) 14:18:49.27").unwrap();
        println!("ok {}", date);
        let date = super::decode_japan_date("20/02/01(日) 00:00:00").unwrap();
        println!("ok {}", date);
        let date = super::decode_japan_date("2020/02/01(日) 00:00:00").unwrap();
        println!("ok {}", date);
    }
}
