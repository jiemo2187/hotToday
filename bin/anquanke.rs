use reqwest::ClientBuilder;
use serde::Deserialize;
use serde::Serialize;
use tracing::info;
use tracing::Level;

/// anquanke RUL
const URL: &str = "https://www.anquanke.com/webapi/api/index/top/list";

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Response {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize, Serialize)]
pub struct Data {
    pub list: Vec<Item>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Item {
    pub rank: u32,
    pub title: String,
    pub url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().with_max_level(Level::INFO).init();

    let client = ClientBuilder::default().build()?;
    let resp = client.get(URL).send().await?;
    let body = resp.json::<Response>().await?;

    info!("{:?}", body.data.list);
    Ok(())
}
