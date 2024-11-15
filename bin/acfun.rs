
use reqwest::ClientBuilder;
use serde::Deserialize;
use serde::Serialize;
use tracing::info;
use tracing::Level;

/// AcFun RUL
const URL : &str = "https://www.acfun.cn/rest/pc-direct/rank/channel?channelId=&subChannelId=&rankLimit=30&rankPeriod=DAY";

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct AcFunResponse {
    #[serde(rename = "rankList")]
    pub rank_list: Vec<Rank>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rank {
    pub content_title: String,
    pub share_url: String,
    pub view_count: u32,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let resp = client.get(URL).send().await?;
    let body = resp.json::<AcFunResponse>().await?;

    info!("{:?}", body.rank_list);
    Ok(())
}
