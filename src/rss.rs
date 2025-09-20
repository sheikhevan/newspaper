use crate::typst::Newspaper;
use rss_gen::{RssData, parse_rss};

pub async fn parse_rss_feed(url: &str) -> Result<RssData, Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let rss_content = response.text().await?;

    match parse_rss(&rss_content, None) {
        Ok(parsed_data) => return Ok(parsed_data),
        Err(e) => {
            eprintln!("There was an error parsing the RSS feed: {}", e);
            Err(Box::new(e))
        }
    }
}

pub fn add_rss_to_newspaper(
    data: RssData,
    newspaper: &mut Newspaper,
    max_articles: u16,
) -> Result<(), Box<dyn std::error::Error>> {
    let latest_posts: Vec<_> = data.items.iter().take(max_articles as usize).collect();

    for item in latest_posts.iter() {
        newspaper.add_article(
            item.title.clone(),
            item.author.clone(),
            item.description.clone(),
            item.link.clone(),
            item.pub_date.clone(),
        );
    }

    Ok(())
}
