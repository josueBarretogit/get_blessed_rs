use std::{default, io};

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
    current_tab_category: CategoriesTabs,
    exit: bool,
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
        Self::from_repr(previous_index).unwrap_or(self)
    }

    pub fn previos(self) -> Self {
        let current_index = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    fn render_clis_section(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("clis section").render(area, buf)
    }

    fn render_graphics_section(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Graphics section").render(area, buf)
    }

    fn render_concurrency_section(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Concurrency section").render(area, buf)
    }
}

impl Widget for CategoriesTabs {
    fn render(self, area: Rect, buf: &mut Buffer)
    where
        Self: Sized,
    {
        match self {
            Self::Clis => self.render_clis_section(area, buf),
            Self::Graphics => self.render_graphics_section(area, buf),
            Self::Concurrency => self.render_concurrency_section(area, buf),
        }
    }
}

pub enum Screen {
    Selecting,
    Reviewing,
}

impl Widget for &AppView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(2),
                Constraint::Length(3),
                Constraint::Fill(1),
                Constraint::Length(2),
            ])
            .split(area);

        self.render_main_title(main_layout[0], buf);

        self.render_tabs(main_layout[1], buf);

        self.render_main_section(main_layout[2], buf);

        self.render_footer(main_layout[3], buf)
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

    fn render_frame(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
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
        self.current_tab_category = self.current_tab_category.previos();
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

    fn render_main_section(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(format!(
            " here goes the most popular crates for: {}",
            self.current_tab_category
        ))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_set(border::ROUNDED),
        )
        .render(area, buf);
    }

    fn render_main_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(" Get blessed.rs ")
            .block(
                Block::default()
                    .borders(Borders::BOTTOM)
                    .border_set(border::ROUNDED),
            )
            .centered()
            .render(area, buf)
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

        Block::default().title(text).render(area, buf)
    }
}
