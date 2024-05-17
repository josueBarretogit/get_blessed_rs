use scraper::{html, selectable::Selectable, selector, ElementRef, Html, Selector};

use crate::{
    backend::{Categories, Crates, Table, TableEntry},
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

        let entry_selector = Selector::parse("tbody > tr  td > p").unwrap();

        //Each p contains the name of the crate
        let docs_selector = Selector::parse("p > a").unwrap();
        let name_selector = Selector::parse("p > b > a").unwrap();
        let description_selector = Selector::parse("p").unwrap();

        let cli_section = self.content.select(&selector);

        let mut entries: Vec<TableEntry> = Vec::new();

        cli_section.for_each(|tbl| {
            let contents = tbl.select(&entry_selector);

            let mut crates: Vec<Crates> = Vec::new();

            contents.for_each(|con| {
                let crate_name = con.select(&name_selector).next().unwrap().inner_html();
                let docs = format!("https://docs.rs/{}/latest/{}/", crate_name, crate_name);

                let text = con
                    .text()
                    .filter(|text| *text != crate_name && !text.contains("[docs]"))
                    .collect::<String>();

                crates.push(Crates {
                    name: crate_name,
                    description: text.trim().to_string(),
                    docs,
                });
            });

            entries.push(TableEntry {
                use_case: "".into(),
                crates,
            })
        });

        println!("{:#?}", entries);

        Table { entries }
    }
}
