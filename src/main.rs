mod config;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    config::init_config()?;
    let config = config::get_config();

    println!("{}", config.news.max_articles);
    Ok(())
}
