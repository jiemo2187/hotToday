use reqwest::ClientBuilder;
use scraper::Html;
use scraper::Selector;
use serde_json::json;
use tracing::info;
use tracing::Level;

/// 360 历史上的今天 RUL
const URL: &str = "https://hao.360.com/histoday/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;

    let resp = client.get(URL).send().await?;
    let body = resp.text().await?;

    let mut hot_data = Vec::new();

    let html = Html::parse_document(&body);
    let today_selector = Selector::parse(".tih-list>dl>dt")?;
    for today in html.select(&today_selector) {
        let title = today.text().collect::<String>();

        hot_data.push(json!({
            "url": "",
            "title": title.trim(),
            "hotScore": 0
        }));
    }

    info!("{}", serde_json::to_string_pretty(&hot_data)?);

    Ok(())
}
