use std::{io, ops::Deref, process::Command};

pub mod dependency_builder;

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct CrateToAdd {
    pub crate_name: String,
    pub features: Option<Vec<String>>,
}

impl Deref for CrateToAdd {
    type Target = CrateToAdd;
    fn deref(&self) -> &Self::Target {
        self
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
                Command::new("cargo")
                    .arg("add")
                    .arg(dependency.crate_name)
                    .arg("-F")
                    .args(features)
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
