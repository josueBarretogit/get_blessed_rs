use std::error::Error;

pub async fn scrape_site() -> Result<String, Box<dyn Error>> {
    let response = reqwest::get("https://blessed.rs/crates")
        .await?
        .text()
        .await?;
    Ok(response)
}
