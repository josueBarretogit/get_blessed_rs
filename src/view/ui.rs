use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::{io, os::windows::raw, sync::Arc};

use ratatui::{
    prelude::*,
    style::Style,
    symbols::border,
    widgets::{block::*, *},
};

use crate::{backend::backend::get_crates, tui::tui::Tui};

use super::widgets::{DependenciesListWidget, *};

#[derive(Default)]
pub struct AppView {
    pub dependencies_list: DependenciesList,
    pub crates_list: CratesList,
    pub current_tab_category: CategoriesTabs,
    pub exit: bool,
}

#[derive(Default)]
pub struct CratesList {
    crates_widget_list: CratesListWidget,
    state: ListState,
}

#[derive(Default, Clone)]
pub struct DependenciesList {
    dependencies_widget: DependenciesListWidget,
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
            KeyCode::Down => self.scroll_down(),
            KeyCode::Up => self.scroll_up(),
            KeyCode::Enter => self.toggle_dependencies(),
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
            .title("Crates")
            .title_alignment(Alignment::Left)
            .title_bottom(Line::from("Check docs").right_aligned())
            .title_bottom(Line::from("<D>".blue()).right_aligned())
            .title_bottom(Line::from("Select ").right_aligned())
            .title_bottom(Line::from("<Space>".blue()).right_aligned())
            .border_set(border::ROUNDED)
            .render(layout[0], buf);

        self.render_crates_list(layout[0], buf);

        Block::bordered()
            .title("dependencies list")
            .border_set(border::ROUNDED)
            .render(layout[1], buf);

        self.render_dependencies_list(layout[1], buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Footer::new(
            vec![
                " Next ".into(),
                "<Tab>,".blue().bold(),
                " Previous ".into(),
                "<Shift + Tab>,".blue().bold(),
                " Quit ".into(),
                "<Q>".blue().bold(),
            ],
            "V0.2.0",
        )
        .render(area, buf);
    }

    fn render_crates_list(&mut self, area: Rect, buf: &mut Buffer) {
        match self.current_tab_category {
            CategoriesTabs::Graphics => {
                let crates = get_crates("Concurrency".to_owned());

                self.crates_list.crates_widget_list = CratesListWidget::from(crates.clone());
                StatefulWidget::render(
                    CratesListWidget::from(crates),
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesTabs::Concurrency => {
                let crates = get_crates("Concurrency".to_owned());

                self.crates_list.crates_widget_list = CratesListWidget::from(crates.clone());
                StatefulWidget::render(
                    CratesListWidget::from(crates),
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesTabs::Clis => {
                let crates = get_crates("Clis".to_owned());

                self.crates_list.crates_widget_list = CratesListWidget::from(crates.clone());

                StatefulWidget::render(
                    CratesListWidget::from(crates),
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
        };
    }

    fn scroll_down(&mut self) {
        let next_index = match self.crates_list.state.selected() {
            Some(index) => {
                if index == 0 {
                    1
                } else if index >= self.crates_list.crates_widget_list.crates.len() {
                    0
                } else {
                    index + 1
                }
            }
            None => self.crates_list.state.selected().unwrap_or(0),
        };

        self.crates_list.state.select(Some(next_index));
    }

    fn scroll_up(&mut self) {
        let next_index = match self.crates_list.state.selected() {
            Some(index) => {
                if index == 0 {
                    self.crates_list.crates_widget_list.crates.len()
                } else {
                    index - 1
                }
            }
            None => self.crates_list.state.selected().unwrap_or(0),
        };
        self.crates_list.state.select(Some(next_index));
    }

    fn render_dependencies_list(&mut self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(
            DependenciesListWidget::new(vec![]),
            area,
            buf,
            &mut self.dependencies_list.state,
        );
    }

    fn unselect_all_crates(&mut self) {}

    fn select_all_crates(&mut self) {}

    fn toggle_dependencies(&mut self) {
        self.crates_list
            .crates_widget_list
            .crates
            .iter_mut()
            .for_each(|crate_item| crate_item.description = "changed".into());
    }
}
