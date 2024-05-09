use std::{default, io};

use color_eyre::owo_colors::OwoColorize;
use crossterm::{event::{self, Event, KeyCode, KeyEvent, KeyEventKind}, style::style};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};
use strum::{Display, EnumIter, FromRepr};

use crate::tui::tui::Tui;

#[derive(Default)]
pub struct AppView {
    current_tab_category : CategoriesTabs,
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
        let title = Title::from(" Get blessed.rs ".bold());

        let instructions = Title::from(Line::from(vec![
            " Next ".into(),
            "<Tab>".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]));


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

        let main_layout = Layout::default()
            .direction(Direction::Vertical)
            .constraints(vec![
                Constraint::Length(3),
                Constraint::Fill(1),
            ]).split(frame.size());

        let block_tabs = Block::default().title("Crates category").borders(Borders::ALL).border_set(border::ROUNDED);

        let categories_container = Tabs::new(vec!["Concurrency", "Graphics", "Clis"])
            .block(block_tabs)
            .style(Style::default().white())
            .highlight_style(Style::default().yellow())
            .select(0)
            .divider(symbols::DOT);

        frame.render_widget(categories_container, main_layout[0]);

        let categories_container = Paragraph::new(" Crates ").block(Block::default().borders(Borders::ALL).border_set(border::ROUNDED));

        frame.render_widget(categories_container, main_layout[1]);


        let categories = ["Concurrency", "Graphics", "Clis"];



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


}


