use tokio;

use crate::rss::{add_rss_to_newspaper, parse_rss_feed};
use crate::typst::{Newspaper, TypstRenderer};

mod config;
mod rss;
mod typst;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init_config()?;
    let config = config::get_config();

    let mut newspaper = Newspaper::new();
    let renderer = TypstRenderer::new("templates")?;

    for source in &config.news.sources {
        match parse_rss_feed(&source.url).await {
            Ok(rss_data) => {
                // TODO: Change this to only print when using "verbose" in config
                println!("Processing feed: {}", &source.url);
                add_rss_to_newspaper(rss_data, &mut newspaper, config.news.max_articles)?;
            }
            Err(e) => {
                eprintln!("Error parsing RSS from {}: {}", &source.url, e);
                continue;
            }
        }
    }

    if newspaper.articles.is_empty() {
        eprintln!("No news articles were successfully parsed. Exiting.");
        return Ok(());
    }

    println!("Here is your news for {}", newspaper.date);

    let output_path = format!("Newspaper_{}.pdf", chrono::Local::now().format("%Y%m%d"));

    renderer.generate_pdf(&newspaper, &output_path)?;

    // TODO: Change this to only print when using "verbose" in config
    println!("\nCollected {} total articles", newspaper.total_articles);

    Ok(())
}
