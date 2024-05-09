use std::{borrow::BorrowMut, io};

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    style::Style,
    symbols::border,
    widgets::{block::*, *},
};
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

use crate::tui::tui::Tui;

#[derive(Default)]
pub struct AppView {
    pub crates_list : StateList,
    pub current_tab_category: CategoriesTabs,
    pub exit: bool,
}

#[derive(Default)]
pub struct CrateItemList {
    pub name : String,
    pub description : String,
    pub docs : String
}


#[derive(Default)]
struct StateList {
    pub state : ListState,
    pub items : Vec<CrateItemList>
}

impl StateList {
    pub fn set_items(&mut self, items : Vec<CrateItemList>) {
        self.items = items;
    }

    fn scroll_up(&mut self) {

    }

    fn scroll_down(&mut self) {

    }
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum CategoriesTabs {
    #[default]
    Graphics,
    Clis,
    Concurrency,
}

pub enum Screen {
    Selecting,
    Reviewing,
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


    fn render_with_state(self, area: Rect, buf: &mut Buffer, state : &mut ListState)
    where
        Self: Sized,
    {
        match self {
            Self::Clis => self.render_clis_section(area, buf, state),
            Self::Graphics => self.render_graphics_section(area, buf, state),
            Self::Concurrency => self.render_concurrency_section(area, buf, state),
        }
    }

    fn render_clis_section(self, area: Rect, buf: &mut Buffer, state : &mut ListState) {

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

    fn render_graphics_section(self, area: Rect, buf: &mut Buffer, state : &mut ListState) {
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

    fn render_concurrency_section(self, area: Rect, buf: &mut Buffer, state : &mut ListState) {
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

    fn block_content(self) -> Block<'static> {
        Block::bordered().border_set(symbols::border::PROPORTIONAL_TALL)
    }
}



impl Widget for &mut AppView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Min(1),
                Constraint::Length(2),
            ])
            .split(area);


        self.render_tabs(main_layout[0], buf);

        self.render_main_section(main_layout[1], buf);

        self.render_footer(main_layout[2], buf)
    }
}

impl AppView {
    pub fn run(&mut self, terminal: &mut Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        self.select_first();
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Tab => self.next_tab(),
            KeyCode::BackTab => self.previos_tab(),
            _ => {}
        }
    }


    fn exit(&mut self) {
        self.exit = true;
    }

    pub fn next_tab(&mut self) {
        self.current_tab_category = self.current_tab_category.next();
    }

    pub fn previos_tab(&mut self) {
        self.current_tab_category = self.current_tab_category.previous();
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let block_tabs = Block::default()
            .title("Categories")
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        let categories: Vec<String> = CategoriesTabs::iter()
            .map(|category| format!(" {category} "))
            .collect();

        let selected_tab_index = self.current_tab_category as usize;

        Tabs::new(categories)
            .block(block_tabs)
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .select(selected_tab_index)
            .divider(symbols::DOT)
            .render(area, buf)
    }

    fn render_main_section(&mut self, area: Rect, buf: &mut Buffer) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![Constraint::Percentage(75), Constraint::Percentage(25)])
            .split(area);

        Block::bordered()
            .title("main content")
            .border_set(border::ROUNDED)
            .render(layout[0], buf);

        self.current_tab_category.render_with_state(layout[0], buf, &mut self.crates_list.state);


        Block::bordered()
            .title("dependencies list")
            .border_set(border::ROUNDED)
            .render(layout[1], buf);
    }


    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        let text = Title::from(Line::from(vec![
            " Next ".into(),
            "<Tab>".blue().bold(),
            " Previous ".into(),
            "<Shift + Tab>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));

        Block::default()
            .title(text)
            .title(Title::from("V0.2.0").alignment(Alignment::Right))
            .borders(Borders::BOTTOM)
            .border_set(border::ROUNDED)
            .render(area, buf);
    }

    fn select_first(&mut self) {
        self.crates_list.state.select(Some(0));
    }
}
