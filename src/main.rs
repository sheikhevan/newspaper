use tokio;

use crate::rss::parse_rss_feed;

mod config;
mod rss;
mod typst;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init_config()?;
    let config = config::get_config();

    for source in &config.news.sources {
        println!("Processing: {} ({})", source.name, source.url);

        if let Err(e) = parse_rss_feed(&source.url).await {
            eprintln!("Error processing RSS from {}: {}", source.name, e);
        }
    }

    Ok(())
}
