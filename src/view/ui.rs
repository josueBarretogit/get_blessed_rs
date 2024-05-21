use std::usize;
use throbber_widgets_tui::ThrobberState;
use tokio::sync::mpsc::UnboundedSender;

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use crate::{
    backend::{Categories, CategoriesWithSubCategories},
    content_parser::content_parser::ContentParser,
    dependency_builder::CrateToAdd,
    tui::handler::Action,
    utils::{centered_rect, toggle_dependencies_all, toggle_one_dependency, toggle_status_all},
};

use super::widgets::{DependenciesListWidget, *};

pub struct AppView {
    pub action_tx: UnboundedSender<Action>,
    pub dependencies_to_add_list: DependenciesList,
    pub crates_list: CratesList,
    pub category_tabs: CategoriesTabs,

    is_adding_deps: bool,

    popup_widget: Popup,

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
pub struct CratesList {
    crates_widget_list: CratesListWidget,
    state: ListState,
}

#[derive(Default, Clone)]
pub struct DependenciesList {
    pub dependencies_to_add: Vec<CrateToAdd>,
    pub state: ListState,
}

impl DependenciesList {
    pub const fn new(state: ListState, dependencies_to_add: Vec<CrateToAdd>) -> Self {
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
            .direction(Direction::Vertical)
            .constraints([Constraint::Percentage(90), Constraint::Percentage(10)]);

        let [main_area, footer_area] = main_layout.areas(area);

        let main_layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(15),
                Constraint::Percentage(60),
                Constraint::Percentage(25),
            ]);

        let [categories_list_area, crates_list_area, dependencies_to_add_area] =
            main_layout.areas(main_area);

        self.render_categories_list(categories_list_area, buf);

        self.render_main_section(crates_list_area, buf);

        self.render_dependencies_list(dependencies_to_add_area, buf);

        self.render_footer_instructions(footer_area, buf);

        if self.is_adding_deps {
            let center = centered_rect(60, 20, area);
            Clear::default().render(center, buf);
            StatefulWidget::render(
                self.popup_widget.clone(),
                center,
                buf,
                &mut self.loader_state,
            )
        }
    }
}

impl AppView {
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

            popup_widget: Popup::default(),
        }
    }

    pub fn set_adding_deps_operation_message(&mut self, message: &str) {
        self.popup_widget.message = message.to_string();
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
        let block_tabs = Block::bordered().border_set(border::ROUNDED);

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
            "Move down ".into(),
            "<Down> <j> ".bold().blue(),
            "Move up ".into(),
            "<Up> <k> ".bold().blue(),
            "Check docs ".into(),
            "<d> ".blue(),
            "Check crates.io ".into(),
            "<c> ".blue(),
        ]));

        Block::bordered()
            .title("Crate name, description")
            .border_set(border::ROUNDED)
            .title(
                instructions
                    .position(Position::Bottom)
                    .alignment(Alignment::Center),
            )
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

    fn render_footer_instructions(&mut self, area: Rect, buf: &mut Buffer) {
        FooterInstructions::new(vec![
            " Next category ".into(),
            "<Tab>".blue(),
            " Previous category ".into(),
            "<Shift + Tab>".blue(),
            " Toggle select ".into(),
            "<s>".blue(),
            " Toggle select all ".into(),
            "<a>".blue(),
            " Add selected dependencies ".into(),
            "<Enter>".bold().blue(),
            " Quit ".into(),
            " <q> <Esc> ".bold().blue(),
        ])
        .render(area, buf);
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

    pub fn check_docs(&self) {
        if let Some(index_selected) = self.crates_list.state.selected() {
            let crate_name = &self.crates_list.crates_widget_list.crates[index_selected].name;
            let url = format!("https://docs.rs/{}/latest/{}/", crate_name, crate_name);

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
