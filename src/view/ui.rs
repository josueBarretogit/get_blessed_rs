use std::io;

use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

use ratatui::{
    prelude::*,
    style::Style,
    symbols::border,
    widgets::{block::*, *},
};

use strum::IntoEnumIterator;

use crate::tui::tui::Tui;

use super::widgets::*;

#[derive(Default)]
pub struct AppView {
    pub crates_list: CratesList,
    pub current_tab_category: CategoriesTabs,
    pub exit: bool,
}

#[derive(Default)]
pub struct CratesList {
    crates: CratesListWidget,
    state: ListState,
}

pub enum Screen {
    Selecting,
    Reviewing,
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
        Widget::render(self.current_tab_category, area, buf);
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

        self.render_crates_list(layout[0], buf);

        Block::bordered()
            .title("dependencies list")
            .border_set(border::ROUNDED)
            .render(layout[1], buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Footer::new(
            vec![
                " Next ".into(),
                "<Tab>".blue().bold(),
                " Previous ".into(),
                "<Shift + Tab>".blue().bold(),
                " Check docs ".into(),
                "<D>".blue().bold(),
                " Quit ".into(),
                "<Q> ".blue().bold(),
            ],
            "V0.2.0",
        )
        .render(area, buf);
    }

    fn render_crates_list(&mut self, area: Rect, buf: &mut Buffer) {
        match self.current_tab_category {
            CategoriesTabs::Graphics => StatefulWidget::render(
                CratesListWidget::new(vec![CrateItemList::new(
                    "cli".to_owned(),
                    "clap".to_owned(),
                    "a cool cli arg parser".to_owned(),
                    "link to docs".to_owned(),
                )]),
                area,
                buf,
                &mut self.crates_list.state,
            ),
            CategoriesTabs::Concurrency => StatefulWidget::render(
                CratesListWidget::new(vec![
                    CrateItemList::new(
                        "rayon concurrency".to_owned(),
                        "another thing concurrency".to_owned(),
                        "multithreading concurrency".to_owned(),
                        "link to docs concurrency".to_owned(),
                    ),
                    CrateItemList::new(
                        "rayon concurrency 2".to_owned(),
                        "another thing concurrency".to_owned(),
                        "multithreading concurrency".to_owned(),
                        "link to docs concurrency".to_owned(),
                    ),
                ]),
                area,
                buf,
                &mut self.crates_list.state,
            ),
            CategoriesTabs::Clis => StatefulWidget::render(
                CratesListWidget::new(vec![CrateItemList::new(
                    "cli tool".to_owned(),
                    "cli".to_owned(),
                    "multithreading with cli".to_owned(),
                    "link to docs cli".to_owned(),
                )]),
                area,
                buf,
                &mut self.crates_list.state,
            ),
        };
    }

    fn select_first(&mut self) {
        self.crates_list.state.select(Some(0));
    }
}
