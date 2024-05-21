pub mod content_parser;

#[cfg(test)]

mod test {
    use crate::{
        backend::{Crates, Table, TableEntry},
        content_parser::content_parser::ContentParser,
    };

    #[test]
    fn general_table_has_expected_data() {
        //I dont know how to test this xd
        todo!()
    }
}
