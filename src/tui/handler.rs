use crossterm::event::KeyEvent;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};

#[derive(Clone, Copy, Debug)]
pub enum Event {
    Error,
    Tick,
    Key(KeyEvent),
}

pub struct EventHandler {
    _tx: UnboundedSender<Event>,
    rx: UnboundedReceiver<Event>,
}
