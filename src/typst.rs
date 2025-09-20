use chrono::Local;
use serde::Serialize;
use tera::Tera;

#[derive(Serialize, Debug)]
pub struct NewsEntry {
    pub title: String,
    pub author: String,
    pub content: String,
    pub link: String,
    pub pub_date: String,
}

#[derive(Serialize, Debug)]
pub struct Newspaper {
    pub date: String,
    pub articles: Vec<NewsEntry>,
    pub total_articles: usize,
}

impl Newspaper {
    pub fn new() -> Self {
        Self {
            date: Local::now().format("%A, %B %d, %Y").to_string(),
            articles: Vec::new(),
            total_articles: 0,
        }
    }

    pub fn add_article(
        &mut self,
        title: String,
        author: String,
        content: String,
        link: String,
        pub_date: String,
    ) {
        self.articles.push(NewsEntry {
            title,
            author,
            content,
            link,
            pub_date,
        });
        self.total_articles = self.articles.len();
    }
}

pub struct TypstRenderer {
    tera: Tera,
}
