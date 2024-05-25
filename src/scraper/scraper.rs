use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Group {
    pub name: String,
    pub subgroups: Option<Vec<Group>>,
    pub purposes: Option<Vec<Purpose>>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Purpose {
    pub name: String,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct Recommendation {
    pub name: String,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Default, Clone)]
pub struct CratesData {
    pub crate_groups: Vec<Group>,
}

pub async fn scrape_site() -> Result<CratesData, Box<dyn Error>> {
    let response = reqwest::get(
        "https://raw.githubusercontent.com/nicoburns/blessed-rs/main/data/crates.json",
    )
    .await?
    .json::<CratesData>()
    .await?;
    Ok(response)
}
