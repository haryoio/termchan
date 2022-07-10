use std::str::Bytes;

use crate::util::encoding::sjis_to_utf8;

// https://mi.5ch.net/news4vip/subject.txt
#[derive(Debug)]
pub struct ThreadSubjectList {
    pub board_name:      String,
    pub thread_subjects: Vec<ThreadSubject>,
}
#[derive(Debug)]
pub struct ThreadSubject {
    pub name:  String,
    pub id:    String,
    pub url:   String,
    pub count: i32,
}

#[derive(Debug)]
pub struct Board {
    pub url:    String,
    pub scheme: String,
    pub host:   String,
    pub name:   String,
}

impl Board {
    pub fn new(url: String) -> Self {
        let mut spurl = url.split("/");
        let mut scheme = spurl.next().unwrap().to_string();
        scheme.pop();
        let host = spurl.next().unwrap().to_string();
        let name = spurl.next().unwrap().to_string();
        Self {
            url,
            scheme,
            host,
            name,
        }
    }
    async fn load(&self) -> ThreadSubjectList {
        let byte = reqwest::get(&self.url)
            .await
            .unwrap()
            .bytes()
            .await
            .unwrap();
        let html = String::from_utf8(byte.to_vec());
        let dat: String = match html {
            Ok(html) => html,
            Err(_) => sjis_to_utf8(&byte),
        };
        parse_board_dat(&dat, &self)
    }
}

fn parse_board_dat(dat: &str, board: &Board) -> ThreadSubjectList {
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
        let l = line.split("<>").collect::<Vec<&str>>();
        let thread_id = l[0].to_string()[..l[0].len() - 4].to_string();
        let right = l[1].split(" (").collect::<Vec<&str>>();
        let subject = right[0].to_string();
        let url = format!(
            "{}/{}/test/read.cgi/{}/{}",
            board.scheme, board.host, board.name, thread_id
        );
        let count = i32::from_str_radix(&right[1][..right.len() - 1], 10).unwrap();
        thread_subjects.push(ThreadSubject {
            id: thread_id,
            name: subject,
            url,
            count,
        });
    }
    ThreadSubjectList {
        board_name: board.name.to_string(),
        thread_subjects,
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_parse_board_dat() {
        let dat = "https://mi.5ch.net/news4vip/subject.txt";
        let board = Board::new(dat.to_string());
        let subjects = board.load().await;

        assert_eq!(board.url, "https://mi.5ch.net/news4vip/subject.txt");
    }
}
