use reqwest;

pub async fn get_repoinfo(url: &str) -> Result<String, reqwest::Error> {
    let user_agent = "ersa/1.0";
    let response = reqwest::Client::new()
        .get(url)
        .header("User-Agent", user_agent)
        .send()
        .await?;
    let body = response.text().await?;
    Ok(body)
}
