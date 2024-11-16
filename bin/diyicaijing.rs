use reqwest::ClientBuilder;
use serde::Deserialize;
use serde::Serialize;
use tracing::info;
use tracing::Level;

/// 第一财经 RUL
const URL: &str = "https://www.yicai.com/api/ajax/getranklistbykeys?keys=newsRank%2CvideoRank%2CimageRank%2CliveRank";

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Response {
    pub news_rank: Rank,
    pub video_rank: Rank,
    pub image_rank: Rank,
    pub live_rank: Rank,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub week: Vec<Row>,
    pub month: Vec<Row>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Row {
    #[serde(rename = "NewsID")]
    pub news_id: u32,
    #[serde(rename = "NewsTitle")]
    pub news_title: String,
    #[serde(rename = "NewsType")]
    pub news_type: u32,
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let resp = client.get(URL).send().await?;
    let body = resp.json::<Response>().await?;
    info!("{:?}", body.news_rank);

    Ok(())
}
