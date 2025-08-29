mod config;
mod rss;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init_config()?;
    let config = config::get_config();

    for source in &config.news.sources {
        println!("{}, {}", source.name, source.url)
    }

    Ok(())
}
