use std::str::FromStr;

use anyhow;
use reqwest::{self, Url};

use crate::{
    header::{build::post_header, cookie::Cookies},
    post::form::reply::ReplyFormData,
    url::{reply::ThreadParams, url::URL},
};

pub async fn post_reply(
    url: &str,
    message: &str,
    name: Option<&str>,
    mail: Option<&str>,
) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let thread_params = ThreadParams::new(url);
    let form_data = ReplyFormData::new(message, mail, name, &thread_params).build();
    println!("{}", form_data);

    let mut cookies = Cookies::new();
    cookies.add("yuki", "akari");
    cookies.add("READJS", "\"off\"");

    let header = post_header(Url::from_str(url).unwrap(), cookies);

    // 一度目書き込み
    let res = client
        .post(thread_params.build_post())
        .headers(header.clone())
        .body(form_data.clone())
        .send()
        .await;

    let body = &res?.text().await?;

    // 書き込み確認画面が出た場合再度書き込み
    if body.contains("■ 書き込み確認 ■") {
        let res = client
            .post(thread_params.build_post())
            .headers(header)
            .body(form_data)
            .send()
            .await;
        let body = &res?.text().await?;

        return Ok(body.to_string());
    }
    if body.contains("書き込みが完了しました") {
        return Ok(body.to_string());
    }

    Ok(body.to_string())
}
