use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::backend::{Table, TableEntry};

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    pub name: String,
    pub subgroups: Option<Vec<Group>>,
    pub purposes: Option<Vec<Purpose>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Purpose {
    pub name: String,
    pub recommendations: Vec<Recommendation>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Recommendation {
    pub name: String,
    pub notes: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CratesData {
    pub crate_groups: Vec<Group>,
}

impl From<&Group> for Table {
    fn from(value: &Group) -> Self {

        let tableEntries  : Vec<TableEntry> = Vec::new();




        Table { entries: vec![] }
    }
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
