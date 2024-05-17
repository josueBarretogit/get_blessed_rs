use scraper::{html, selectable::Selectable, selector, Html, Selector};

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

    pub fn get_clis_tables(&self) -> [Table; 3] {
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

            let crates_elements = tbl.select(&name_selector);
            let docs = tbl.select(&docs_selector);

            for ((cont, name), description) in contents
                .zip(tbl.select(&name_selector))
                .zip(tbl.select(&description_selector))
            {
                crates.push(Crates {
                    name: name.inner_html(),
                    description: description
                        .text()
                        .last()
                        .unwrap_or("Description not found".into())
                        .to_string(),
                    docs: "aaa".into(),
                })
            }

            entries.push(TableEntry {
                use_case: "".into(),
                crates,
            })
        });

        println!("{:#?}", entries);

        [
            Table { entries: vec![] },
            Table { entries: vec![] },
            Table { entries: vec![] },
        ]
    }
}
