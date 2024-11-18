use reqwest::ClientBuilder;
use scraper::Html;
use scraper::Selector;
use serde_json::json;
use tracing::info;
use tracing::Level;

/// Hack 新闻 RUL
const URL: &str = "https://news.ycombinator.com";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let resp = client.get(URL).send().await?;
    let body = resp.text().await?;

    let mut hot_data = Vec::new();

    let html = Html::parse_document(&body);
    let article_selector = Selector::parse(".athing")?;
    let a_selector = Selector::parse(".titleline a")?;
    for article in html.select(&article_selector) {
        let title = article
            .select(&a_selector)
            .next()
            .map(|l| l.text().collect::<String>())
            .unwrap_or_default();
        let uri = article
            .select(&a_selector)
            .next()
            .and_then(|l| l.value().attr("href"))
            .unwrap_or_default();

        let url = url::Url::parse(URL)?.join(uri)?;

        hot_data.push(json!({
            "url": url.to_string(),
            "title": title.trim(),
            "hotScore": 0
        }));
    }

    info!("{}", serde_json::to_string_pretty(&hot_data)?);

    Ok(())
}
