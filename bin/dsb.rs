use reqwest::ClientBuilder;
use scraper::Html;
use scraper::Selector;
use serde_json::json;
use tracing::info;
use tracing::Level;

/// DSB 电商报快讯 RUL
const URL: &str = "https://www.dsb.cn/news";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let resp = client.get(URL).send().await?;
    let body = resp.text().await?;

    let mut hot_data = Vec::new();

    let html = Html::parse_document(&body);
    // 解析 <ol><li></li></ol> 的数据
    let li_selector = Selector::parse("ol>li")?;
    let a_selector = Selector::parse("a")?;
    for li in html.select(&li_selector) {
        let title = li
            .select(&a_selector)
            .next()
            .map(|it| it.text().collect::<String>())
            .unwrap_or_default();
        let url = li
            .select(&a_selector)
            .next()
            .and_then(|it| it.value().attr("href"))
            .unwrap_or_default();
        hot_data.push(json!({
            "url": url,
            "title": title,
            "hotScore": 0
        }));
    }

    info!("{}", serde_json::to_string_pretty(&hot_data)?);

    Ok(())
}
