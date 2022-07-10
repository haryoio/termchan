use anyhow;
use reqwest;

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

    let header = post_header(thread_params.clone(), cookies);

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

#[cfg(test)]
mod tests {
    use termchan::controller::board::Board;

    use super::*;

    #[tokio::test]
    async fn test_post_reply() {
        let url = "https://mi.5ch.net/news4vip/";
        let board = Board::new(url.to_string());
        let thread = board.load().await.unwrap();
        let ten = thread.get(10).unwrap();
        let th_url = ten.url().clone();
        let message = "test";

        let res = post_reply(&th_url, &message, None, None).await.unwrap();
        println!("{}", res);
    }
}
