use rss::Channel;
use std::error::Error;

pub async fn get_rss_feed(url: &str) -> Result<Channel, Box<dyn Error>> {
    let content = reqwest::get(url).await?.bytes().await?;
    let channel = Channel::read_from(&content[..])?;

    Ok(channel)
}
