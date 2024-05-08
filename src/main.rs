use  view::crate_category::*;
mod view;

#[tokio::main]
async fn main() {
    let response =
        reqwest::get("https://blessed.rs/crates").await;
    let html_content = response.unwrap().text().await.unwrap();

    display_category_view();

    println!("{html_content}");

    let document = scraper::Html::parse_document(&html_content);
}
