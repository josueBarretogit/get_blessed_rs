use core::time;
use std::{error::Error, io, process, time::Duration};

use crossterm::{
    event::{KeyCode, KeyEvent, KeyEventKind},
    terminal,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::{dependency_builder::DependenciesBuilder, view::ui::AppView};

use super::tui::{init, init_error_hooks, restore};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Action {
    None,
    Tick,
    ShowLoadingAddingDeps,
    AddingDeps,
    ScrollUp,
    ScrollDown,
    SelectAll,
    ShowAddingOperation,
    Quit,
}

pub fn update(app: &mut AppView, even: Action) {
    match even {
        Action::ShowAddingOperation => {
            let tx = app.action_tx.clone();
            app.set_adding_deps_operation_message(" Dependencies added successfully ");

            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_secs(2)).await;
                tx.send(Action::Quit).unwrap();
            });
        }

        Action::Tick => {
            app.on_tick();
        }

        Action::ScrollUp => app.scroll_up(),
        Action::ScrollDown => app.scroll_down(),
        Action::SelectAll => app.toggle_select_all_dependencies(),
        Action::Quit => app.exit(),
        Action::None => {}

        Action::ShowLoadingAddingDeps => {
            let tx = app.action_tx.clone();
            app.add_dependencies();

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
                        tx.send(Action::ShowAddingOperation).unwrap();
                    }
                    Err(_) => todo!(),
                }
            });
        }
    }
}
pub fn handle_event(tx: UnboundedSender<Action>) -> tokio::task::JoinHandle<()> {
    let tick_rate = std::time::Duration::from_millis(250);

    let mut last_tick = std::time::Instant::now();

    tokio::spawn(async move {
        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));

        loop {
            let action = if crossterm::event::poll(timeout).unwrap() {
                if let crossterm::event::Event::Key(key) = crossterm::event::read().unwrap() {
                    if key.kind == KeyEventKind::Press {
                        match key.code {
                            KeyCode::Enter => Action::ShowLoadingAddingDeps,
                            KeyCode::Up => Action::ScrollUp,
                            KeyCode::Down => Action::ScrollDown,
                            KeyCode::Char('q') => Action::Quit,
                            KeyCode::Char('a') => Action::SelectAll,
                            _ => Action::Tick,
                        }
                    } else {
                        Action::Tick
                    }
                } else if last_tick.elapsed() >= tick_rate {
                    last_tick = std::time::Instant::now();
                    Action::Tick
                } else {
                    Action::Tick
                }
            } else {
                Action::Tick
            };

            if let Err(_) = tx.send(action) {
                break;
            }
        }
    })
}

pub async fn run() -> Result<(), Box<dyn Error>> {
    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    let (action_tx, mut action_rx) = mpsc::unbounded_channel::<Action>();

    let mut app = AppView::new(action_tx.clone()).await;

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

pub async fn handle_key_events(app: &mut AppView, code: KeyCode, should_exit: &mut bool) {
    match code {
        KeyCode::Char('q') | KeyCode::Esc => *should_exit = true,
        KeyCode::Tab => app.next_tab(),
        KeyCode::BackTab => app.previos_tab(),
        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),

        KeyCode::Char('s') => app.toggle_select_dependencie(),
        KeyCode::Char('a') => app.toggle_select_all_dependencies(),
        KeyCode::Char('d') => app.check_docs(),
        KeyCode::Char('c') => app.check_crates_io(),
        _ => {}
    }
}
