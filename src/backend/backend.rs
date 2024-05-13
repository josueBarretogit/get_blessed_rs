use fake::faker::lorem::en::{Word, Words};
use fake::faker::name::raw::*;
use fake::locales::*;
use fake::{Fake, Faker};

use super::{Crates, Table, TableEntry};

pub fn get_crates(category: String) -> Table {
    let entries1 = vec![
        TableEntry {
            use_case: Faker.fake(),
            crates: vec![
                Crates {
                    name: Name(EN).fake(),
                    description: Name(EN).fake(),
                    docs: Word().fake(),
                },
                Crates {
                    name: Name(EN).fake(),
                    description: Name(EN).fake(),
                    docs: Word().fake(),
                },
            ],
        },
        TableEntry {
            use_case: "a".into(),
            crates: vec![
                Crates {
                    name: Name(EN).fake(),
                    description: Name(EN).fake(),
                    docs: Word().fake(),
                },
                Crates {
                    name: Name(EN).fake(),
                    description: Name(EN).fake(),
                    docs: Word().fake(),
                },
            ],
        },
    ];
    Table { entries: entries1 }
}
