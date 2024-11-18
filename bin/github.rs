use reqwest::ClientBuilder;
use scraper::Html;
use scraper::Selector;
use serde_json::json;
use tracing::info;
use tracing::Level;

/// Github 趋势 RUL
const URL: &str = "https://github.com/trending";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let resp = client.get(URL).send().await?;
    let body = resp.text().await?;

    let mut hot_data = Vec::new();

    let html = Html::parse_document(&body);
    let article_selector = Selector::parse("div>article")?;
    let a_selector = Selector::parse("h2 a")?;
    let hot_selector = Selector::parse("h2~div>a")?;
    for article in html.select(&article_selector) {
        let title = article
            .select(&a_selector)
            .next()
            .map(|l| l.text().collect::<String>())
            .map(|s| s.split("/").last().unwrap_or_default().trim().to_string())
            .unwrap_or_default();
        let uri = article
            .select(&a_selector)
            .next()
            .and_then(|l| l.value().attr("href"))
            .unwrap_or_default();

        let hot = article
            .select(&hot_selector)
            .next()
            .map(|it| it.text().collect::<String>().trim().replace(",", ""))
            .unwrap_or_default();

        let url = url::Url::parse(URL)?.join(uri)?;

        hot_data.push(json!({
            "url": url.to_string(),
            "title": title.trim(),
            "hotScore": hot
        }));
    }

    info!("{}", serde_json::to_string_pretty(&hot_data)?);

    Ok(())
}
