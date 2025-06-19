use scraper::{Html, Selector};
use reqwest::Client;

pub async fn fetch_links(client: &Client, url: &str) -> Result<Vec<String>, reqwest::Error> {
    let body = client.get(url).send().await?.text().await?;
    let document = Html::parse_document(&body);
    let selector = Selector::parse("a").unwrap();

    let links = document
        .select(&selector)
        .filter_map(|elem| elem.value().attr("href"))
        .filter(|href| href.starts_with("/wiki/") && !href.contains(":"))
        .map(|href| format!("https://es.wikipedia.org{}", href))
        .collect();

    Ok(links)
}

pub fn url_to_title(url: &str) -> String {
    if let Some(pos) = url.find("/wiki/") {
        let title = &url[pos + 6..];
        let title = title.replace('_', " ");
        percent_encoding::percent_decode_str(&title)
            .decode_utf8()
            .unwrap_or_else(|_| title.clone().into())
            .to_string()
    } else {
        url.to_string()
    }
}
