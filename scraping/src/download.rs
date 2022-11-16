pub async fn download(url: &str) -> Result<String, Box<dyn std::error::Error>> {
    println!("Downloading from url {}", url);

    let body = reqwest::get(url).await?.text().await?;

    Ok(body)
}
