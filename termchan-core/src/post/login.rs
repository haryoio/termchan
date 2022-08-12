use std::sync::Arc;

use anyhow::Ok;
use reqwest::{
    cookie::{CookieStore, Jar},
    header::HeaderValue,
    Url,
};

use super::form::login::LoginFormData;


pub async fn ronin_login(email: &str, password: &str) -> anyhow::Result<Arc<Jar>> {
    let url = "https://login.5ch.net/log.php";
    let host = "login.5ch.net";

    let jar = Arc::new(Jar::default());
    let client = reqwest::Client::builder()
        .cookie_store(true)
        .cookie_provider(Arc::clone(&jar))
        .build()?;
    let resp = client
        .get(url)
        .header("Host", host.to_string())
        .send()
        .await?;

    // セッションが有効ならログイン処理をスキップ
    if resp.text().await.unwrap().contains("ログインしています") {
        return Ok(Arc::clone(&jar));
    }

    // ログイン画面のフォームデータを生成
    let email = email.replace("@", "%40");
    let form_data = LoginFormData::new(&email, password).build();
    // ログインリクエスト用のURLを生成
    let post_url = format!("https://{}/log.php", host);
    let resp = client
        .post(&post_url)
        .header("Host", host.to_string())
        .header("Referer", url)
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(form_data)
        .send()
        .await?;

    let html = &resp.text().await?;
    println!("{}", html);
    if html.contains("ログインできません") {
        return Err(anyhow::anyhow!(
            "ERROR: login failed (email or password is wrong)"
        ));
    } else if html.contains("ログインしました") {
        Ok(Arc::clone(&jar))
    } else if html.contains("ログインしています") {
        Ok(Arc::clone(&jar))
    } else {
        return Err(anyhow::anyhow!("ERROR: login failed"));
    }
}

#[cfg(test)]
mod login_test {
    #[tokio::test]
    async fn test_login() {
        let email = "mizusecocolte@gmail.com";
        let password = "Puruto638466!";
        let res = super::ronin_login(password, email).await;
        println!("{:?}", res);
    }
}
