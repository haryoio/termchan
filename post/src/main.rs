use std::{collections::HashMap, hash::Hash};
extern crate post;
use anyhow;
use post::{
    form::data::ReplyFormData,
    header::{self, build::generate_header, cookie::Cookies},
    url::reply::BoardParams,
};
use reqwest;
use termchan::{
    configs::board,
    controller::{board::Board, thread::Thread},
};

struct Reply {
    url: String,
}

async fn post_reply(
    url: &str,
    message: &str,
    name: Option<&str>,
    mail: Option<&str>,
) -> anyhow::Result<String> {
    let mut client = reqwest::Client::new();
    let board_params = BoardParams::new(url.to_string());
    let form_data = ReplyFormData::new(message, mail, name, &board_params).build();
    println!("{}", form_data);

    let mut cookies = Cookies::new();
    cookies.add("yuki", "akari");
    cookies.add("READJS", "\"off\"");

    let header = generate_header(&board_params, cookies);

    let res = client
        .post(board_params.build_post())
        .headers(header)
        .body(form_data)
        .send()
        .await?;
    let body = res.text().await?;
    Ok(body)
}

#[tokio::main]
async fn main() {
    let url = "https://mi.5ch.net/news4vip/";
    let board = Board::new(url.to_string());
    let thread = board.load().await.unwrap();
    let ten = thread.get(10).unwrap();
    let th_url = ten.url().clone();
    let message = "test";

    let res = post_reply(&th_url, &message, None, None).await.unwrap();
    println!("{}", res);
}
