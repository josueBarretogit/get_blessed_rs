use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, ModifierKeyCode};
use std::{default, io, usize};
use throbber_widgets_tui::ThrobberState;
use tokio::sync::mpsc::UnboundedSender;

use ratatui::{
    prelude::*,
    style::Style,
    symbols::border,
    widgets::{block::*, *},
};

use crate::{
    backend::{backend::get_crates, Categories, CategoriesWithSubCategories},
    content_parser::content_parser::ContentParser,
    dependency_builder::DependenciesBuilder,
    tui::{handler::Action, tui::Tui},
    utils::{centered_rect, toggle_dependencies_all, toggle_one_dependency, toggle_status_all},
};

use super::widgets::{DependenciesListWidget, *};

pub struct AppView {
    pub action_tx: UnboundedSender<Action>,
    pub dependencies_to_add_list: DependenciesList,
    pub crates_list: CratesList,
    pub category_tabs: CategoriesTabs,

    pub is_adding_deps: bool,
    loader_state: throbber_widgets_tui::ThrobberState,

    pub exit: bool,
    categories_list_state: ListState,

    general_crates: Vec<CrateItemList>,
    math_crates: Vec<CrateItemList>,
    ffi_crates: Vec<CrateItemList>,
    cryptography_crates: Vec<CrateItemList>,
    common_crates: Vec<CrateItemList>,
    concurrency_crates: Vec<CrateItemList>,
    networking_crates: Vec<CrateItemList>,
    database_crates: Vec<CrateItemList>,
    clis_crates: Vec<CrateItemList>,
    graphics_crates: Vec<CrateItemList>,
}

#[derive(Default)]
pub enum AppModes {
    #[default]
    AddingDeps,
}

#[derive(Default)]
pub struct CratesList {
    crates_widget_list: CratesListWidget,
    state: ListState,
}

#[derive(Default, Clone)]
pub struct DependenciesList {
    dependencies_to_add: Vec<String>,
    state: ListState,
}

impl DependenciesList {
    pub const fn new(state: ListState, dependencies_to_add: Vec<String>) -> Self {
        Self {
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
                Constraint::Percentage(15),
                Constraint::Percentage(60),
                Constraint::Percentage(25),
            ])
            .split(area);

        self.render_categories_list(main_layout[0], buf);

        self.render_main_section(main_layout[1], buf);

        self.render_dependencies_list(main_layout[2], buf);

        if self.is_adding_deps {
            let popup = Popup::default();
            let center = centered_rect(60, 20, area);
            Clear::default().render(center, buf);
            StatefulWidget::render(popup, center, buf, &mut self.loader_state)
        }
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

    pub async fn new(action_tx: UnboundedSender<Action>) -> Self {
        let page_contents = ContentParser::new().await;

        let mut list_state = ListState::default();

        list_state.select(Some(0));
        let general_crates = page_contents.get_general_crates();

        let math_crates = page_contents.get_crates(Categories::Math);
        let ffi_crates = page_contents.get_crates(Categories::FFI);
        let cryptography_crates = page_contents.get_crates(Categories::Cryptography);

        let common_crates = page_contents.get_crates_with_sub(CategoriesWithSubCategories::Common);
        let concurrency_crates =
            page_contents.get_crates_with_sub(CategoriesWithSubCategories::Concurrency);
        let networking_crates =
            page_contents.get_crates_with_sub(CategoriesWithSubCategories::Networking);
        let database_crates =
            page_contents.get_crates_with_sub(CategoriesWithSubCategories::Databases);
        let clis_crates = page_contents.get_crates_with_sub(CategoriesWithSubCategories::Clis);
        let graphics_crates =
            page_contents.get_crates_with_sub(CategoriesWithSubCategories::Graphics);

        Self {
            action_tx,
            dependencies_to_add_list: DependenciesList::default(),
            crates_list: CratesList::default(),
            category_tabs: CategoriesTabs::default(),
            is_adding_deps: false,
            loader_state: ThrobberState::default(),

            general_crates: general_crates.into(),

            math_crates: math_crates.into(),
            ffi_crates: ffi_crates.into(),

            cryptography_crates: cryptography_crates.into(),

            common_crates: common_crates.into(),
            concurrency_crates: concurrency_crates.into(),
            networking_crates: networking_crates.into(),
            database_crates: database_crates.into(),
            clis_crates: clis_crates.into(),
            graphics_crates: graphics_crates.into(),

            categories_list_state: list_state,

            exit: false,
        }
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
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
            KeyCode::Char('q') | KeyCode::Esc => self.exit(),
            KeyCode::Tab => self.next_tab(),
            KeyCode::BackTab => self.previos_tab(),
            KeyCode::Down | KeyCode::Char('j') => self.scroll_down(),
            KeyCode::Up | KeyCode::Char('k') => self.scroll_up(),
            KeyCode::Char('s') => self.toggle_select_dependencie(),
            KeyCode::Char('a') => self.toggle_select_all_dependencies(),
            KeyCode::Char('d') => self.check_docs(),
            KeyCode::Char('c') => self.check_crates_io(),
            _ => {}
        }
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn next_tab(&mut self) {
        self.crates_list.state.select(Some(0));
        self.category_tabs = self.category_tabs.next();

        self.categories_list_state
            .select(Some(self.category_tabs as usize));
    }

