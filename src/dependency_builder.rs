use std::{io, process::Command};

pub mod dependency_builder;

pub struct DependenciesBuilder {
    crate_names: Vec<String>,
}

impl DependenciesBuilder {
    pub fn new(crate_names: Vec<String>) -> Self {
        Self { crate_names }
    }

    pub fn add_dependencies(&self) -> io::Result<()> {
        for dependency in self.crate_names.clone() {
            Command::new("cargo")
                .arg("add")
                .arg(dependency)
                .arg("-q")
                .output()?;
        }
        Ok(())
    }
}
