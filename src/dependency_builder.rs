/// This module's job is to add de crate or dependencies to the user's project
use std::fmt::Write;
use std::{io, process::Command};
use crate::view::widgets::{CrateItemList, ItemListStatus};


/// This is the data neccesary to add a crate to the user's project
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrateToAdd {
    pub crate_name: String,
    pub features: Option<Vec<String>>,
}


impl From<CrateItemList> for CrateToAdd {
    fn from(value: CrateItemList) -> Self {
        Self {
            crate_name: value.name,
            features: value.features.map(|features| {
                features
                    .iter()
                    .filter_map(|feature_item| {
                        if feature_item.status == ItemListStatus::Selected {
                            Some(feature_item.name.clone())
                        } else {
                            None
                        }
                    })
                    .collect()
            }),
        }
    }
}



impl From<&CrateItemList> for CrateToAdd {
    fn from(value: &CrateItemList) -> Self {
        Self {
            crate_name: value.name.clone(),
            features: value.features.as_ref().map(|features| {
                features
                    .iter()
                    .filter_map(|feature_item| {
                        if feature_item.status == ItemListStatus::Selected {
                            Some(feature_item.name.clone())
                        } else {
                            None
                        }
                    })
                    .collect()
            }),
        }
    }
}


pub struct DependenciesBuilder {
    crates_to_add: Vec<CrateToAdd>,
}

impl DependenciesBuilder {
    pub fn new(crates_to_add: Vec<CrateToAdd>) -> Self {
        Self { crates_to_add }
    }

    pub fn add_dependencies(&self) -> io::Result<()> {
        for dependency in self.crates_to_add.clone() {
            if let Some(features) = dependency.features {
                let features: String =
                    features.iter().fold(String::new(), |mut output, feature| {
                        let _ = write!(output, " {feature} ");
                        output
                    });
                Command::new("cargo")
                    .arg("add")
                    .arg(dependency.crate_name)
                    .arg("-F")
                    .arg(features)
                    .output()?;
            } else {
                Command::new("cargo")
                    .arg("add")
                    .arg(dependency.crate_name)
                    .arg("-q")
                    .output()?;
            }
        }
        Ok(())
    }
}
