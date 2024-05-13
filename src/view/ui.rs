use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::io;

use ratatui::{
    prelude::*,
    style::Style,
    symbols::border,
    widgets::{block::*, *},
};

use crate::{
    backend::backend::get_crates,
    tui::tui::Tui,
    utils::{toggle_dependencies_all, toggle_one_dependency, toggle_status_all},
};

use super::widgets::{DependenciesListWidget, *};

#[derive(Default)]
pub struct AppView {
    pub dependencies_list: DependenciesList,
    pub crates_list: CratesList,
    pub category_tabs: CategoriesTabs,
    pub cli_crates: Vec<CrateItemList>,
    pub graphics_crates: Vec<CrateItemList>,
    pub dependencies_added: Vec<String>,
    pub exit: bool,
}

#[derive(Default)]
pub struct CratesList {
    crates_widget_list: CratesListWidget,
    state: ListState,
}

impl CratesList {
    pub fn new(crates_widget_list: CratesListWidget, state: ListState) -> Self {
        Self {
            crates_widget_list,
            state,
        }
    }
}

#[derive(Default, Clone)]
pub struct DependenciesList {
    dependencies_widget: DependenciesListWidget,
    state: ListState,
}

impl DependenciesList {
    pub fn new(dependencies_widget: DependenciesListWidget, state: ListState) -> Self {
        Self {
            dependencies_widget,
            state,
        }
    }
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

    pub fn new() -> Self {
        let cli_crates_from_page = get_crates("Clis".into());
        let graphics_crates_from_page = get_crates("graphics".into());

        let mut cli_crates: Vec<CrateItemList> = vec![];
        let mut graphics_crates: Vec<CrateItemList> = vec![];

        cli_crates_from_page.entries.iter().for_each(|entr| {
            entr.crates.iter().for_each(|cr| {
                cli_crates.push(CrateItemList::new(
                    cr.name.to_owned(),
                    cr.description.to_owned(),
                    cr.docs.to_owned(),
                    ItemListStatus::default(),
                ))
            })
        });

        graphics_crates_from_page.entries.iter().for_each(|entr| {
            entr.crates.iter().for_each(|cr| {
                graphics_crates.push(CrateItemList::new(
                    cr.name.to_owned(),
                    cr.description.to_owned(),
                    cr.docs.to_owned(),
                    ItemListStatus::default(),
                ))
            })
        });

        Self {
            dependencies_list: DependenciesList::default(),
            crates_list: CratesList::default(),
            category_tabs: CategoriesTabs::default(),
            cli_crates,
            graphics_crates,
            dependencies_added: Vec::new(),
            exit: false,
        }
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
            KeyCode::Enter => self.toggle_select_all_dependencies(),
            KeyCode::Char('a') => self.toggle_select_dependencie(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }

    pub fn next_tab(&mut self) {
        self.category_tabs = self.category_tabs.next();
    }

    pub fn previos_tab(&mut self) {
        self.category_tabs = self.category_tabs.previous();
    }

    fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        Widget::render(self.category_tabs, area, buf);
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
            .title_bottom(Line::from(" Toggle select ").right_aligned())
            .title_bottom(Line::from("<a>".blue()).right_aligned())
            .title_bottom(Line::from(" Toggle select all ").right_aligned())
            .title_bottom(Line::from("<Enter>".blue()).right_aligned())
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
        match self.category_tabs {
            CategoriesTabs::Graphics => {
                self.crates_list.crates_widget_list = CratesListWidget::new(&self.graphics_crates);
                StatefulWidget::render(
                    self.crates_list.crates_widget_list.clone(),
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
                self.crates_list.crates_widget_list = CratesListWidget::new(&self.cli_crates);

                StatefulWidget::render(
                    self.crates_list.crates_widget_list.clone(),
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
        };
    }

    fn render_dependencies_list(&mut self, area: Rect, buf: &mut Buffer) {
        StatefulWidget::render(
            DependenciesListWidget::new(self.dependencies_added.clone()),
            area,
            buf,
            &mut self.dependencies_list.state,
        );
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

    fn toggle_select_all_dependencies(&mut self) {
        match self.category_tabs {
            CategoriesTabs::Clis => {
                toggle_status_all(&mut self.cli_crates);
                toggle_dependencies_all(&self.cli_crates, &mut self.dependencies_added);
            }
            CategoriesTabs::Graphics => {
                toggle_status_all(&mut self.graphics_crates);
                toggle_dependencies_all(&self.graphics_crates, &mut self.dependencies_added);
            }
            _ => unimplemented!(),
        }
    }

    fn toggle_select_dependencie(&mut self) {
        if let Some(index_crate_selected) = self.crates_list.state.selected() {
            match self.category_tabs {
                CategoriesTabs::Clis => toggle_one_dependency(
                    &mut self.cli_crates[index_crate_selected],
                    &mut self.dependencies_added,
                ),
                CategoriesTabs::Graphics => toggle_one_dependency(
                    &mut self.graphics_crates[index_crate_selected],
                    &mut self.dependencies_added,
                ),
                _ => unimplemented!(),
            }
        }
    }
}
