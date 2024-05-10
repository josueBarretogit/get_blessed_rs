use super::{Crates, Table, TableEntry};

pub fn get_crates(category: String, index: usize) -> Table {
    let entries1 = vec![
        TableEntry {
            use_case: "a".into(),
            crates: vec![
                Crates {
                    name: "rand".into(),
                    description: "de facto standar".into(),
                    docs: "httpp://".into(),
                },
                Crates {
                    name: "time".into(),
                    description: "de facto standar".into(),
                    docs: "httpp://".into(),
                },
            ],
        },
        TableEntry {
            use_case: "a".into(),
            crates: vec![
                Crates {
                    name: index.to_string(),
                    description: "de facto standar".into(),
                    docs: "httpp://".into(),
                },
                Crates {
                    name: "fancy-regex".into(),
                    description: "de facto standar".into(),
                    docs: "httpp://".into(),
                },
            ],
        },
    ];
    Table { entries: entries1 }
}
