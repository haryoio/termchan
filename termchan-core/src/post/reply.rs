use std::{str::FromStr, sync::Arc};

use anyhow;
use reqwest::{
    self,
    cookie::{CookieStore, Jar},
    header::HeaderValue,
    Url,
};

use crate::{
    header::{build::post_header, cookie::Cookies},
    post::form::reply::ReplyFormData,
    url::{reply::ThreadParams, url::URL},
};

///
/// example
/// ```
/// let pass = Some("Password".to_string());
/// let mail = Some("Email".to_string());
/// let login = do_login(mail.as_ref().unwrap, pass.as_ref().unwrap()).await;
///
/// let message = "test";
/// let res = post_reply(url, message, None, None, (&header_str).to_string(), None).await;
/// ```
pub async fn post_reply(
    url: &str,
    message: &str,
    name: Option<String>,
    mail: Option<String>,
    header_str: String,
    jar: Option<Arc<Jar>>,
) -> anyhow::Result<String> {
    let mut login = false;
    let jar = jar.unwrap_or(Arc::new(Jar::default()));
    // 申し訳程度の検索よけ
    let login_str = &"\x68\x74\x74\x70\x73\x3a\x2f\x2f\x35\x63\x68\x2e\x6e\x65\x74\x2f"
        .parse::<Url>()
        .unwrap();

    let sid = Arc::clone(&jar)
        .cookies(&login_str)
        .unwrap_or(HeaderValue::from_str("").unwrap());
    let sid = if sid.is_empty() {
        ""
    } else {
        login = true;
        sid.to_str().unwrap().split("=").nth(1).unwrap()
    };

    let client = reqwest::Client::builder().cookie_store(true).build()?;
    let thread_params = ThreadParams::new(url);
    let form_data = ReplyFormData::new(message, mail, name, &thread_params).build();

    let mut cookies = Cookies::new();
    cookies.add("yuki", "akari");
    cookies.add("READJS", "\"off\"");
    if login {
        cookies.add("sid", sid);
    }

    let header = post_header(Url::from_str(url).unwrap(), cookies, header_str);

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
