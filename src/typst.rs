use chrono::{Datelike, Local};
use serde::Serialize;
use tera::Tera;

#[derive(Serialize, Debug)]
struct NewsEntry {
    title: String,
    content: String,
}

#[derive(Serialize, Debug)]
pub struct Newspaper {
    date: String,
    articles: Vec<NewsEntry>,
}

pub fn fill_newspaper_data(
    title: &str,
    content: &str,
) -> Result<Newspaper, Box<dyn std::error::Error>> {
    let newspaper = Newspaper {
        date: Local::now().format("%A, %B %d, %Y").to_string(),
        articles: vec![NewsEntry {
            title: title.to_string(),
            content: content.to_string(),
        }],
    };

    Ok(newspaper)
}
