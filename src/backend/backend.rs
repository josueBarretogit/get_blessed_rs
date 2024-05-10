use super::{Crates, Table, TableEntry};

pub async fn get_crates(category: String) -> Table {
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
                    name: "serde".into(),
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
