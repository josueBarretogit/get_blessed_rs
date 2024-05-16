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

#[derive(Default, Debug, Clone, Dummy)]
pub struct CratesContent {
    pub tables: Vec<Table>,
}

#[derive(Debug, Clone)]
pub enum Categories {
    ErrorHandling,
    Loggin,
    LanguageExtensions,
    System,
    Math,
    FFI,
    Cryptography,
    Networking,
    Http,
    WebSockets,
    Grpc,
    Databases,
    Clis,
    Utility,
    TerminalRendering,
    Concurrency,
    Graphics,
    GameDevelopment,
}
