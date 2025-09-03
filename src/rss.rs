use rss_gen::parse_rss;

pub async fn parse_rss_feed(url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let response = reqwest::get(url).await?;
    let rss_content = response.text().await?;

    match parse_rss(&rss_content, None) {
        Ok(parsed_data) => println!("Parsed RSS data: {:?}", parsed_data),
        Err(e) => eprintln!("There was an error parsing the RSS feed: {}", e),
    }

    Ok(())
}
