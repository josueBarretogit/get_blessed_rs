use core::time;
use std::{error::Error, io};

use crossterm::{event::KeyEvent, terminal};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::sync::mpsc::{self, UnboundedReceiver, UnboundedSender};

use crate::view::ui::AppView;

use super::tui::{init, init_error_hooks, restore};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Error,
    None,
    Tick,
    Quit,
}

pub struct EventHandler {
    _tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<Event>,
}


pub fn update(app : &mut AppView, even : Event) -> Event {
    match even {
        Event::Tick => {
            app.on_tick();
            Event::Tick
        },

        Event::Quit => {
            app.exit();
            Event::Quit
        }, 

        Event::None => Event::None,
        Event::Error => unimplemented!(), 
    }
}

pub fn handle_event(app : &mut AppView, tx: UnboundedSender<Event>) -> tokio::task::JoinHandle<()> {

    let tick_rate = time::Duration::from_millis(250);

    tokio::spawn(async move {
        loop {
            let action = if crossterm::event::poll(tick_rate).unwrap() {
                Event::Tick
            } else {
                Event::None
            };

            if let Err(_) = tx.send(action) {
                break;
            }
        }
    })
}


pub async fn run() -> Result<(), Box<dyn Error>> {


    let mut terminal = Terminal::new(CrosstermBackend::new(std::io::stdout()))?;

    let (action_tx, mut action_rx) = mpsc::unbounded_channel();

    let mut app = AppView::new().await;

    let task = handle_event(&mut app, action_tx);

    loop {
        terminal.draw(|f| {
            let area = f.size();
            f.render_widget(&mut app, area);
            app.handle_events().unwrap();
        
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
