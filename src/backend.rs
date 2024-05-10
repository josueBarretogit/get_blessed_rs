pub mod backend;

#[derive(Default, Debug)]
pub struct Crates {
    pub name: String,
    pub description: String,
    pub docs: String,
}

#[derive(Default, Debug)]
pub struct TableEntry {
    pub use_case: String,
    pub crates: Vec<Crates>,
}

#[derive(Default, Debug)]
pub struct Table {
    pub entries: Vec<TableEntry>,
}
