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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_board_cert() {
        let res = board_cert("https://mi.5ch.net/news4vip/".to_string())
            .await
            .unwrap();
        println!("{}", res);
    }
}
