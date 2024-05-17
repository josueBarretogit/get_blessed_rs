use scraper::{html, selectable::Selectable, selector, ElementRef, Html, Selector};

use crate::{
    backend::{
        Categories, CategoriesWithSubCategories, CategoriesWithSubCategoriesIter, Crates, Table,
        TableEntry,
    },
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

    pub fn get_crates(&self, category: Categories) -> Table {
        let section_to_get = format!("#section-{}", category.to_string());

        let entry_selector =
            Selector::parse(format!("{} > table > tbody > tr > td", section_to_get).as_str())
                .unwrap();

        //Each p contains the name of the crate
        let docs_selector = Selector::parse("p > a").unwrap();
        let name_selector = Selector::parse("p > b > a").unwrap();
        let description_selector = Selector::parse("p").unwrap();

        let mut crates_section = self.content.select(&entry_selector);

        let mut entries: Vec<TableEntry> = Vec::new();

        crates_section.for_each(|entr| {
            let crates_in_entry = entr.select(&name_selector);
            let mut crates: Vec<Crates> = Vec::new();
            crates_in_entry.for_each(|cr| {
                let crate_name = cr.inner_html();
                let docs = format!("https://docs.rs/{}/latest/{}/", crate_name, crate_name);

                let description = entr
                    .text()
                    .filter(|text| *text != crate_name && !text.contains("[docs]"))
                    .collect::<String>();

                crates.push(Crates {
                    name: crate_name,
                    description: description.trim().to_string(),
                    docs,
                });
            });
            entries.push(TableEntry {
                use_case: "".into(),
                crates,
            });
        });

        let entries: Vec<TableEntry> = entries
            .iter()
            .filter_map(|entry| {
                if entry.crates.iter().len() != 0 {
                    Some(entry.to_owned())
                } else {
                    None
                }
            })
            .collect();

        Table { entries }
    }

    pub fn get_crates_with_sub(&self, category: CategoriesWithSubCategories) -> Table {
        let section_name = format!("#section-{}", category.to_string());

        let selector =
            Selector::parse(format!("{} > section > table", section_name).as_str()).unwrap();

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
                let crate_name = match con.select(&name_selector).next() {
                    Some(elemen) => elemen.inner_html(),
                    None => "there is no name wtf".into(),
                };

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

        Table { entries }
    }
}
