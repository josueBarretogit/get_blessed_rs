use strum::{Display, EnumIter, FromRepr};

use crate::view::widgets::{CrateItemList, FeatureItemList, ItemListStatus};

pub mod backend;

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Crates {
    pub name: String,
    pub description: String,
    pub features: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct TableEntry {
    pub use_case: String,
    pub crates: Vec<Crates>,
}

#[derive(Default, Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Table {
    pub entries: Vec<TableEntry>,
}

impl From<crate::backend::Table> for Vec<CrateItemList> {
    fn from(val: crate::backend::Table) -> Self {
        let mut items: Vec<CrateItemList> = vec![];

        for entr in val.entries {
            for krate in entr.crates {
                items.push(CrateItemList::new(
                    krate.name.clone(),
                    krate.description.clone(),
                    ItemListStatus::default(),
                    krate.features.clone().map(|features| {
                        features
                            .iter()
                            .map(|feat| FeatureItemList::new(feat.clone()))
                            .collect()
                    }),
                ));
            }
        }

        items
    }
}

#[derive(Debug, Clone, FromRepr, Display, EnumIter)]
pub enum Categories {
    #[strum(to_string = "math-scientific")]
    Math,

    #[strum(to_string = "ffi")]
    FFI,

    #[strum(to_string = "cryptography")]
    Cryptography,
}

#[derive(Debug, Clone, FromRepr, Display, EnumIter, PartialEq, Eq, PartialOrd, Ord)]
pub enum CategoriesWithSubCategories {
    #[strum(to_string = "common")]
    Common,

    #[strum(to_string = "concurrency")]
    Concurrency,

    #[strum(to_string = "networking")]
    Networking,

    #[strum(to_string = "databases")]
    Databases,

    #[strum(to_string = "cli-tools")]
    Clis,

    #[strum(to_string = "graphics")]
    Graphics,
}
