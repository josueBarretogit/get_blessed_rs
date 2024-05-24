use std::error::Error;

use serde::{Deserialize, Serialize};

use crate::backend::{Crates, Table, TableEntry};

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
        let mut entries: Vec<TableEntry> = Vec::new();

        if let Some(subgroups) = &value.subgroups   {
            for subgroup in subgroups {
                let mut crates: Vec<Crates> = Vec::new();
                if let Some(purposes) = &subgroup.purposes {
                    for purpose in purposes {
                        purpose.recommendations.iter().for_each(|recommendation| {
                            crates.push(Crates {
                                name: recommendation.name.clone(),
                                description: recommendation
                                    .notes
                                    .clone()
                                    .unwrap_or("No description".to_string()),
                                features: None,
                            });
                        });
                    }
                }
                entries.push(TableEntry {
                    use_case: String::default(),
                    crates,
                });
            }
        } else if let Some(purposes) = &value.purposes {
            for purpose in purposes {
                let mut crates : Vec<Crates> = Vec::new();



            }
        }

        Table { entries }
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
