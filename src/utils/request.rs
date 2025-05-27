use reqwest::Error;

pub async fn make_request(url: String) -> Result<String, Error> {
    let response = reqwest::get(url.to_string()).await?;
    let body = response.text().await?;
    Ok(body)
}
