use std::error::Error;

use tui::tui::{init, init_error_hooks, restore};
use view::ui::AppView;

mod backend;
mod content_parser;
mod dependency_builder;
mod scraper;
mod tui;
mod utils;
mod view;

fn main() -> Result<(), Box<dyn Error>> {
    init_error_hooks()?;

    let mut terminal = init()?;

    AppView::new().run(&mut terminal)?;

    restore()?;
    Ok(())
}
