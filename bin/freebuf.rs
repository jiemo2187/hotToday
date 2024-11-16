use reqwest::header::HeaderMap;
use reqwest::header::HeaderValue;
use reqwest::header::USER_AGENT;
use reqwest::ClientBuilder;
use scraper::Element;
use scraper::Html;
use scraper::Selector;
use serde_json::json;
use tracing::info;
use tracing::Level;

/// freebuf FreeBuf RUL
const URL: &str = "https://www.freebuf.com/news";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let mut headers = HeaderMap::new();
    headers.insert(USER_AGENT, HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/116.0.0.0 Safari/537.36")?);
    let resp = client.get(URL).headers(headers).send().await?;
    let body = resp.text().await?;

    let mut hot_data = Vec::new();

    let html = Html::parse_document(&body);
    let div_selector = Selector::parse("div>.header-title~div")?;
    let title_selector = Selector::parse("span.title")?;
    let a_selector = Selector::parse("a")?;
    let hot_selector = Selector::parse("div>p.bottom-right")?;
    let span_selector = Selector::parse("a>span")?;
    for div in html.select(&div_selector) {
        let title = div
            .select(&title_selector)
            .next()
            .map(|l| l.text().collect::<String>())
            .unwrap_or_default();
        let uri = div
            .parent_element()
            .and_then(|p| {
                p.select(&a_selector)
                    .next()
                    .and_then(|l| l.value().attr("href"))
            })
            .unwrap_or_default();

        let hot = div
            .select(&hot_selector)
            .next()
            .and_then(|it| {
                it.select(&span_selector)
                    .next()
                    .map(|s| s.text().collect::<String>())
            })
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
