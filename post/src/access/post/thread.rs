use std::{collections::HashMap, error, hash::Hash};

use anyhow::{self, anyhow as any, Error, Ok};
use reqwest;
use termchan::{
    configs::board,
    controller::{board::Board, thread::Thread},
};

use crate::{
    access::get::board_cert::board_cert,
    form::{reply::ReplyFormData, thread::ThreadFormData},
    header::{self, build::generate_header, cookie::Cookies},
    url::{reply::ThreadParams, thread::BoardParams, url::URL},
    util::error::get_error,
};

pub async fn create_thread(
    url: &str,
    subject: &str,
    message: &str,
    name: Option<&str>,
    mail: Option<&str>,
) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let board_params = BoardParams::new(url);
    let cert = board_cert(board_params.build_board_url()).await?;
    let form_data = ThreadFormData::new(subject, message, mail, name, &board_params, &cert).build();
    println!("{}", form_data);

    let mut cookies = Cookies::new();
    cookies.add("yuki", "akari");
    cookies.add("READJS", "\"off\"");

    let header = generate_header(board_params.clone(), cookies);

    let res = client
        .post(board_params.build_post())
        .headers(header.clone())
        .body(form_data.clone())
        .send()
        .await;

    let body = &res?.text().await?;
    return match get_error(body) {
        Err(err) => Err(any!(err)),
        _ => Ok(body.to_string()),
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_thread() {
        let res = create_thread("https://mi.5ch.net/news4vip/", "test", "test", None, None)
            .await
            .unwrap();
        println!("{}", res);
    }
}
