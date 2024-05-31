use core::{alloc, panic};
use std::sync::Arc;
use std::{error::Error, time::Duration};

use crates_io_api::CratesQueryBuilder;
use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::content_parser::content_parser::JsonContentParser;
use crate::view::widgets::{CategoriesTabs, CrateItemList};
use crate::{dependency_builder::DependenciesBuilder, view::ui::AppView};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    FetchFeatures,
    UpdateFeatures(CategoriesTabs, Vec<CrateItemList>),
    Tick,
    ToggleShowFeatures,
    ShowLoadingAddingDeps,
    AddingDeps,
    ScrollUp,
    ScrollDown,
    ScrollNextCategory,
    ScrollPreviousCategory,
    ToggleAll,
    ToggleOne,
    CheckDocs,
    CheckCratesIo,
    ShowAddingDependenciesOperation,
    Quit,
}

#[allow(clippy::too_many_lines)]
pub fn update(app: &mut AppView, action: Action) {
    match action {
        Action::ToggleShowFeatures => {
            app.toggle_show_features();
        }
        Action::ShowAddingDependenciesOperation => {
            let tx = app.action_tx.clone();
            app.set_adding_deps_operation_message("Dependencies added successfully ✓");

            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(1200)).await;
                tx.send(Action::Quit).unwrap();
            });
        }

        Action::CheckDocs => app.check_docs(),
        Action::CheckCratesIo => app.check_crates_io(),
        Action::ScrollPreviousCategory => app.previos_tab(),
        Action::ScrollNextCategory => app.next_tab(),
        Action::ToggleOne => app.toggle_select_dependencie(),
        Action::ToggleAll => app.toggle_select_all_dependencies(),
        Action::Tick => {
            app.on_tick();
        }
        Action::ScrollUp => {
            if app.is_showing_features {
                app.scroll_up_features();
            } else {
                app.scroll_up();
            }
        }
        Action::ScrollDown => {
            if app.is_showing_features {
                app.scroll_down_features();
            } else {
                app.scroll_down();
            }
        }

        Action::ShowLoadingAddingDeps => {
            let tx = app.action_tx.clone();
            app.show_popup();

            tokio::spawn(async move {
                tx.send(Action::AddingDeps).unwrap();
            });
        }

        Action::AddingDeps => {
            let tx = app.action_tx.clone();

            let deps_builder =
                DependenciesBuilder::new(app.dependencies_to_add_list.dependencies_to_add.clone());

            tokio::spawn(async move {
                match deps_builder.add_dependencies() {
                    Ok(()) => {
                        tx.send(Action::ShowAddingDependenciesOperation).unwrap();
                    }
                    Err(e) => panic!("An Error ocurred, please report it on github: https://github.com/josueBarretogit/get_blessed_rs \n
                    details: {e}"),
                }
            });
        }
        Action::FetchFeatures => {
            let client = Arc::new(
                crates_io_api::AsyncClient::new(
                    "josuebarretogit (josuebarretogit@gmail.com)",
                    Duration::from_millis(100),
                )
                .unwrap(),
            );

            fetch_features(
                app.general_crates.clone(),
                app.action_tx.clone(),
                Arc::clone(&client),
                CategoriesTabs::General,
            );

            fetch_features(
                app.common_crates.clone(),
                app.action_tx.clone(),
                Arc::clone(&client),
                CategoriesTabs::Common,
            );

            fetch_features(
                app.math_crates.clone(),
                app.action_tx.clone(),
                Arc::clone(&client),
                CategoriesTabs::Math,
            );

            fetch_features(
                app.ffi_crates.clone(),
                app.action_tx.clone(),
                Arc::clone(&client),
                CategoriesTabs::FFI,
            );

            fetch_features(
                app.clis_crates.clone(),
                app.action_tx.clone(),
                Arc::clone(&client),
                CategoriesTabs::Clis,
            );


            fetch_features(
                app.graphics_crates.clone(),
                app.action_tx.clone(),
                Arc::clone(&client),
                CategoriesTabs::Graphics,
            );


            fetch_features(
                app.database_crates.clone(),
                app.action_tx.clone(),
                Arc::clone(&client),
                CategoriesTabs::Databases,
            );


            fetch_features(
                app.networking_crates.clone(),
                app.action_tx.clone(),
                Arc::clone(&client),
                CategoriesTabs::Networking,
            );


            fetch_features(
                app.concurrency_crates.clone(),
                app.action_tx.clone(),
                Arc::clone(&client),
                CategoriesTabs::Concurrency,
            );

        }

        Action::UpdateFeatures(category, items) => match category {
            CategoriesTabs::General => app.general_crates = items,
            CategoriesTabs::Common => app.common_crates = items,
            CategoriesTabs::Math => app.math_crates = items,
            CategoriesTabs::FFI => app.ffi_crates = items,
            CategoriesTabs::Clis => app.clis_crates = items,
            CategoriesTabs::Graphics => app.graphics_crates = items, 
            CategoriesTabs::Databases => app.database_crates = items, 
            CategoriesTabs::Networking => app.networking_crates = items,
            CategoriesTabs::Concurrency => app.concurrency_crates = items,
            CategoriesTabs::Cryptography => app.cryptography_crates = items,
        },

        Action::Quit => app.exit(),
    }
}
pub fn handle_event(tx: UnboundedSender<Action>) -> tokio::task::JoinHandle<()> {
    let tick_rate = std::time::Duration::from_millis(250);

    tokio::spawn(async move {
        loop {
            let action = if crossterm::event::poll(tick_rate).unwrap() {
                if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Enter => Action::ShowLoadingAddingDeps,

                            KeyCode::Char('q') | KeyCode::Esc => Action::Quit,
                            KeyCode::Tab => Action::ScrollNextCategory,
                            KeyCode::BackTab => Action::ScrollPreviousCategory,
                            KeyCode::Down | KeyCode::Char('j') => Action::ScrollDown,
                            KeyCode::Up | KeyCode::Char('k') => Action::ScrollUp,

                            KeyCode::Char('a') => Action::ToggleAll,
                            KeyCode::Char('s') => Action::ToggleOne,
                            KeyCode::Char('d') => Action::CheckDocs,
                            KeyCode::Char('c') => Action::CheckCratesIo,
                            KeyCode::Char('f') => Action::ToggleShowFeatures,
                            _ => Action::Tick,
                        }
                    } else {
                        Action::Tick
                    }
                } else {
                    Action::Tick
                }
            } else {
                Action::Tick
            };

            if tx.send(action).is_err() {
                break;
            }
        }
    })
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    let (action_tx, mut action_rx) = mpsc::unbounded_channel::<Action>();

    let json_parser = JsonContentParser::parse_content().await;

    let mut app = AppView::setup(action_tx.clone(), &json_parser);

    let task = handle_event(app.action_tx.clone());

    action_tx.send(Action::FetchFeatures).unwrap();

    loop {
        terminal.draw(|f| {
            let area = f.size();
            f.render_widget(&mut app, area);
        })?;

        if let Some(action) = action_rx.recv().await {
            update(&mut app, action);
        }

        if app.exit {
            break;
        }
    }

    task.abort();

    Ok(())
}

fn fetch_features(
    crates: Vec<CrateItemList>,
    tx: UnboundedSender<Action>,
    client: Arc<crates_io_api::AsyncClient>,
    category: CategoriesTabs,
) {
    let mut crates_with_features = crates;
    tokio::spawn(async move {
        for crateitem in &mut crates_with_features {
            let crate_name = crateitem.name.as_str();
            let response = client.get_crate(crate_name).await;

            if let Ok(information) = response {
                match information.versions.first() {
                    Some(latest) => {
                        crateitem.features = Some(latest.features.clone().into_keys().collect());
                    }
                    None => crateitem.features = None,
                };
            };
        }

        tx.send(Action::UpdateFeatures(category, crates_with_features))
            .unwrap();
    });
}
