use serde::Deserialize;
use std::env;
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::Read;

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

fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let home = env::var("HOME")?;
    let mut file = OpenOptions::new()
        .read(true)
        .create(true)
        .open(format!("{}/.config/newspaper/config.toml", home))?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
