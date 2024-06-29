#![allow(clippy::too_many_lines)]
use throbber_widgets_tui::{Throbber, ThrobberState};
use tokio::sync::mpsc::UnboundedSender;

use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{
        block::{Block, Position, Title},
        Clear, ListState, StatefulWidgetRef,
    },
};
use type_utilities::bool::methods::Toggle;

use crate::{
    backend::{Categories, CategoriesWithSubCategories},
    content_parser::ContentParser,
    tui::handler::Action,
    utils::{
        centered_rect, push_or_remove_crates, toggle_one_feature, toggle_status_all,
        toggle_status_one_crate,
    },
};

use super::widgets::{
    CategoriesWidget, CrateItemList, CratesListWidget, CratesToAddListWidget, FeaturesWidgetList,
    FooterInstructions, Popup,
};

pub struct App {
    pub action_tx: UnboundedSender<Action>,
    ///These are the crates that will be added to the users's project
    pub crates_to_add: CrateToAddList,
    ///These are the crates that the user can select
    pub crates_list: CratesList,
    ///These are the categories e.g General , Common, Math-scientific
    pub crate_categories: CategoriesList,
    ///Contains the information about the features being displayed
    features: Features,
    pub exit: bool,
    pub is_showing_features: bool,
    pub general_crates: Vec<CrateItemList>,
    pub math_crates: Vec<CrateItemList>,
    pub ffi_crates: Vec<CrateItemList>,
    pub cryptography_crates: Vec<CrateItemList>,
    pub common_crates: Vec<CrateItemList>,
    pub concurrency_crates: Vec<CrateItemList>,
    pub networking_crates: Vec<CrateItemList>,
    pub database_crates: Vec<CrateItemList>,
    pub clis_crates: Vec<CrateItemList>,
    pub graphics_crates: Vec<CrateItemList>,
    is_adding_dependencies: bool,
    popup_widget: Popup,
    loader_state: throbber_widgets_tui::ThrobberState,
}

#[derive(Default)]
pub struct Features {
    widget: FeaturesWidgetList,
    state: ListState,
}

///This struct holds the current list of crates displayed
#[derive(Default)]
pub struct CratesList {
    widget: CratesListWidget,
    state: ListState,
}

///This struct holds the list of crates to be added to user's project
#[derive(Default, Clone)]
pub struct CrateToAddList {
    pub widget: CratesToAddListWidget,
    pub state: ListState,
}

///This struct holds the list of crates to be added to user's project
#[derive(Default, Clone)]
pub struct CategoriesList {
    pub widget: CategoriesWidget,
    pub state: ListState,
}

impl CrateToAddList {
    pub const fn new(state: ListState, widget: CratesToAddListWidget) -> Self {
        Self { widget, state }
    }
}

impl Widget for &mut App {
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

        let [categories_list_area, crates_list_area, crates_to_add] = main_layout.areas(main_area);

        self.render_categories_list(categories_list_area, buf);

        self.render_main_section(crates_list_area, buf);

        self.render_crates_to_add_list(crates_to_add, buf);

        App::render_footer_instructions(footer_area, buf);

        if self.is_showing_features {
            self.render_features_popup(area, buf);
        }

        if self.is_adding_dependencies {
            let center = centered_rect(60, 20, area);
            Clear.render(center, buf);
            StatefulWidgetRef::render_ref(&self.popup_widget, center, buf, &mut self.loader_state);
        }
    }
}