    pub fn previos_tab(&mut self) {
        self.crates_list.state.select(Some(0));
        self.category_tabs = self.category_tabs.previous();

        self.categories_list_state
            .select(Some(self.category_tabs as usize));
    }

    pub fn render_categories_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block_tabs = Block::default()
            .title(Title::from(Line::from(vec![
                "go down ".into(),
                "<Tab> ".blue(),
                "go up ".into(),
                "<Shift + Tab>".blue(),
            ])))
            .title_position(Position::Bottom)
            .borders(Borders::ALL)
            .border_set(border::ROUNDED);

        block_tabs.render(area, buf);

        let margin = area.inner(&Margin {
            horizontal: 1,
            vertical: 3,
        });

        StatefulWidget::render(
            self.category_tabs,
            margin,
            buf,
            &mut self.categories_list_state,
        );
    }

    pub fn render_main_section(&mut self, area: Rect, buf: &mut Buffer) {
        let instructions = Title::from(Line::from(vec![
            "Check docs".into(),
            "<d>".blue(),
            "Check crates.io".into(),
            "<c>".blue(),
            " Toggle select ".into(),
            "<s>".blue(),
            " Toggle select all ".into(),
            "<a>".blue(),
        ]));

        Block::bordered()
            .title("Crate name, description")
            .title(
                instructions
                    .alignment(Alignment::Right)
                    .position(Position::Bottom),
            )
            .border_set(border::ROUNDED)
            .render(area, buf);

        let inner_area_for_list = area.inner(&Margin {
            vertical: 1,
            horizontal: 1,
        });

        self.render_crates_list(inner_area_for_list, buf);
    }

    fn render_crates_list(&mut self, area: Rect, buf: &mut Buffer) {
        match self.category_tabs {
            CategoriesTabs::General => {
                self.crates_list.crates_widget_list = CratesListWidget::new(&self.general_crates);
                StatefulWidget::render(
                    self.crates_list.crates_widget_list.clone(),
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
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
                self.crates_list.crates_widget_list = CratesListWidget::new(&self.clis_crates);

                StatefulWidget::render(
                    self.crates_list.crates_widget_list.clone(),
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesTabs::FFI => {
                self.crates_list.crates_widget_list = CratesListWidget::new(&self.ffi_crates);

                StatefulWidget::render(
                    self.crates_list.crates_widget_list.clone(),
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesTabs::Math => {
                self.crates_list.crates_widget_list = CratesListWidget::new(&self.math_crates);

                StatefulWidget::render(
                    self.crates_list.crates_widget_list.clone(),
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }

            CategoriesTabs::Common => {
                self.crates_list.crates_widget_list = CratesListWidget::new(&self.common_crates);

                StatefulWidget::render(
                    self.crates_list.crates_widget_list.clone(),
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesTabs::Databases => {
                self.crates_list.crates_widget_list = CratesListWidget::new(&self.database_crates);

                StatefulWidget::render(
                    self.crates_list.crates_widget_list.clone(),
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesTabs::Networking => {
                self.crates_list.crates_widget_list =
                    CratesListWidget::new(&self.networking_crates);

                StatefulWidget::render(
                    self.crates_list.crates_widget_list.clone(),
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesTabs::Cryptography => {
                self.crates_list.crates_widget_list =
                    CratesListWidget::new(&self.cryptography_crates);

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

    pub fn scroll_down(&mut self) {
        let next = match self.crates_list.state.selected() {
            Some(index) => {
                if index
                    == self
                        .crates_list
                        .crates_widget_list
                        .crates
                        .iter()
                        .len()
                        .saturating_sub(1)
                {
                    0
                } else {
                    index.saturating_add(1)
                }
            }
            None => self.crates_list.state.selected().unwrap_or(0),
        };

        self.crates_list.state.select(Some(next));
    }

    pub fn scroll_up(&mut self) {
        let next_index = match self.crates_list.state.selected() {
            Some(index) => {
                if index == 0 {
                    self.crates_list
                        .crates_widget_list
                        .crates
                        .len()
                        .saturating_sub(1)
                } else {
                    index.saturating_sub(1)
                }
            }
            None => 1,
        };
        self.crates_list.state.select(Some(next_index));
    }

    pub fn toggle_select_all_dependencies(&mut self) {
        match self.category_tabs {
            CategoriesTabs::Clis => {
                toggle_status_all(&mut self.clis_crates);
                toggle_dependencies_all(
                    &self.clis_crates,
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

            CategoriesTabs::FFI => {
                toggle_status_all(&mut self.ffi_crates);
                toggle_dependencies_all(
                    &self.ffi_crates,
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                );
            }
            CategoriesTabs::Math => {
                toggle_status_all(&mut self.math_crates);
                toggle_dependencies_all(
                    &self.math_crates,
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                );
            }

            CategoriesTabs::Common => {
                toggle_status_all(&mut self.common_crates);
                toggle_dependencies_all(
                    &self.common_crates,
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                );
            }

            CategoriesTabs::General => {
                toggle_status_all(&mut self.general_crates);
                toggle_dependencies_all(
                    &self.general_crates,
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                );
            }

            CategoriesTabs::Databases => {
                toggle_status_all(&mut self.database_crates);
                toggle_dependencies_all(
                    &self.database_crates,
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                );
            }

            CategoriesTabs::Networking => {
                toggle_status_all(&mut self.networking_crates);
                toggle_dependencies_all(
                    &self.networking_crates,
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                );
            }

            CategoriesTabs::Cryptography => {
                toggle_status_all(&mut self.cryptography_crates);
                toggle_dependencies_all(
                    &self.cryptography_crates,
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                );
            }
        }
    }

    pub fn toggle_select_dependencie(&mut self) {
        if let Some(index_crate_selected) = self.crates_list.state.selected() {
            match self.category_tabs {
                CategoriesTabs::Clis => toggle_one_dependency(
                    &mut self.clis_crates[index_crate_selected],
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

                CategoriesTabs::Cryptography => toggle_one_dependency(
                    &mut self.cryptography_crates[index_crate_selected],
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                ),

                CategoriesTabs::Networking => toggle_one_dependency(
                    &mut self.networking_crates[index_crate_selected],
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                ),

                CategoriesTabs::Databases => toggle_one_dependency(
                    &mut self.database_crates[index_crate_selected],
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                ),

                CategoriesTabs::General => toggle_one_dependency(
                    &mut self.general_crates[index_crate_selected],
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                ),

                CategoriesTabs::Common => toggle_one_dependency(
                    &mut self.common_crates[index_crate_selected],
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                ),

                CategoriesTabs::Math => toggle_one_dependency(
                    &mut self.math_crates[index_crate_selected],
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                ),

                CategoriesTabs::FFI => toggle_one_dependency(
                    &mut self.ffi_crates[index_crate_selected],
                    &mut self.dependencies_to_add_list.dependencies_to_add,
                ),
            };
        }
    }

    pub fn show_popup(&mut self) {
        self.is_adding_deps = true;
    }

    pub fn add_dependencies(&mut self) {
        self.show_popup();
    }

    pub fn check_docs(&self) {
        if let Some(index_selected) = self.crates_list.state.selected() {
            let url = &self.crates_list.crates_widget_list.crates[index_selected].docs;
            open::that(url).ok();
        }
    }

    pub fn check_crates_io(&self) {
        if let Some(index_selected) = self.crates_list.state.selected() {
            let url = format!(
                "https://crates.io/crates/{}",
                self.crates_list.crates_widget_list.crates[index_selected].name
            );
            open::that(url).ok();
        }
    }

    pub fn on_tick(&mut self) {
        self.loader_state.calc_next()
    }
}
