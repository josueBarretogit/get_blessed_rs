use tui::tui::{init, restore};
use view::ui::AppView;

mod backend;
mod content_parser;
mod dependency_builder;
mod scraper;
mod tui;
mod utils;
mod view;

fn main() {
    let mut terminal = init().unwrap();

    let app_result = AppView::new().run(&mut terminal);

    restore().unwrap()
}
