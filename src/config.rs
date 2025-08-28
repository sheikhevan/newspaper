use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::{Read, Write};
use std::path::Path;
use std::sync::OnceLock;
use std::{env, fs};

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub news: News,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct News {
    pub max_articles: u16,
    pub sources: NewsSources,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct NewsSources {
    pub name: String,
    pub url: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            news: News::default(),
        }
    }
}

impl Default for News {
    fn default() -> Self {
        News {
            max_articles: 5,
            sources: NewsSources::default(),
        }
    }
}

impl Default for NewsSources {
    fn default() -> Self {
        NewsSources {
            name: "Vatican News".to_string(),
            url: "https://www.vaticannews.va/en.rss.xml".to_string(),
        }
    }
}

static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = read_config()?;
    CONFIG
        .set(config)
        .map_err(|_| "Config already initialized")?;
    Ok(())
}

pub fn get_config() -> &'static Config {
    CONFIG.get().expect("Config not initialized")
}

pub fn read_config() -> Result<Config, Box<dyn std::error::Error>> {
    let home = env::var("HOME")?;
    let config_path = format!("{}/.config/newspaper/config.toml", home);

    if let Some(parent) = Path::new(&config_path).parent() {
        fs::create_dir_all(parent)?;
    }

    if !Path::new(&config_path).exists() {
        let default_config = Config::default();
        let toml_string = toml::to_string_pretty(&default_config)?;

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .open(&config_path)?;

        file.write_all(toml_string.as_bytes())?;
    }

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(config_path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}
