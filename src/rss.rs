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

pub fn print_latest_posts(data: RssData, number: u16) -> Result<(), Box<dyn std::error::Error>> {
    let latest_posts: Vec<_> = data.items.iter().take(number as usize).collect();

    for (i, item) in latest_posts.iter().enumerate() {
        println!("{}. {}", i + 1, &item.title);
        println!("\nLink: {}", &item.link);
        println!("\nPub Date: {}", &item.pub_date);
    }
    Ok(())
}
