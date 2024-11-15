use regex::Regex;
use reqwest::ClientBuilder;
use scraper::Html;
use scraper::Selector;
use serde_json::json;
use tracing::info;
use tracing::Level;

/// Baidu 百度热搜 RUL
const URL: &str = "https://top.baidu.com/board?tab=realtime";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let resp = client.get(URL).send().await?;
    let body = resp.text().await?;

    let mut hot_data = Vec::new();

    let html = Html::parse_document(&body);
    // 解析 <div id="sanRoot"></div> 的数据
    let root = Selector::parse("div#sanRoot")?;
    let root_content = html
        .select(&root)
        .next()
        .map(|it| it.html())
        .unwrap_or_default();
    let regex = Regex::new("<!--s-data:(.*?)-->")?;
    if let Some(captures) = regex.captures(&root_content) {
        let data = captures.get(1).map(|m| m.as_str()).unwrap_or_default();
        let data: serde_json::Value = serde_json::from_str(data)?;
        let card_list = data["data"]["cards"].as_array();
        if let Some(card_list) = card_list {
            for card in card_list {
                let content_list = card["content"].as_array();
                if let Some(content_list) = content_list {
                    for content in content_list {
                        let url = content["appUrl"].as_str().unwrap_or_default().to_string();
                        let title = content["word"].as_str().unwrap_or_default().to_string();
                        let hot_score = content["hotScore"].as_i64().unwrap_or_default();
                        hot_data.push(json!({
                            "url": url,
                            "title": title,
                            "hotScore": hot_score
                        }));
                    }
                }
            }
        }
    }

    info!("{}", serde_json::to_string_pretty(&hot_data)?);

    Ok(())
}