impl App {
    pub fn setup(action_tx: UnboundedSender<Action>, parser: &dyn ContentParser) -> Self {
        let page_contents = parser;

        let mut list_state = ListState::default();
        let mut feature_list_state = ListState::default();

        list_state.select(Some(0));
        feature_list_state.select(Some(0));

        let general_crates: Vec<CrateItemList> = page_contents.get_general_crates().into();

        let math_crates: Vec<CrateItemList> = page_contents.get_crates(&Categories::Math).into();
        let ffi_crates: Vec<CrateItemList> = page_contents.get_crates(&Categories::FFI).into();

        let cryptography_crates: Vec<CrateItemList> =
            page_contents.get_crates(&Categories::Cryptography).into();

        let common_crates: Vec<CrateItemList> = page_contents
            .get_crates_with_sub(&CategoriesWithSubCategories::Common)
            .into();
        let concurrency_crates: Vec<CrateItemList> = page_contents
            .get_crates_with_sub(&CategoriesWithSubCategories::Concurrency)
            .into();
        let networking_crates: Vec<CrateItemList> = page_contents
            .get_crates_with_sub(&CategoriesWithSubCategories::Networking)
            .into();
        let database_crates: Vec<CrateItemList> = page_contents
            .get_crates_with_sub(&CategoriesWithSubCategories::Databases)
            .into();
        let clis_crates: Vec<CrateItemList> = page_contents
            .get_crates_with_sub(&CategoriesWithSubCategories::Clis)
            .into();
        let graphics_crates: Vec<CrateItemList> = page_contents
            .get_crates_with_sub(&CategoriesWithSubCategories::Graphics)
            .into();

        Self {
            action_tx,
            crates_to_add: CrateToAddList::default(),
            crates_list: CratesList::default(),
            crate_categories: CategoriesList {
                widget: CategoriesWidget::default(),
                state: list_state,
            },
            loader_state: ThrobberState::default(),
            exit: false,
            is_showing_features: false,
            is_adding_dependencies: false,
            popup_widget: Popup::default(),
            features: Features::default(),
            general_crates,
            math_crates,
            ffi_crates,
            cryptography_crates,
            common_crates,
            concurrency_crates,
            networking_crates,
            database_crates,
            clis_crates,
            graphics_crates,
        }
    }

    pub fn set_adding_deps_operation_message(&mut self, message: &str) {
        self.popup_widget.message = message.to_string();
    }

    pub fn exit(&mut self) {
        self.exit = true;
    }

    pub fn next_category(&mut self) {
        self.crates_list.state.select(Some(0));
        self.crate_categories.widget = self.crate_categories.widget.next();

        self.crate_categories
            .state
            .select(Some(self.crate_categories.widget as usize));
    }

    pub fn previos_category(&mut self) {
        self.crates_list.state.select(Some(0));
        self.crate_categories.widget = self.crate_categories.widget.previous();

        self.crate_categories
            .state
            .select(Some(self.crate_categories.widget as usize));
    }

