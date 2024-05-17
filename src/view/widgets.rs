use std::default;

use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use ratatui::{
    prelude::*,
    style::Style,
    symbols::border,
    widgets::{block::*, *},
};

pub struct Footer<'a> {
    hints: Vec<Span<'a>>,
    version: &'a str,
}

#[derive(Default, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum ItemListStatus {
    Selected,
    #[default]
    Unselected,
}

#[derive(Default, Clone)]
pub struct CrateItemList {
    pub name: String,
    pub description: String,
    pub docs: String,
    pub status: ItemListStatus,
}

#[derive(Clone, Default)]
pub struct DependenciesListWidget {
    pub dependencies: Vec<String>,
}

impl DependenciesListWidget {
    pub fn new(dependencies: Vec<String>) -> Self {
        Self { dependencies }
    }
}

impl StatefulWidget for DependenciesListWidget {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let items: Vec<Line<'_>> = self
            .dependencies
            .iter()
            .map(|dep| Line::from(vec![dep.into(), " ✓ ".blue()]))
            .collect();

        let list = List::new(items)
            .block(
                Block::bordered()
                    .padding(Padding::uniform(2))
                    .title("Dependencies to add"),
            )
            .highlight_style(Style::default().blue())
            .highlight_symbol("* ")
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, state);
    }
}

impl CrateItemList {
    pub fn new(name: String, description: String, docs: String, status: ItemListStatus) -> Self {
        Self {
            name,
            description,
            docs,
            status,
        }
    }
}

impl Into<Vec<CrateItemList>> for crate::backend::Table {
    fn into(self) -> Vec<CrateItemList> {
        let mut items: Vec<CrateItemList> = vec![];

        self.entries.iter().for_each(|entr| {
            entr.crates.iter().for_each(|cr| {
                items.push(CrateItemList::new(
                    cr.name.to_owned(),
                    cr.description.to_owned(),
                    cr.docs.to_owned(),
                    ItemListStatus::default(),
                ))
            })
        });

        items
    }
}

#[derive(Default, Clone)]
pub struct CratesListWidget {
    pub crates: Vec<CrateItemList>,
}

impl From<Vec<crate::backend::Crates>> for CratesListWidget {
    fn from(value: Vec<crate::backend::Crates>) -> Self {
        Self {
            crates: value
                .iter()
                .map(|cra| {
                    CrateItemList::new(
                        cra.name.to_owned(),
                        cra.description.to_owned(),
                        cra.docs.to_owned(),
                        ItemListStatus::Unselected,
                    )
                })
                .collect(),
        }
    }
}

impl StatefulWidget for CratesListWidget {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let list = List::new(
            self.crates
                .iter()
                .map(|crate_item| {
                    let is_selected = match crate_item.status {
                        ItemListStatus::Selected => "✓",
                        ItemListStatus::Unselected => "☐",
                    };
                    format!(
                        "{}, {}, {}",
                        crate_item.name, crate_item.description, is_selected
                    )
                })
                .collect::<Vec<String>>(),
        )
        .block(Block::default().padding(Padding::uniform(2)))
        .highlight_style(Style::default().blue())
        .highlight_symbol(">> ")
        .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, state);
    }
}

impl CratesListWidget {
    pub fn new(crates: &Vec<CrateItemList>) -> Self {
        Self {
            crates: crates.to_vec(),
        }
    }
}

impl<'a> Widget for Footer<'a> {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let text = Title::from(Line::from(self.hints));

        Block::default()
            .title(text)
            .title(Title::from(self.version).alignment(Alignment::Right))
            .borders(Borders::BOTTOM)
            .border_set(border::ROUNDED)
            .render(area, buf);
    }
}

impl<'a> Footer<'a> {
    pub fn new(hints: Vec<Span<'a>>, version: &'a str) -> Self {
        Self { hints, version }
    }
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum CategoriesTabs {
    #[strum(to_string = "common")]
    #[default]
    Common,
    #[strum(to_string = "math-scientific")]
    Math,

    #[strum(to_string = "ffi")]
    FFI,

    #[strum(to_string = "cryptography")]
    Cryptography,

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

impl CategoriesTabs {
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let previous_index = current_index.saturating_add(1);
        Self::from_repr(previous_index).unwrap_or(Self::from_repr(0).unwrap())
    }

    pub fn previous(self) -> Self {
        let current_index = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }
}

impl StatefulWidget for CategoriesTabs {
    type State = ListState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let categories: Vec<String> = CategoriesTabs::iter()
            .map(|category| format!("{category}"))
            .collect();

        let list = List::new(categories)
            .style(Style::default().white())
            .highlight_style(Style::default().blue())
            .highlight_symbol(">> ")
            .highlight_style(Style::default().yellow());

        StatefulWidget::render(list, area, buf, state);
    }
}
