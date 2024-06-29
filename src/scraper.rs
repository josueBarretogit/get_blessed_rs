use serde::{Deserialize};

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Group {
    pub name: String,
    pub subgroups: Option<Vec<Group>>,
    pub purposes: Option<Vec<Purpose>>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Purpose {
    pub name: String,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct Recommendation {
    pub name: String,
    pub notes: Option<String>,
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct CratesData {
    pub crate_groups: Vec<Group>,
}

pub async fn scrape_site() -> Result<CratesData, reqwest::Error> {
    let response = reqwest::get(
        "https://raw.githubusercontent.com/nicoburns/blessed-rs/main/data/crates.json",
    )
    .await?
    .json::<CratesData>()
    .await?;
    Ok(response)
}
