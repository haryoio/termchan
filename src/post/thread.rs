use std::{collections::HashMap, error, hash::Hash};

use anyhow::{self, anyhow as any, Error, Ok};
use reqwest;

use crate::{
    get::board_cert::board_cert,
    header::{build::post_header, cookie::Cookies},
    post::form::thread::ThreadFormData,
    url::{thread::BoardParams, url::URL},
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

    let header = post_header(board_params.clone(), cookies);

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
