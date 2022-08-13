use std::sync::Arc;

use eyre::bail;
use reqwest::cookie::Jar;

use super::form::login::LoginFormData;

pub async fn do_login(email: &str, password: &str) -> eyre::Result<Arc<Jar>> {
    // 申し訳程度の検索よけ
    let url = "\x68\x74\x74\x70\x73\x3a\x2f\x2f\x6c\x6f\x67\x69\x6e\x2e\x35\x63\x68\x2e\x6e\x65\x74\x2f\x6c\x6f\x67\x2e\x70\x68\x70";
    let host = "\x6c\x6f\x67\x69\x6e\x2e\x35\x63\x68\x2e\x6e\x65\x74";

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
    let form_data = LoginFormData::new(password, &email).build();
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
    if html.contains("ログインできません") {
        bail!("ログインできません");
    } else if html.contains("ログインしました") {
        Ok(Arc::clone(&jar))
    } else if html.contains("ログインしています") {
        Ok(Arc::clone(&jar))
    } else {
        bail!("ERROR: login failed");
    }
}
