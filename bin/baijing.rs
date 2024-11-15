use reqwest::ClientBuilder;
use scraper::Html;
use scraper::Selector;
use serde_json::json;
use tracing::info;
use tracing::Level;

/// baijing 白鲸出海 RUL
const URL: &str = "https://www.baijing.cn/newsflashes_txzq/";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let resp = client.get(URL).send().await?;
    let body = resp.text().await?;

    let mut hot_data = Vec::new();

    let html = Html::parse_document(&body);
    // 解析 <ul id="content_ul"></ul> 下<li></li>的数据
    let content_selector = Selector::parse("#content_ul>li")?;
    let link_selector = Selector::parse("h3>a")?;
    for content in html.select(&content_selector) {
        let title = content
            .select(&link_selector)
            .next()
            .map(|a| a.text().collect::<String>())
            .unwrap_or_default();
        let title = title.trim();
        let url = content
            .select(&link_selector)
            .next()
            .map(|a| a.value().attr("href").unwrap_or_default())
            .unwrap_or_default();
        let url = url.trim();

        hot_data.push(json!({
            "url": url,
            "title": title,
            "hotScore": 0
        }));
    }

    info!("{}", serde_json::to_string_pretty(&hot_data)?);

    Ok(())
}
