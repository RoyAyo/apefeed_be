use std::error::Error as StdError;
use crate::utils::helpers;


pub async fn get_trending_tokens() -> Result<serde_json::Value, Box<dyn StdError>> {
    let birdeye_api_key: String = std::env::var("BIRDEYE_API_KEY").expect("BIRDEYE_API_KEY must be set");
    let url = "https://public-api.birdeye.so/defi/token_trending";
    let headers = vec![("x-api-key", birdeye_api_key.as_str())];
    let body = helpers::fetch_get(url, Some(headers)).await?;
    Ok(body)
}

// pub async fn get_top_traders() -> Result<serde_json::Value, Box<dyn StdError>> {
//     let url = "https://public-api.birdeye.so/defi/v2/tokens/top_traders";
//     let body = helpers::fetch_get(url).await?;
//     Ok(body)
// }