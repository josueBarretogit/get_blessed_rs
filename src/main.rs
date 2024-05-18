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
    let table = content_parser::content_parser::ContentParser::new().get_general_crates();

    println!("{:#?}", table)
}
