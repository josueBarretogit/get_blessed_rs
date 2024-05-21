use scraper::{selectable::Selectable, Html, Selector};

use crate::{
    backend::{Categories, CategoriesWithSubCategories, Crates, Table, TableEntry},
    scraper::scraper::scrape_site,
};

pub struct ContentParser {
    content: Html,
}

impl ContentParser {
    pub async fn new() -> Self {
        let page_content = scrape_site().await.unwrap();

        let html_content = Html::parse_document(&page_content);

        Self {
            content: html_content,
        }
    }

    //Todo! I think hashtables would be useful in this case rather than vectors
    pub fn get_general_crates(&self) -> Table {
        let general_table : Table = Table {
            entries: [
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "rand".into(),
                            description: "De facto standard random number generation library split out from the standard library,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "time".into(),
                            description: "A smaller, simpler library. Preferrable if covers your needs, but it's quite limited in what it provides.,".into(),
                            features : Some(vec!["macros".into(), "formatting".into(), "parsing".into()])
                        },
                        Crates {
                            name: "chrono".into(),
                            description: "The most comprehensive and full-featured datetime library, but more complex because of it.,".into(),
                            features: Some(vec!["serde".into()]),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "serde".into(),
                            description: "De facto standard serialization library. Use in conjunction with sub-crates like serde_json for the specific format that you are using.,".into(),
                            features : Some(vec!["derive".into()])
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "regex".into(),
                            description: "De facto standard regex library. Very fast, but does not support fancier features such as backtracking.,".into(),
                            features : None
                        },
                        Crates {
                            name: "fancy-regex".into(),
                            description: "Use if need features such as backtracking which regex doesn't support,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "uuid".into(),
                            description: "Implements generating and parsing UUIDs and a number of utility functions,".into(),
                            features : Some(vec!["v4".into(), "serde".into()])
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "tempfile".into(),
                            description: "Supports both temporary files and temporary directories,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "flate2".into(),
                            description: "Uses a pure-Rust implementation by default. Use feature flags to opt in to system zlib.,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "indexmap".into(),
                            description: "A HashMap that seperately keeps track of insertion order and allows you to efficiently iterate over its elements in that order,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "arrayvec".into(),
                            description: "Arrays that are ONLY stack-allocated with fixed capacity,".into(),
                            features : Some(vec!["serde".into()])
                        },
                        Crates {
                            name: "smallvec".into(),
                            description: "Arrays that are stack-allocated with fallback to the heap if the fixed stack capacity is exceeded,".into(),
                            features : None,
                        },
                        Crates {
                            name: "tinyvec".into(),
                            description: "Stack allocated arrays in 100% safe Rust code but requires items to implement the Default trait.,".into(),
                            features : None
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "reqwest".into(),
                            description: "Full-fat HTTP client. Can be used in both synchronous and asynchronous code. Requires tokio runtime.,".into(),
                            features : Some(vec!["json".into()])
                        },
                        Crates {
                            name: "ureq".into(),
                            description: "Minimal synchronous HTTP client focussed on simplicity and minimising dependencies.,".into(),
                            features : Some(vec!["json".into(), "charset".into()])
                        },
                    ].to_vec(),
                },
            ].to_vec(),
        };
        general_table
    }

    pub fn get_crates(&self, category: Categories) -> Table {
        let section_to_get = format!("#section-{}", category.to_string());

        let entry_selector =
            Selector::parse(format!("{} > table > tbody > tr > td", section_to_get).as_str())
                .unwrap();

        //Each p contains the name of the crate
        let description_selector = Selector::parse("p").unwrap();

        let crates_section = self.content.select(&entry_selector);

        let mut entries: Vec<TableEntry> = Vec::new();

        crates_section.for_each(|entr| {
            let crates_in_entry = entr.select(&description_selector);
            let mut crates: Vec<Crates> = Vec::new();
            crates_in_entry.for_each(|cr| {
                let text = cr
                    .text()
                    .map(|text| text.trim().to_string())
                    .filter_map(|te| {
                        if !te.is_empty()
                            && !te.contains("[docs]")
                            && !te.contains("For more algorithms, see")
                            && !te.contains("Rust Crypto Password Hashes")
                        {
                            return Some(format!("{},", te));
                        } else {
                            return None;
                        }
                    })
                    .collect::<String>();

                let data: Vec<&str> = text.splitn(2, ',').collect();

                let name = data[0].to_string();
                let description = if data[1].is_empty() {
                    "no description".to_string()
                } else {
                    data[1].to_string()
                };
                let docs = format!("https://docs.rs/{}/latest/{}/", name, name);

                if !name.contains(".")
                    && !name.contains("For more formats")
                    && !name.contains("Rust Crypto Signatures")
                    && !name.contains("Rust Crypto AEADs")
                    && !name.contains("Rust Crypto Hashes")
                {
                    crates.push(Crates {
                        name,
                        description,
                        features: None,
                    });
                }
            });
            entries.push(TableEntry {
                use_case: "".into(),
                crates,
            });
        });

        let entries: Vec<TableEntry> = entries
            .iter()
            .filter_map(|entry| {
                if entry.crates.iter().len() != 0 {
                    Some(entry.to_owned())
                } else {
                    None
                }
            })
            .collect();

        Table { entries }
    }

    pub fn get_crates_with_sub(&self, category: CategoriesWithSubCategories) -> Table {
        let section_name = format!("#section-{}", category.to_string());

        let selector =
            Selector::parse(format!("{} > section > table", section_name).as_str()).unwrap();

        let entry_selector = Selector::parse("tbody > tr  td > p").unwrap();

        let name_selector = Selector::parse("p > b > a").unwrap();

        let mut crate_section = self.content.select(&selector);

        if category == CategoriesWithSubCategories::Common {
            crate_section.next();
        }

        let mut entries: Vec<TableEntry> = Vec::new();

        crate_section.for_each(|tbl| {
            let contents = tbl.select(&entry_selector);

            let mut crates: Vec<Crates> = Vec::new();

            contents.for_each(|entry| {
                let crate_name = match entry.select(&name_selector).next() {
                    Some(elemen) => elemen.inner_html(),
                    None => "name not found".into(),
                };

                let text = entry
                    .text()
                    .filter(|text| *text != crate_name && !text.contains("[docs]"))
                    .collect::<String>();

                let description = if text.trim().is_empty() {
                    "no description ".into()
                } else {
                    text.trim().to_string()
                };

                if !crate_name.contains("name not found") {
                    crates.push(Crates {
                        name: crate_name,
                        description,
                        features: None,
                    });
                }
            });

            entries.push(TableEntry {
                use_case: "".into(),
                crates,
            })
        });

        Table { entries }
    }
}
