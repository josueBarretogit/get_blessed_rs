use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use std::{io, usize};

use ratatui::{
    prelude::*,
    style::Style,
    symbols::border,
    widgets::{block::*, *},
};

use crate::{
    backend::{backend::get_crates, Categories},
    tui::tui::Tui,
    utils::{toggle_dependencies_all, toggle_one_dependency, toggle_status_all},
};

use super::widgets::{DependenciesListWidget, *};

#[derive(Default)]
pub struct AppView {
    pub dependencies_to_add_list: DependenciesList,
    pub crates_list: CratesList,
    pub category_tabs: CategoriesTabs,
    pub cli_crates: Vec<CrateItemList>,
    pub graphics_crates: Vec<CrateItemList>,
    pub concurrency_crates: Vec<CrateItemList>,
    pub exit: bool,
    categories_list_state: ListState,

    loggin: Vec<CrateItemList>,
    language: Vec<CrateItemList>,
    system: Vec<CrateItemList>,
    math: Vec<CrateItemList>,
    websockets: Vec<CrateItemList>,
    databasae: Vec<CrateItemList>,
    terminalre: Vec<CrateItemList>,
    grpc: Vec<CrateItemList>,
    utility: Vec<CrateItemList>,
}

#[derive(Default)]
pub struct CratesList {
    crates_widget_list: CratesListWidget,
    state: ListState,
}

#[derive(Default, Clone)]
pub struct DependenciesList {
    dependencies_widget: DependenciesListWidget,
    dependencies_to_add: Vec<String>,
    state: ListState,
}

impl DependenciesList {
    pub fn new(
        dependencies_widget: DependenciesListWidget,
        state: ListState,
        dependencies_to_add: Vec<String>,
    ) -> Self {
        Self {
            dependencies_widget,
            state,
            dependencies_to_add,
        }
    }
}

impl Widget for &mut AppView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer)
    where
        Self: Sized,
    {
        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(24),
                Constraint::Percentage(51),
                Constraint::Percentage(25),
            ])
            .split(area);

        self.render_categories_list(main_layout[0], buf);

        self.render_main_section(main_layout[1], buf);

        self.render_dependencies_list(main_layout[2], buf);
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
        let crates = get_crates(Categories::FFI);

        let cli_crates = get_crates(Categories::Clis);
        let graphics_crates = get_crates(Categories::Graphics);
        let concurrency_crates = get_crates(Categories::Concurrency);
        let loggin = get_crates(Categories::Loggin);
        let language = get_crates(Categories::LanguageExtensions);
        let system = get_crates(Categories::System);
        let math = get_crates(Categories::Math);
        let websockets = get_crates(Categories::WebSockets);
        let databasae = get_crates(Categories::Databases);
        let terminalre = get_crates(Categories::TerminalRendering);
        let grpc = get_crates(Categories::Grpc);
        let utility = get_crates(Categories::Utility);
        let gamedevelopment_crates = get_crates(Categories::GameDevelopment);
        let networking_crates = get_crates(Categories::Networking);

        let mut list_state = ListState::default();

        list_state.select(Some(0));

        Self {
            dependencies_to_add_list: DependenciesList::default(),
            crates_list: CratesList::default(),
            category_tabs: CategoriesTabs::default(),
            cli_crates: cli_crates.into(),
            graphics_crates: graphics_crates.into(),
            concurrency_crates: concurrency_crates.into(),
            loggin: loggin.into(),
            language: language.into(),
            system: system.into(),
            math: math.into(),
            websockets: websockets.into(),
            utility: utility.into(),
            databasae: databasae.into(),
            terminalre: terminalre.into(),
            grpc: grpc.into(),

            categories_list_state: list_state,

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

        self.categories_list_state
            .select(Some(self.category_tabs as usize));
    }

    pub fn previos_tab(&mut self) {
        self.category_tabs = self.category_tabs.previous();

        self.categories_list_state
            .select(Some(self.category_tabs as usize));
    }

    fn render_categories_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block_tabs = Block::default()
            .title(Title::from(Line::from(vec![
                "Next ".into(),
                "<Tab> ".blue(),
                "Previous ".into(),
                "<Shift + Tab>".blue(),
            ])))
            .title_position(Position::Bottom)
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        block_tabs.render(area, buf);

        let margin = area.inner(&Margin {
            horizontal: 5,
            vertical: 3,
        });

        StatefulWidget::render(
            self.category_tabs,
            margin,
            buf,
            &mut self.categories_list_state,
        );
    }

    fn render_main_section(&mut self, area: Rect, buf: &mut Buffer) {
        let instructions = Title::from(Line::from(vec![
            "Check docs".into(),
            "<d>".blue(),
            " Toggle select ".into(),
            "<a>".blue(),
            " Toggle select all ".into(),
            "<Enter>".blue(),
        ]));
        Block::bordered()
            .title("Crates")
            .title(
                instructions
                    .alignment(Alignment::Right)
                    .position(Position::Bottom),
            )
            .border_set(border::ROUNDED)
            .render(area, buf);

        let inner_main_area = area.inner(&Margin {
            horizontal: 10,
            vertical: 3,
        });

        self.render_crates_list(inner_main_area, buf);
    }

    fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Footer::new(
            vec![
                " Next ".into(),
                "<Tab>,".blue().bold(),
                " Previous ".into(),
                "<Shift + Tab>,".blue().bold(),
                " Quit ".into(),
                "<q>".blue().bold(),
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
                self.crates_list.crates_widget_list =
                    CratesListWidget::new(&self.concurrency_crates);
                StatefulWidget::render(
                    self.crates_list.crates_widget_list.clone(),
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
            CategoriesTabs::Loggin
            | CategoriesTabs::FFI
            | CategoriesTabs::Math
            | CategoriesTabs::Http
            | CategoriesTabs::Databases
            | CategoriesTabs::WebSockets
            | CategoriesTabs::Cryptography
            | CategoriesTabs::ErrorHandling
            | CategoriesTabs::TerminalRendering
            | CategoriesTabs::LanguageExtensions
            | CategoriesTabs::Networking
            | CategoriesTabs::Utility
            | CategoriesTabs::System
            | CategoriesTabs::GUI
            | CategoriesTabs::General
            | CategoriesTabs::GameDevelopment
            | CategoriesTabs::Grpc => {
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
            DependenciesListWidget::new(self.dependencies_to_add_list.dependencies_to_add.clone()),
            area,
            buf,
            &mut self.dependencies_to_add_list.state,
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
                toggle_dependencies_all(
                    &self.cli_crates,
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                );
            }
            CategoriesTabs::Graphics => {
                toggle_status_all(&mut self.graphics_crates);
                toggle_dependencies_all(
                    &self.graphics_crates,
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                );
            }
            CategoriesTabs::Concurrency => {
                toggle_status_all(&mut self.concurrency_crates);
                toggle_dependencies_all(
                    &self.concurrency_crates,
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                );
            }

            _ => unimplemented!(),
        }
    }

    fn toggle_select_dependencie(&mut self) {
        if let Some(index_crate_selected) = self.crates_list.state.selected() {
            match self.category_tabs {
                CategoriesTabs::Clis => toggle_one_dependency(
                    &mut self.cli_crates[index_crate_selected],
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                ),
                CategoriesTabs::Graphics => toggle_one_dependency(
                    &mut self.graphics_crates[index_crate_selected],
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                ),
                CategoriesTabs::Concurrency => toggle_one_dependency(
                    &mut self.concurrency_crates[index_crate_selected],
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                ),

                _ => unimplemented!(),
            };
        }
    }
}
