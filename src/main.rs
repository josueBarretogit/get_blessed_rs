#![warn(clippy::pedantic)]
use std::error::Error;

use tui::{
    handler::run,
    tui::{init, init_error_hooks, restore},
};

mod backend;
mod content_parser;
mod dependency_builder;
mod scraper;
mod tui;
mod utils;
mod view;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    init_error_hooks()?;
    init()?;
    run().await?;
    restore()?;
    Ok(())
}
