pub fn scrape_site() -> Result<String, anyhow::Error> {
    let response = reqwest::blocking::get("https://blessed.rs/crates")?.text()?;
    Ok(response)
}
