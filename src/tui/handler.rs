use core::time;
use std::{error::Error, io};

use crossterm::{
    event::{KeyCode, KeyEvent},
    terminal,
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::view::ui::AppView;

use super::tui::{init, init_error_hooks, restore};

#[derive(Clone, Copy, Debug)]
pub enum Action {
    Error,
    None,
    Tick,
    Quit,
}

pub struct EventHandler {
    _tx: UnboundedSender<Action>,
    rx: UnboundedReceiver<Action>,
}

pub fn update(app: &mut AppView, even: Action) -> Action {
    match even {
        Action::Tick => {
            app.on_tick();
            Action::Tick
        }

        Action::Quit => Action::Quit,

        Action::None => Action::None,
        Action::Error => unimplemented!(),
    }
}

pub fn handle_event(app: &AppView, tx: UnboundedSender<Action>) -> tokio::task::JoinHandle<()> {
    let tick_rate = time::Duration::from_millis(250);

    tokio::spawn(async move {
        loop {
            let action = if crossterm::event::poll(tick_rate).unwrap() {
                Action::Tick
            } else {
                Action::None
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

    let mut app = AppView::new(action_tx).await;

    let task = handle_event(&app, app.action_tx.clone());

    let tick_rate = std::time::Duration::from_millis(250);

    let mut last_tick = std::time::Instant::now();
    loop {
        terminal.draw(|f| {
            let area = f.size();
            f.render_widget(&mut app, area);
        })?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| std::time::Duration::from_secs(0));

        if crossterm::event::poll(timeout)? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                if !app.is_adding_deps {
                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => app.exit(),
                        KeyCode::Tab => app.next_tab(),
                        KeyCode::BackTab => app.previos_tab(),
                        KeyCode::Down | KeyCode::Char('j') => app.scroll_down(),
                        KeyCode::Up | KeyCode::Char('k') => app.scroll_up(),
                        KeyCode::Char('h') => app.add_dependencies(),
                        KeyCode::Char('s') => app.toggle_select_dependencie(),
                        KeyCode::Char('a') => app.toggle_select_all_dependencies(),
                        KeyCode::Char('d') => app.check_docs(),
                        KeyCode::Char('c') => app.check_crates_io(),
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.on_tick();
            last_tick = std::time::Instant::now();
        }

        if app.exit {
            break;
        }
    }

    task.abort();

    Ok(())
}
