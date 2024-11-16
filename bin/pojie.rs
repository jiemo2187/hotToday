use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::USER_AGENT;
use reqwest::ClientBuilder;
use scraper::Html;
use scraper::Selector;
use serde_json::json;
use tracing::info;
use tracing::Level;

/// pojie 吾爱破解 RUL
const URL: &str = "https://www.52pojie.cn/forum.php?mod=guide&view=hot";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36")?);
    let resp = client.get(URL).headers(headers).send().await?;
    let body = resp.text().await?;

    info!("Response: {}", body);
    let mut hot_data = Vec::new();

    let html = Html::parse_document(&body);
    let tbody_selector = Selector::parse("#threadlist table>tbody")?;
    let a_selector = Selector::parse("th>a")?;
    let em_selector = Selector::parse("td.num em")?;
    for tbody in html.select(&tbody_selector) {
        let title = tbody
            .select(&a_selector)
            .next()
            .map(|l| l.text().collect::<String>())
            .unwrap_or_default();
        let uri = tbody
            .select(&a_selector)
            .next()
            .and_then(|l| l.value().attr("href"))
            .unwrap_or_default();

        let hot = tbody
            .select(&em_selector)
            .next()
            .map(|it| it.text().collect::<String>())
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
