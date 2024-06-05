use crate::{
    backend::{Categories, CategoriesWithSubCategories, Table},
    scraper::scraper::{scrape_site, CratesData, Group},
};

use super::ContentParser;

#[derive(Debug)]
pub struct JsonContentParser {
    pub content: CratesData,

    general_crates: Table,
    math_crates: Table,
    ffi_crates: Table,
    cryptography_crates: Table,
    common_crates: Table,
    concurrency_crates: Table,
    networking_crates: Table,
    database_crates: Table,
    clis_crates: Table,
    graphics_crates: Table,
}

impl JsonContentParser {
    pub async fn parse_content() -> Self {
        let page_content = scrape_site().await.unwrap();

        let mut general_crates = Table::default();
        let mut math_crates = Table::default();
        let mut ffi_crates = Table::default();
        let mut cryptography_crates = Table::default();
        let mut common_crates = Table::default();
        let mut concurrency_crates = Table::default();
        let mut networking_crates = Table::default();
        let mut database_crates = Table::default();
        let mut clis_crates = Table::default();
        let mut graphics_crates = Table::default();

        for group in &page_content.crate_groups {
            match group.name.trim().to_lowercase().as_str() {
                "common" => {
                    //extract general table
                    //the rest belongs to common crates , ignoring general
                    let mut common_table = Group::default();

                    let mut general_table = group.subgroups.as_ref().unwrap().iter();

                    general_crates = general_table
                        .find(|sub| sub.name.trim().to_lowercase() == "general")
                        .unwrap()
                        .into();

                    let new_subgropus: Vec<Group> = group
                        .subgroups
                        .as_ref()
                        .unwrap()
                        .iter()
                        .filter(|gr| gr.name.trim().to_lowercase() != "general")
                        .cloned()
                        .collect();

                    common_table.subgroups = Some(new_subgropus);
                    common_crates = Into::into(&common_table);
                }
                "math / scientific" => math_crates = group.into(),
                "ffi / interop" => ffi_crates = group.into(),
                "cryptography" => cryptography_crates = group.into(),
                "networking" => networking_crates = group.into(),
                "databases" => database_crates = group.into(),
                "clis" => clis_crates = group.into(),
                "concurrency" => concurrency_crates = group.into(),
                "graphics" => graphics_crates = group.into(),
                _ => {}
            }
        }

        Self {
            content: page_content,
            graphics_crates,
            math_crates,
            ffi_crates,
            cryptography_crates,
            common_crates,
            concurrency_crates,
            networking_crates,
            database_crates,
            clis_crates,
            general_crates,
        }
    }
}

impl ContentParser for JsonContentParser {
    fn get_general_crates(&self) -> Table {
        self.general_crates.clone()
    }

    fn get_crates(&self, category: &Categories) -> Table {
        match category {
            Categories::FFI => self.ffi_crates.clone(),
            Categories::Math => self.math_crates.clone(),
            Categories::Cryptography => self.cryptography_crates.clone(),
        }
    }

    fn get_crates_with_sub(&self, category: &CategoriesWithSubCategories) -> Table {
        match category {
            CategoriesWithSubCategories::Clis => self.clis_crates.clone(),
            CategoriesWithSubCategories::Common => self.common_crates.clone(),
            CategoriesWithSubCategories::Graphics => self.graphics_crates.clone(),
            CategoriesWithSubCategories::Databases => self.database_crates.clone(),
            CategoriesWithSubCategories::Networking => self.networking_crates.clone(),
            CategoriesWithSubCategories::Concurrency => self.concurrency_crates.clone(),
        }
    }
}
