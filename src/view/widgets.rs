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

#[derive(Default)]
pub struct CrateItemList {
    pub name: String,
    pub description: String,
    pub docs: String,
}

impl CrateItemList {
    pub fn new(name: String, description: String, docs: String) -> Self {
        Self {
            name,
            description,
            docs,
        }
    }
}

impl Into<ListItem<'_>> for CrateItemList {
    fn into(self) -> ListItem<'static> {
        let line = Line::styled(
            format!("{} {} {}", self.name, self.description, self.docs),
            Style::default().bold(),
        );
        ListItem::new(line)
    }
}

#[derive(Default)]
pub struct CratesListWidget {
    pub crates: Vec<CrateItemList>,
}

impl StatefulWidget for CratesListWidget {
    type State = ListState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let list = List::new(self.crates)
            .block(
                Block::default()
                    .title("Crates")
                    .borders(Borders::ALL)
                    .padding(Padding::uniform(2)),
            )
            .highlight_style(Style::default().blue())
            .highlight_symbol(">> ")
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, state);
    }
}

impl CratesListWidget {
    pub fn new(crates: Vec<CrateItemList>) -> Self {
        Self { crates }
    }
}

impl From<crate::backend::Table> for CratesListWidget {
    fn from(value: crate::backend::Table) -> Self {
        let mut crates: Vec<CrateItemList> = vec![];

        value.entries.iter().for_each(|entry| {
            entry
                .crates
                .iter()
                .for_each(|cr| crates.push(CrateItemList::from(cr)));
        });

        Self { crates }
    }
}

impl From<&crate::backend::Crates> for CrateItemList {
    fn from(value: &crate::backend::Crates) -> Self {
        Self {
            name: value.name.to_owned(),
            description: value.description.to_owned(),
            docs: value.docs.to_owned(),
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
    #[default]
    Graphics,
    Clis,
    Concurrency,
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

impl Widget for CategoriesTabs {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        let block_tabs = Block::default()
            .title("Categories")
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        let categories: Vec<String> = CategoriesTabs::iter()
            .map(|category| format!(" {category} "))
            .collect();

        Tabs::new(categories)
            .block(block_tabs)
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .select(self as usize)
            .divider(symbols::DOT)
            .render(area, buf)
    }
}
