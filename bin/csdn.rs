use reqwest::ClientBuilder;
use serde::Deserialize;
use serde::Serialize;
use tracing::info;
use tracing::Level;

/// CSDN RUL
const URL: &str = "https://blog.csdn.net/phoenix/web/blog/hot-rank?page=0&pageSize=25&type=";

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Response {
    pub data: Vec<Blog>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Blog {
    pub article_title: String,
    pub article_detail_url: String,
    pub view_count: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let resp = client.get(URL).send().await?;
    let body = resp.json::<Response>().await?;
    info!("{:?}", body.data);

    Ok(())
}
