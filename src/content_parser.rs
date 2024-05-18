pub mod content_parser;

#[cfg(test)]

mod test {
    use crate::{
        backend::{Crates, Table, TableEntry},
        content_parser::content_parser::ContentParser,
    };

    #[test]
    fn general_table_has_expected_data() {
        let general_table : Table = Table {
            entries: [
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "rand".into(),
                            description: "De facto standard random number generation library split out from the standard library,".into(),
                            docs: "https://docs.rs/rand/latest/rand/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "time".into(),
                            description: "A smaller, simpler library. Preferrable if covers your needs, but it's quite limited in what it provides.,".into(),
                            docs: "https://docs.rs/time/latest/time/".into(),
                        },
                        Crates {
                            name: "chrono".into(),
                            description: "The most comprehensive and full-featured datetime library, but more complex because of it.,".into(),
                            docs: "https://docs.rs/chrono/latest/chrono/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "serde".into(),
                            description: "De facto standard serialization library. Use in conjunction with sub-crates like serde_json for the specific format that you are using.,".into(),
                            docs: "https://docs.rs/serde/latest/serde/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".to_string(),
                    crates: [
                        Crates {
                            name: "regex".into(),
                            description: "De facto standard regex library. Very fast, but does not support fancier features such as backtracking.,".into(),
                            docs: "https://docs.rs/regex/latest/regex/".into(),
                        },
                        Crates {
                            name: "fancy-regex".into(),
                            description: "Use if need features such as backtracking which regex doesn't support,".into(),
                            docs: "https://docs.rs/fancy-regex/latest/fancy-regex/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "uuid".into(),
                            description: "Implements generating and parsing UUIDs and a number of utility functions,".into(),
                            docs: "https://docs.rs/uuid/latest/uuid/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "tempfile".into(),
                            description: "Supports both temporary files and temporary directories,".into(),
                            docs: "https://docs.rs/tempfile/latest/tempfile/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "flate2".into(),
                            description: "Uses a pure-Rust implementation by default. Use feature flags to opt in to system zlib.,".into(),
                            docs: "https://docs.rs/flate2/latest/flate2/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "indexmap".into(),
                            description: "A HashMap that seperately keeps track of insertion order and allows you to efficiently iterate over its elements in that order,".into(),
                            docs: "https://docs.rs/indexmap/latest/indexmap/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "arrayvec".into(),
                            description: "Arrays that are ONLY stack-allocated with fixed capacity,".into(),
                            docs: "https://docs.rs/arrayvec/latest/arrayvec/".into(),
                        },
                        Crates {
                            name: "smallvec".into(),
                            description: "Arrays that are stack-allocated with fallback to the heap if the fixed stack capacity is exceeded,".into(),
                            docs: "https://docs.rs/smallvec/latest/smallvec/".into(),
                        },
                        Crates {
                            name: "tinyvec".into(),
                            description: "Stack allocated arrays in 100% safe Rust code but requires items to implement the Default trait.,".into(),
                            docs: "https://docs.rs/tinyvec/latest/tinyvec/".into(),
                        },
                    ].to_vec(),
                },
                TableEntry {
                    use_case: "".into(),
                    crates: [
                        Crates {
                            name: "reqwest".into(),
                            description: "Full-fat HTTP client. Can be used in both synchronous and asynchronous code. Requires tokio runtime.,".into(),
                            docs: "https://docs.rs/reqwest/latest/reqwest/".into(),
                        },
                        Crates {
                            name: "ureq".into(),
                            description: "Minimal synchronous HTTP client focussed on simplicity and minimising dependencies.,".into(),
                            docs: "https://docs.rs/ureq/latest/ureq/".into(),
                        },
                    ].to_vec(),
                },
            ].to_vec(),
        };

        assert_eq!(general_table, ContentParser::new().get_general_crates())
    }
}
