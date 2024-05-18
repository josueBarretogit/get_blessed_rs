use tui::tui::{init, restore};
use view::ui::AppView;

use crate::content_parser::content_parser::ContentParser;

mod backend;
mod content_parser;
mod dependency_builder;
mod scraper;
mod tui;
mod utils;
mod view;

fn main() {
    let table =  ContentParser::new().get_crates(backend::Categories::Cryptography);

    println!("{:#?}", table);
}
