use crate::backend::{Categories, CategoriesWithSubCategories, Crates, Table, TableEntry};
use crate::scraper::scraper::Group;

pub mod content_parser;

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
                        features: None
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
