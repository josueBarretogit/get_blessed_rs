use fake::faker::lorem::en::Words;
use fake::{ Fake, Faker}; 
use fake::faker::name::raw::*;
use fake::locales::*;

use super::{Crates, Table, TableEntry};



pub fn get_crates(category: String) -> Table {
    let entries1 = vec![
        TableEntry {
            use_case: Faker.fake(),
            crates: vec![
                Crates {
                    name: Name(EN).fake(),
                    description: Name(EN).fake(),
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
                    name: "rand".into(),
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
