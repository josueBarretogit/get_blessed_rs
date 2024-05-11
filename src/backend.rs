use fake::Dummy;

pub mod backend;

#[derive(Default, Debug, Clone, Dummy)]
pub struct Crates {
    pub name: String,
    pub description: String,
    pub docs: String,
}

#[derive(Default, Debug, Clone, Dummy)]
pub struct TableEntry {
    pub use_case: String,
    pub crates: Vec<Crates>,
}

#[derive(Default, Debug, Clone, Dummy)]
pub struct Table {
    pub entries: Vec<TableEntry>,
}
