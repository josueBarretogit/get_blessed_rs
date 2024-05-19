pub async  fn scrape_site() -> Result<String, anyhow::Error> {
    let response = reqwest::get("https://blessed.rs/crates").await?.text().await?;
    Ok(response)
}
