use tokio;

use crate::rss::{get_rss_feed, parse_rss_feed};

mod config;
mod rss;
mod typst;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init_config()?;
    let config = config::get_config();

    for source in &config.news.sources {
        match get_rss_feed(&source.url).await {
            Ok(channel) => {
                println!("Fetched from: {} ({})", source.name, source.url);
                println!("Feed title: {}", channel.title);

                if let Err(e) = parse_rss_feed(&source.url).await {
                    eprintln!("Error parsing RSS from {}: {}", source.name, e);
                }
            }
            Err(e) => {
                eprintln!("Error fetching RSS from {}: {}", source.name, e);
            }
        }
    }

    Ok(())
}
