use crate::backend::{Categories, CategoriesWithSubCategories, Crates, Table, TableEntry};
use crate::scraper::scraper::Group;

pub mod jsoncontentparser;

pub trait ContentParser {
    fn get_general_crates(&self) -> Table;

    fn get_crates(&self, category: &Categories) -> Table;

    fn get_crates_with_sub(&self, category: &CategoriesWithSubCategories) -> Table;
}

impl From<&Group> for Table {
    fn from(value: &Group) -> Self {
        let mut entries: Vec<TableEntry> = Vec::new();

        //This means this is parsing a category with 1 table
        if let Some(purposes) = &value.purposes {
            for purpose in purposes {
                let mut crates: Vec<Crates> = Vec::new();

                for recommendation in &purpose.recommendations {
                    crates.push(Crates {
                        name: recommendation.name.clone(),
                        description: recommendation
                            .notes
                            .clone()
                            .unwrap_or("No description".to_string()),
                        features: None,
                    });
                }

                entries.push(TableEntry {
                    use_case: String::default(),
                    crates,
                });
            }

            return Table { entries };
        };

        //Parsing a category with multiple tables
        if let Some(subgroups) = &value.subgroups {
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
        };
        Table { entries }
    }
}

#[cfg(test)]
mod test {
    use self::jsoncontentparser::JsonContentParser;

    use super::*;


    async fn setup_json_content_parser() -> JsonContentParser {
        JsonContentParser::parse_content().await
    }

    fn test_section(entries: &[TableEntry], name_first_recommended_crate: &str) {
        assert!(!entries.is_empty());

        let first_entry = entries.first().unwrap();
        assert!(!first_entry.crates.is_empty());

        let first_recommended_crate = first_entry.crates.first().unwrap();

        assert_eq!(first_recommended_crate.name, name_first_recommended_crate);
    }

    #[tokio::test]
    async fn general_crates_has_expected_data() {
        let json_content_parser = setup_json_content_parser().await;
        let section = json_content_parser.get_general_crates();
        test_section(&section.entries, "rand");
    }

    #[tokio::test]
    async fn common_crates_has_expected_data() {
        let json_content_parser = setup_json_content_parser().await;
        let section = json_content_parser.get_crates_with_sub(&CategoriesWithSubCategories::Common);
        test_section(&section.entries, "anyhow");
    }

    #[tokio::test]
    async fn math_section_has_expected_data() {
        let json_content_parser = setup_json_content_parser().await;
        let section = json_content_parser.get_crates(&Categories::Math);
        test_section(&section.entries, "num-traits");
    }

    #[tokio::test]
    async fn ffi_section_has_expected_data() {
        let json_content_parser = setup_json_content_parser().await;
        let section = json_content_parser.get_crates(&Categories::FFI);
        test_section(&section.entries, "bindgen");
    }

    #[tokio::test]
    async fn cryptography_has_expected_data() {
        let json_content_parser = setup_json_content_parser().await;
        let section = json_content_parser.get_crates(&Categories::Cryptography);
        test_section(&section.entries, "argon2");
    }

    #[tokio::test]
    async fn concurrency_has_expected_data() {
        let json_content_parser = setup_json_content_parser().await;
        let section =
            json_content_parser.get_crates_with_sub(&CategoriesWithSubCategories::Concurrency);
        test_section(&section.entries, "parking_lot");
    }

    #[tokio::test]
    async fn networking_has_expected_data() {
        let json_content_parser = setup_json_content_parser().await;
        let section =
            json_content_parser.get_crates_with_sub(&CategoriesWithSubCategories::Networking);
        test_section(&section.entries, "tokio");
    }

    #[tokio::test]
    async fn databases_has_expected_data() {
        let json_content_parser = setup_json_content_parser().await;
        let section =
            json_content_parser.get_crates_with_sub(&CategoriesWithSubCategories::Databases);
        test_section(&section.entries, "sqlx");
    }

    #[tokio::test]
    async fn cli_has_expected_data() {
        let json_content_parser = setup_json_content_parser().await;
        let section = json_content_parser.get_crates_with_sub(&CategoriesWithSubCategories::Clis);
        test_section(&section.entries, "clap");
    }

    #[tokio::test]
    async fn graphics_has_expected_data() {
        let json_content_parser = setup_json_content_parser().await;
        let section =
            json_content_parser.get_crates_with_sub(&CategoriesWithSubCategories::Graphics);
        test_section(&section.entries, "gtk4");
    }
}
