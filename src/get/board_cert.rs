pub async fn board_cert(url: String) -> anyhow::Result<String> {
    let client = reqwest::Client::new();
    let res = client.get(url).send().await;
    let body = &res?.text().await?;

    let mut cert_n = body.split(r#"<input type="hidden" name="cert" value=""#);
    let cert = cert_n
        .nth(1)
        .unwrap()
        .to_string()
        .split("\">")
        .nth(0)
        .unwrap()
        .to_string();
    Ok(cert.to_string())
}
