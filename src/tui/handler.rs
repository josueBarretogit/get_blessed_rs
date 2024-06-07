use core::panic;
use std::sync::Arc;
use std::{error::Error, time::Duration};

use crossterm::event::{self, poll, Event, KeyCode, KeyEventKind};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::content_parser::jsoncontentparser::JsonContentParser;
use crate::utils::{load_features, select_crate_if_features_are_selected};
use crate::view::widgets::{CategoriesTabs, CrateItemList, FeatureItemList};
use crate::{dependency_builder::DependenciesBuilder, view::app::App};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    FetchFeatures,
    UpdateFeatures(CategoriesTabs, Option<Vec<FeatureItemList>>, usize),
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
pub fn update(app: &mut App, action: Action) {
    match action {
        Action::ToggleShowFeatures => {
            app.toggle_show_features();
            //After user closes the popup where they can se the features we check if we can add
            //the crate if the user selected at least 1 feature
            //THe way to do this must be improved since it is really ugly
            if !app.is_showing_features {
                select_crate_if_features_are_selected(app);
            }
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
        Action::ScrollPreviousCategory => {
            if !app.is_showing_features {
                app.previos_tab();
            }
        }
        Action::ScrollNextCategory => {
            if !app.is_showing_features {
                app.next_tab();
            }
        }
        Action::ToggleOne => {
            if app.is_showing_features {
                app.toggle_select_one_feature();
            } else {
                app.toggle_select_dependencie();
            }
        }
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
                &app.general_crates,
                &app.action_tx,
                &client,
                CategoriesTabs::General,
            );

            fetch_features(
                &app.common_crates,
                &app.action_tx,
                &client,
                CategoriesTabs::Common,
            );

            fetch_features(
                &app.ffi_crates,
                &app.action_tx,
                &client,
                CategoriesTabs::FFI,
            );

            fetch_features(
                &app.math_crates,
                &app.action_tx,
                &client,
                CategoriesTabs::Math,
            );

            fetch_features(
                &app.clis_crates,
                &app.action_tx,
                &client,
                CategoriesTabs::Clis,
            );

            fetch_features(
                &app.graphics_crates,
                &app.action_tx,
                &client,
                CategoriesTabs::Graphics,
            );

            fetch_features(
                &app.networking_crates,
                &app.action_tx,
                &client,
                CategoriesTabs::Networking,
            );

            fetch_features(
                &app.database_crates,
                &app.action_tx,
                &client,
                CategoriesTabs::Databases,
            );

            fetch_features(
                &app.cryptography_crates,
                &app.action_tx,
                &client,
                CategoriesTabs::Cryptography,
            );

            fetch_features(
                &app.concurrency_crates,
                &app.action_tx,
                &client,
                CategoriesTabs::Concurrency,
            );
        }

        Action::UpdateFeatures(category, features, crate_index_to_update) => match category {
            CategoriesTabs::General => {
                load_features(&mut app.general_crates, crate_index_to_update, features);
            }
            CategoriesTabs::Common => {
                load_features(&mut app.common_crates, crate_index_to_update, features);
            }
            CategoriesTabs::FFI => {
                load_features(&mut app.ffi_crates, crate_index_to_update, features);
            }

            CategoriesTabs::Math => {
                load_features(&mut app.math_crates, crate_index_to_update, features);
            }
            CategoriesTabs::Clis => {
                load_features(&mut app.clis_crates, crate_index_to_update, features);
            }
            CategoriesTabs::Graphics => {
                load_features(&mut app.graphics_crates, crate_index_to_update, features);
            }
            CategoriesTabs::Databases => {
                load_features(&mut app.database_crates, crate_index_to_update, features);
            }
            CategoriesTabs::Networking => {
                load_features(&mut app.networking_crates, crate_index_to_update, features);
            }

            CategoriesTabs::Concurrency => {
                load_features(&mut app.concurrency_crates, crate_index_to_update, features);
            }
            CategoriesTabs::Cryptography => load_features(
                &mut app.cryptography_crates,
                crate_index_to_update,
                features,
            ),
        },

        Action::Quit => app.exit(),
    }
}
pub fn handle_event(tx: UnboundedSender<Action>) -> tokio::task::JoinHandle<()> {
    let tick_rate = std::time::Duration::from_millis(250);

    tokio::spawn(async move {
        loop {
            let action = if poll(tick_rate).unwrap() {
                if let Event::Key(key) = event::read().unwrap() {
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

    let mut app = App::setup(action_tx.clone(), &json_parser);

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
    crates: &[CrateItemList],
    tx: &UnboundedSender<Action>,
    client: &Arc<crates_io_api::AsyncClient>,
    category: CategoriesTabs,
) {
    for (index, crateitem) in crates.iter().enumerate() {
        let crate_name = crateitem.name.clone();
        let tx = tx.clone();
        let client = Arc::clone(client);
        tokio::spawn(async move {
            let response = client.get_crate(&crate_name).await;
            if let Ok(information) = response {
                if let Some(latest) = information.versions.first() {
                    let latest: Vec<FeatureItemList> = latest
                        .features
                        .clone()
                        .into_keys()
                        .map(FeatureItemList::new)
                        .collect();

                    if !latest.is_empty() {
                        tx.send(Action::UpdateFeatures(category, Some(latest), index))
                            .unwrap_or(());
                    } else {
                        tx.send(Action::UpdateFeatures(category, None, index))
                            .unwrap_or(());
                    }
                };
            };
        });
    }
}
