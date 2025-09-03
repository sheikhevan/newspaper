use tokio;

mod config;
mod rss;
mod typst;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init_config()?;
    let config = config::get_config();

    for source in &config.news.sources {
        match rss::get_rss_feed(&source.url).await {
            Ok(channel) => {
                println!("Fetched from: {} ({})", source.name, source.url);
                println!("Feed title: {}", channel.title);
                let data = typst::fill_newspaper_data(&channel.title, &channel.description)?;
                println!("{:?}", data)
            }
            Err(e) => {
                eprintln!("Error fetching RSS from {}: {}", source.name, e)
            }
        };
    }

    Ok(())
}
