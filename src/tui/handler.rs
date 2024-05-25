use core::panic;
use std::{error::Error, time::Duration};

use crossterm::event::{KeyCode, KeyEventKind};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{self, UnboundedSender};

use crate::content_parser::content_parser::JsonContentParser;
use crate::{dependency_builder::DependenciesBuilder, view::ui::AppView};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    Tick,
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

pub fn update(app: &mut AppView, action: Action) {
    match action {
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
        Action::ScrollUp => app.scroll_up(),
        Action::ScrollDown => app.scroll_down(),
        Action::Quit => app.exit(),

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
                    Ok(_) => {
                        tx.send(Action::ShowAddingDependenciesOperation).unwrap();
                    }
                    Err(e) => panic!("An Error ocurred, please report it on github: https://github.com/josueBarretogit/get_blessed_rs \n
                    details: {e}"),
                }
            });
        }
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
