use reqwest::ClientBuilder;
use scraper::Element;
use scraper::Html;
use scraper::Selector;
use serde_json::json;
use tracing::info;
use tracing::Level;

/// douban 豆瓣电影 RUL
const URL: &str = "https://movie.douban.com/chart";

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let resp = client.get(URL).send().await?;
    let body = resp.text().await?;

    let mut hot_data = Vec::new();

    let html = Html::parse_document(&body);
    //#listCont2 > li:nth-child(1) > span > div
    //#listCont1 > li:nth-child(2) > span
    let ul_selector = Selector::parse(".movie_top>.movie_top>ul")?;
    let h2_selector = Selector::parse("h2")?;
    let li_selector = Selector::parse("li")?;
    let a_selector = Selector::parse("div>a")?;
    let span_selector = Selector::parse("span")?;
    for ul in html.select(&ul_selector) {
        let name = ul
            .parent_element()
            .and_then(|p| {
                p.select(&h2_selector)
                    .next()
                    .map(|h2| h2.text().collect::<String>())
            })
            .unwrap_or_default();
        info!("{}", name);

        for li in ul.select(&li_selector) {
            let title = li
                .select(&a_selector)
                .next()
                .map(|l| l.text().collect::<String>())
                .unwrap_or_default();
            let url = li
                .select(&a_selector)
                .next()
                .and_then(|l| l.value().attr("href"))
                .unwrap_or_default();

            let hot = li
                .select(&span_selector)
                .next()
                .map(|s| s.text().collect::<String>())
                .unwrap_or_default();

            hot_data.push(json!({
                "url": url,
                "title": title.trim(),
                "hotScore": hot.trim()
            }));
        }
    }

    info!("{}", serde_json::to_string_pretty(&hot_data)?);

    Ok(())
}
