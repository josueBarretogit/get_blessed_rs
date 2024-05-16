use scraper::scraper::scrape_site;
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
    content_parser::content_parser::ContentParser::new().get_clis_tables();
}
