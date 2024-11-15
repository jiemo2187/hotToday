use reqwest::ClientBuilder;
use scraper::Html;
use scraper::Selector;
use serde_json::json;
use tracing::info;
use tracing::Level;

/// crypto 加密货币价格 RUL
const URL: &str = "https://crypto.com/price";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let resp = client.get(URL).send().await?;
    let body = resp.text().await?;

    let mut hot_data = Vec::new();

    let html = Html::parse_document(&body);
    let price_selector = Selector::parse("div>table>tbody>tr")?;
    let text_selector = Selector::parse("p.chakra-text")?;
    for element in html.select(&price_selector) {
        let name = element
            .select(&text_selector)
            .next()
            .map(|it| it.text().collect::<String>())
            .unwrap_or_default();
        let price = element
            .select(&text_selector)
            .nth(1)
            .map(|it| it.text().collect::<String>())
            .unwrap_or_default();
        let hot = element
            .select(&text_selector)
            .last()
            .map(|it| it.text().collect::<String>())
            // .map(|it| it.parse::<i32>().unwrap_or_default())
            .unwrap_or_default();

        hot_data.push(json!({
            "url": format!("https://crypto.com/price/{}", name.to_lowercase().replace(" ", "-")),
            "title": format!("{} {}", name, price),
            "hotScore": hot
        }));
    }

    info!("{}", serde_json::to_string_pretty(&hot_data)?);

    Ok(())
}
