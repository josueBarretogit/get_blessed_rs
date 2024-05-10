use color_eyre::owo_colors::OwoColorize;
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
    pub use_case: String,
    pub name: String,
    pub description: String,
    pub docs: String,
}

impl CrateItemList {
    pub fn new(use_case: String, name: String, description: String, docs: String) -> Self {
        Self {
            use_case,
            name,
            description,
            docs,
        }
    }
}

impl Into<ListItem<'_>> for CrateItemList {
    fn into(self) -> ListItem<'static> {
        let line = Line::styled(
            format!(
                "{} {} {} {}",
                self.name, self.use_case, self.description, self.docs
            ),
            Style::default().bold(),
        );
        ListItem::new(line)
    }
}

#[derive(Default)]
pub struct CratesListWidget {
    crates: Vec<CrateItemList>,
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
            .highlight_symbol(">>")
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

        value.entries.iter().for_each(|entrie| {
            crates.push(CrateItemList::from(entrie));
        });

        Self { crates: crates }
    }
}

impl From<&crate::backend::TableEntry> for CrateItemList {
    fn from(value: &crate::backend::TableEntry) -> Self {
        Self {
            use_case: value.use_case.clone(),
            name: String::default(),
            description: String::default(),
            docs: String::default(),
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

    pub fn render_with_state(self, area: Rect, buf: &mut Buffer, state: &mut ListState)
    where
        Self: Sized,
    {
        match self {
            Self::Clis => self.render_clis_section(area, buf, state),
            Self::Graphics => self.render_graphics_section(area, buf, state),
            Self::Concurrency => self.render_concurrency_section(area, buf, state),
        }
    }

    fn render_clis_section(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let cli_items = [
            ListItem::new("item 1 clis"),
            ListItem::new("item 1 Graphics"),
            ListItem::new("item 1"),
        ];

        let list = List::new(cli_items)
            .block(
                Block::default()
                    .title("List")
                    .borders(Borders::ALL)
                    .padding(Padding::uniform(2)),
            )
            .highlight_style(Style::default().blue())
            .highlight_symbol(">>")
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, state);
    }

    fn render_graphics_section(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let cli_items = [
            ListItem::new("item 1 Graphics"),
            ListItem::new("item 1 Graphics"),
            ListItem::new("item 1"),
        ];

        let list = List::new(cli_items)
            .block(
                Block::default()
                    .title("List")
                    .borders(Borders::ALL)
                    .padding(Padding::uniform(2)),
            )
            .highlight_style(Style::default().blue())
            .highlight_symbol(">>")
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, state);
    }

    fn render_concurrency_section(self, area: Rect, buf: &mut Buffer, state: &mut ListState) {
        let cli_items = [
            ListItem::new("item 1"),
            ListItem::new("item 1"),
            ListItem::new("item 1"),
        ];

        let list = List::new(cli_items)
            .block(
                Block::default()
                    .title("List")
                    .borders(Borders::ALL)
                    .padding(Padding::uniform(2)),
            )
            .highlight_style(Style::default().blue())
            .highlight_symbol(">>")
            .direction(ListDirection::TopToBottom);

        StatefulWidget::render(list, area, buf, state);
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
