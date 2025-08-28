use serde::Deserialize;
use std::env::{self, home_dir};
use std::fs::File;
use std::io::prelude::*;

#[derive(Deserialize)]
struct Config {
    news: News,
}

#[derive(Deserialize)]
struct News {
    max_articles: u16,
    sources: NewsSources,
}

#[derive(Deserialize)]
struct NewsSources {
    name: String,
    url: String,
}

fn read_config() -> Config {
    let mut file = File::open(format!(
        "{}/.config/newspaper/config.toml",
        home_dir()?.display()
    ));
}
