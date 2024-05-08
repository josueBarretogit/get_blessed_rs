use tui::tui::{init, restore};
use view::{crate_category::*, view::AppView};

mod backend;
mod content_parser;
mod dependency_builder;
mod scraper;
mod tui;
mod utils;
mod view;

#[tokio::main]
async fn main() {
    let mut terminal = init().unwrap();

    let app_result = AppView::default().run(&mut terminal);

    restore().unwrap()
}