    pub fn render_categories_list(&mut self, area: Rect, buf: &mut Buffer) {
        let block_tabs = Block::bordered()
            .title_top("Categories")
            .border_set(border::ROUNDED);

        block_tabs.render(area, buf);

        let margin = area.inner(&Margin {
            horizontal: 1,
            vertical: 3,
        });

        StatefulWidget::render(
            self.crate_categories.widget,
            margin,
            buf,
            &mut self.crate_categories.state,
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
            "Select features ".into(),
            "<f> ".blue(),
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
        match self.crate_categories.widget {
            CategoriesWidget::General => {
                self.crates_list.widget = CratesListWidget::new(&self.general_crates);
                StatefulWidgetRef::render_ref(
                    &self.crates_list.widget,
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesWidget::Graphics => {
                self.crates_list.widget = CratesListWidget::new(&self.graphics_crates);

                StatefulWidgetRef::render_ref(
                    &self.crates_list.widget,
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesWidget::Concurrency => {
                self.crates_list.widget = CratesListWidget::new(&self.concurrency_crates);
                StatefulWidgetRef::render_ref(
                    &self.crates_list.widget,
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesWidget::Clis => {
                self.crates_list.widget = CratesListWidget::new(&self.clis_crates);

                StatefulWidgetRef::render_ref(
                    &self.crates_list.widget,
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesWidget::FFI => {
                self.crates_list.widget = CratesListWidget::new(&self.ffi_crates);

                StatefulWidgetRef::render_ref(
                    &self.crates_list.widget,
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesWidget::Math => {
                self.crates_list.widget = CratesListWidget::new(&self.math_crates);

                StatefulWidgetRef::render_ref(
                    &self.crates_list.widget,
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }

            CategoriesWidget::Common => {
                self.crates_list.widget = CratesListWidget::new(&self.common_crates);

                StatefulWidgetRef::render_ref(
                    &self.crates_list.widget,
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesWidget::Databases => {
                self.crates_list.widget = CratesListWidget::new(&self.database_crates);

                StatefulWidgetRef::render_ref(
                    &self.crates_list.widget,
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesWidget::Networking => {
                self.crates_list.widget = CratesListWidget::new(&self.networking_crates);

                StatefulWidgetRef::render_ref(
                    &self.crates_list.widget,
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
            CategoriesWidget::Cryptography => {
                self.crates_list.widget = CratesListWidget::new(&self.cryptography_crates);

                StatefulWidgetRef::render_ref(
                    &self.crates_list.widget,
                    area,
                    buf,
                    &mut self.crates_list.state,
                );
            }
        };
    }

    fn render_crates_to_add_list(&mut self, area: Rect, buf: &mut Buffer) {
        self.crates_to_add.widget =
            CratesToAddListWidget::new(self.crates_to_add.widget.crates.clone());
        StatefulWidgetRef::render_ref(
            &self.crates_to_add.widget,
            area,
            buf,
            &mut self.crates_to_add.state,
        );
    }

    fn render_footer_instructions(area: Rect, buf: &mut Buffer) {
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
                if index == self.crates_list.widget.crates.len().saturating_sub(1) {
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
                    self.crates_list.widget.crates.len().saturating_sub(1)
                } else {
                    index.saturating_sub(1)
                }
            }
            None => 1,
        };
        self.crates_list.state.select(Some(next_index));
    }

    pub fn scroll_up_features(&mut self) {
        let next_index = match self.features.state.selected() {
            Some(index) => {
                let (current_crate_selected, _) = self.get_current_crate_selected().unwrap();
                if index == 0 {
                    current_crate_selected
                        .features
                        .unwrap_or_default()
                        .len()
                        .saturating_sub(1)
                } else {
                    index.saturating_sub(1)
                }
            }
            None => 1,
        };
        self.features.state.select(Some(next_index));
    }

    pub fn scroll_down_features(&mut self) {
        let next = match self.features.state.selected() {
            Some(index) => {
                let (current_crate_selected, _) = self.get_current_crate_selected().unwrap();
                if index
                    == current_crate_selected
                        .features
                        .unwrap_or_default()
                        .len()
                        .saturating_sub(1)
                {
                    0
                } else {
                    index.saturating_add(1)
                }
            }
            None => self.features.state.selected().unwrap_or(0),
        };

        self.features.state.select(Some(next));
    }

    pub fn toggle_select_all_dependencies(&mut self) {
        match self.crate_categories.widget {
            CategoriesWidget::Clis => {
                toggle_status_all(&mut self.clis_crates);
            }
            CategoriesWidget::Graphics => {
                toggle_status_all(&mut self.graphics_crates);
            }
            CategoriesWidget::Concurrency => {
                toggle_status_all(&mut self.concurrency_crates);
            }

            CategoriesWidget::FFI => {
                toggle_status_all(&mut self.ffi_crates);
            }
            CategoriesWidget::Math => {
                toggle_status_all(&mut self.math_crates);
            }

            CategoriesWidget::Common => {
                toggle_status_all(&mut self.common_crates);
            }

            CategoriesWidget::General => {
                toggle_status_all(&mut self.general_crates);
            }

            CategoriesWidget::Databases => {
                toggle_status_all(&mut self.database_crates);
            }

            CategoriesWidget::Networking => {
                toggle_status_all(&mut self.networking_crates);
            }

            CategoriesWidget::Cryptography => {
                toggle_status_all(&mut self.cryptography_crates);
            }
        }
    }

    pub fn get_current_crate_selected(&self) -> Option<(CrateItemList, usize)> {
        self.crates_list.state.selected().map(|index| {
            let crate_item = self.crates_list.widget.crates[index].clone();
            (crate_item, index)
        })
    }

    pub fn toggle_select_dependencie(&mut self) {
        if let Some(index_crate_selected) = self.crates_list.state.selected() {
            match self.crate_categories.widget {
                CategoriesWidget::Clis => {
                    toggle_status_one_crate(&mut self.clis_crates[index_crate_selected]);
                }
                CategoriesWidget::Graphics => {
                    toggle_status_one_crate(&mut self.graphics_crates[index_crate_selected]);
                }
                CategoriesWidget::Concurrency => {
                    toggle_status_one_crate(&mut self.concurrency_crates[index_crate_selected]);
                }

                CategoriesWidget::Cryptography => {
                    toggle_status_one_crate(&mut self.cryptography_crates[index_crate_selected]);
                }

                CategoriesWidget::Networking => {
                    toggle_status_one_crate(&mut self.networking_crates[index_crate_selected]);
                }

                CategoriesWidget::Databases => {
                    toggle_status_one_crate(&mut self.database_crates[index_crate_selected]);
                }

                CategoriesWidget::General => {
                    toggle_status_one_crate(&mut self.general_crates[index_crate_selected]);
                }

                CategoriesWidget::Common => {
                    toggle_status_one_crate(&mut self.common_crates[index_crate_selected]);
                }

                CategoriesWidget::Math => {
                    toggle_status_one_crate(&mut self.math_crates[index_crate_selected]);
                }

                CategoriesWidget::FFI => {
                    toggle_status_one_crate(&mut self.ffi_crates[index_crate_selected]);
                }
            };
        }
    }

    #[inline]
    pub fn show_popup(&mut self) {
        self.is_adding_dependencies = true;
    }

    pub fn check_docs(&self) {
        if let Some(index_selected) = self.crates_list.state.selected() {
            let crate_name = &self.crates_list.widget.crates[index_selected].name;
            let url = format!("https://docs.rs/{crate_name}/latest/{crate_name}/");

            open::that(url).ok();
        }
    }

    pub fn check_crates_io(&self) {
        if let Some(index_selected) = self.crates_list.state.selected() {
            let url = format!(
                "https://crates.io/crates/{}",
                self.crates_list.widget.crates[index_selected].name
            );
            open::that(url).ok();
        }
    }

    #[inline]
    pub fn on_tick(&mut self) {
        self.loader_state.calc_next();
    }

    ///This method checks for selected crates, adds them and ensures not selected crates are
    ///removed
    pub fn push_or_remove_selected_crates(&mut self) {
        match self.crate_categories.widget {
            CategoriesWidget::FFI => {
                push_or_remove_crates(&mut self.crates_to_add.widget.crates, &self.ffi_crates);
            }
            CategoriesWidget::Math => {
                push_or_remove_crates(&mut self.crates_to_add.widget.crates, &self.math_crates);
            }
            CategoriesWidget::Clis => {
                push_or_remove_crates(&mut self.crates_to_add.widget.crates, &self.clis_crates);
            }

            CategoriesWidget::Common => {
                push_or_remove_crates(&mut self.crates_to_add.widget.crates, &self.common_crates);
            }

            CategoriesWidget::General => {
                push_or_remove_crates(&mut self.crates_to_add.widget.crates, &self.general_crates);
            }
            CategoriesWidget::Graphics => {
                push_or_remove_crates(&mut self.crates_to_add.widget.crates, &self.graphics_crates);
            }
            CategoriesWidget::Databases => {
                push_or_remove_crates(&mut self.crates_to_add.widget.crates, &self.database_crates);
            }

            CategoriesWidget::Networking => {
                push_or_remove_crates(
                    &mut self.crates_to_add.widget.crates,
                    &self.networking_crates,
                );
            }

            CategoriesWidget::Concurrency => {
                push_or_remove_crates(
                    &mut self.crates_to_add.widget.crates,
                    &self.concurrency_crates,
                );
            }
            CategoriesWidget::Cryptography => {
                push_or_remove_crates(
                    &mut self.crates_to_add.widget.crates,
                    &self.cryptography_crates,
                );
            }
        }
    }

    #[inline]
    pub fn toggle_show_features(&mut self) {
        if self.get_current_crate_selected().is_some() {
            self.features.state.select(Some(0));
            self.is_showing_features.toggle();
        }
    }

    pub fn toggle_select_one_feature(&mut self) {
        let (current_crate_selected, index_current_crate_selected) =
            self.get_current_crate_selected().unwrap();
        if !current_crate_selected.is_loading {
            match self.crate_categories.widget {
                CategoriesWidget::General => {
                    let current_crate = &mut self.general_crates[index_current_crate_selected];

                    toggle_one_feature(current_crate, &self.features.state);
                }
                CategoriesWidget::Common => {
                    let current_crate = &mut self.common_crates[index_current_crate_selected];
                    toggle_one_feature(current_crate, &self.features.state);
                }
                CategoriesWidget::FFI => {
                    let current_crate = &mut self.ffi_crates[index_current_crate_selected];
                    toggle_one_feature(current_crate, &self.features.state);
                }

                CategoriesWidget::Math => {
                    let current_crate = &mut self.math_crates[index_current_crate_selected];
                    toggle_one_feature(current_crate, &self.features.state);
                }

                CategoriesWidget::Clis => {
                    let current_crate = &mut self.clis_crates[index_current_crate_selected];
                    toggle_one_feature(current_crate, &self.features.state);
                }

                CategoriesWidget::Graphics => {
                    let current_crate = &mut self.graphics_crates[index_current_crate_selected];
                    toggle_one_feature(current_crate, &self.features.state);
                }

                CategoriesWidget::Databases => {
                    let current_crate = &mut self.database_crates[index_current_crate_selected];
                    toggle_one_feature(current_crate, &self.features.state);
                }
                CategoriesWidget::Networking => {
                    let current_crate = &mut self.networking_crates[index_current_crate_selected];
                    toggle_one_feature(current_crate, &self.features.state);
                }
                CategoriesWidget::Concurrency => {
                    let current_crate = &mut self.concurrency_crates[index_current_crate_selected];
                    toggle_one_feature(current_crate, &self.features.state);
                }

                CategoriesWidget::Cryptography => {
                    let current_crate = &mut self.cryptography_crates[index_current_crate_selected];
                    toggle_one_feature(current_crate, &self.features.state);
                }
            }
        }
    }

    fn render_features_popup(&mut self, area: Rect, buf: &mut Buffer) {
        let center = centered_rect(80, 40, area);
        let (current_crate_selected, index_current_crate_selected) =
            self.get_current_crate_selected().unwrap();

        self.features.widget = FeaturesWidgetList::new(
            index_current_crate_selected,
            current_crate_selected.name,
            current_crate_selected.features,
        );

        Clear.render(center, buf);

        if current_crate_selected.is_loading {
            Block::bordered().render(center, buf);

            let loader = Throbber::default()
                .label(format!(
                    "Fetching features of {}, please wait a moment",
                    self.features.widget.crate_name
                ))
                .throbber_set(throbber_widgets_tui::BRAILLE_SIX)
                .use_type(throbber_widgets_tui::WhichUse::Spin);

            StatefulWidget::render(loader, center, buf, &mut self.loader_state);
        } else {
            StatefulWidgetRef::render_ref(
                &self.features.widget,
                center,
                buf,
                &mut self.features.state,
            );
        }
    }
}
