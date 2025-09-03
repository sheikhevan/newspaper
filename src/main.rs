use tokio;

use crate::rss::{parse_rss_feed, print_latest_posts};

mod config;
mod rss;
mod typst;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init_config()?;
    let config = config::get_config();

    for source in &config.news.sources {
        match parse_rss_feed(&source.url).await {
            Ok(rss) => {
                print_latest_posts(rss, config.news.max_articles);
                Ok(())
            }
            Err(e) => {
                eprintln!("There was an error parsing the RSS: {}", e);
                Err(())
            }
        };
    }

    Ok(())
}
