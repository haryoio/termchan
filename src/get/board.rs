use std::{io::Write, str::Bytes};

use chrono::{DateTime, TimeZone, Utc};
use chrono_tz::{Asia::Tokyo, Tz};
use eyre::{eyre, ContextCompat, Result, WrapErr};

use crate::util::encoding::sjis_to_utf8;

#[derive(Debug, Clone)]
pub struct ThreadSubject {
    pub board_name: String,
    pub name:       String,
    pub id:         String,
    pub url:        String,
    pub count:      i32,
    pub ikioi:      f64,
    pub created_at: DateTime<Tz>,
}
impl Default for ThreadSubject {
    fn default() -> Self {
        ThreadSubject {
            board_name: "".to_string(),
            name:       "".to_string(),
            id:         "".to_string(),
            url:        "".to_string(),
            count:      0,
            ikioi:      0.0,
            created_at: Tokyo.timestamp(0, 0),
        }
    }
}

#[derive(Debug)]
pub struct Board {
    pub url:    String,
    pub scheme: String,
    pub host:   String,
    pub name:   String,
}

impl Board {
    pub fn new(url: String) -> Result<Self> {
        let mut spurl = url.split("/");
        let mut scheme = spurl.next().context(eyre!(" {}", url.clone()))?.to_string();
        scheme.pop();
        spurl.next();
        let host = spurl.next().context(eyre!(" {}", url.clone()))?.to_string();
        let name = spurl.next().context(eyre!(" {}", url.clone()))?.to_string();
        Ok(Self {
            url,
            scheme,
            host,
            name,
        })
    }
    pub async fn get(&self) -> Result<Vec<ThreadSubject>> {
        let byte = reqwest::get(format!("{}/subject.txt", &self.url))
            .await?
            .bytes()
            .await?;
        let html = String::from_utf8(byte.to_vec());
        let dat: String = match html {
            Ok(html) => html,
            Err(_) => sjis_to_utf8(&byte),
        };

        parse_board_dat(&dat, &self)
    }
}

fn parse_board_dat(dat: &str, board: &Board) -> Result<Vec<ThreadSubject>> {
    let mut thread_subjects: Vec<ThreadSubject> = Vec::new();
    let mut lines = dat.split('\n');
    loop {
        let line = match lines.next() {
            Some(line) => line,
            None => break,
        };
        if line.is_empty() {
            break;
        }
        let mut name = String::new();
        let mut splitted = line.split(".dat<>");
        let id = splitted
            .next()
            .context(eyre!(" {}", line.clone()))?
            .to_string();
        let right = splitted
            .next()
            .context(eyre!(" {}", line.clone()))?
            .to_string();

        let mut splitted = right.split(" (");
        let count = splitted
            .clone()
            .last()
            .context(eyre!(" {}", line.clone()))?;

        let count = count[..count.len() - 1]
            .parse::<i32>()
            .context(eyre!(" {}", line.clone()))?;

        let url = format!(
            "{}://{}/test/read.cgi/{}/{}",
            &board.scheme, &board.host, &board.name, &id
        );

        for c in splitted.next().context(eyre!(" {}", line.clone()))?.chars() {
            name.push(c);
        }

        // rep_count / ((now - first_rep) / 86400)

        let now = Utc::now().with_timezone(&Tokyo).timestamp() as usize;
        let created_at = Tokyo.timestamp(id.parse::<i64>().context(eyre!(" {}", line.clone()))?, 0);
        let first_resp: usize = created_at.timestamp() as usize;

        let ikioi = if now >= first_resp {
            count as f64 / ((now - first_resp) as f64 / 86400.0)
        } else {
            0.0
        };

        thread_subjects.push(ThreadSubject {
            board_name: board.name.clone(),
            id,
            name,
            url,
            count,
            ikioi,
            created_at,
        });
    }
    Ok(thread_subjects)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_parse_dat() {
        let url = "https://mi.5ch.net/news4vip/";
        let board = Board::new(url.to_string()).unwrap();
        let subjects = board.get().await.unwrap();
        for subject in subjects {
            println!("{:?}", subject);
        }
    }

    #[test]
    fn test_parse_dats() {
        let dat = r#"0000000000.dat<>テスト (9999)
9999999999.dat<><><><> (0)
0000000000.dat<>(テ)(ス)(ト) (1)
0000000000.dat<>(1000) (1001)
0000000000.dat<> (0) (9999999)"#;
        let board = Board {
            url:    "https://bbs.test.net/testboard/".to_string(),
            scheme: "https".to_string(),
            host:   "bbs.test.net".to_string(),
            name:   "testboard".to_string(),
        };
        let subjects = parse_board_dat(dat, &board);
        for subject in subjects.unwrap() {
            println!("{:?}", subject);
        }
    }
}
