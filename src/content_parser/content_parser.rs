use scraper::{html, selectable::Selectable, selector, Html, Selector};

use crate::{
    backend::{Categories, Table, TableEntry},
    scraper::scraper::scrape_site,
};

pub struct ContentParser {
    content: Html,
}

impl ContentParser {
    pub fn new() -> Self {
        let page_content = scrape_site().unwrap();

        let html_content = Html::parse_document(&page_content);

        Self {
            content: html_content,
        }
    }

    pub fn get_clis_tables(&self) -> Table {
        let selector = Selector::parse("#section-cli-tools > section > table").unwrap();

        let entry_selector = Selector::parse("tr").unwrap();

        //Each p contains the name of the crate
        let crates_selector = Selector::parse("td > p > a").unwrap();

        let cli_section = self.content.select(&selector);

        let entries: Vec<TableEntry> = Vec::new();

        cli_section.for_each(|tbl| {
            let entry = tbl.select(&entry_selector);

            entry.for_each(|cra| {
                let crates = cra.select(&crates_selector);
            })
        });

        Table { entries: vec![] }
    }
}
