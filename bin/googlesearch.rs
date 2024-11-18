use reqwest::ClientBuilder;
use serde_json::json;
use serde_json::Value;
use tracing::info;
use tracing::Level;

/// Google 搜索趋势 RUL
const URL: &str = "https://trends.google.com/trends/api/realtimetrends?hl=zh-CN&tz=-480&cat=all&fi=0&fs=0&geo=US&ri=300&rs=20&sort=0";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let resp = client.get(URL).send().await?;
    let body = resp.text().await?;

    let body = body.replace(")]}'", "");
    let mut hot_data = Vec::new();

    let data: Value = serde_json::from_str(&body)?;
    if let Some(stories) = data["storySummaries"]["trendingStories"].as_array() {
        for story in stories {
            if let Some(articles) = story["articles"].as_array() {
                for article in articles {
                    let title = article["articleTitle"].as_str().unwrap_or_default();
                    let url = article["url"].as_str().unwrap_or_default();
                    hot_data.push(json!({
                        "url": url,
                        "title": title,
                        "hotScore": 0
                    }));
                }
            }
        }
    }

    info!("{}", serde_json::to_string_pretty(&hot_data)?);

    Ok(())
}
