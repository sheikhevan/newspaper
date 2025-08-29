use tokio;
mod config;
mod rss;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init_config()?;
    let config = config::get_config();

    for source in &config.news.sources {
        println!("{}, {}", source.name, source.url);
        println!("{}", rss::get_rss_feed(&source.url).await?)
    }

    Ok(())
}
