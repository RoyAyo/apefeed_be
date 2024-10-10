use std::error::Error as StdError;

use crate::{utils::helpers, EnvVar};


pub async fn get_trending_tokens(env_var: &EnvVar) -> Result<serde_json::Value, Box<dyn StdError>> {
    let birdeye_api_key = env_var.birdeye_api_key.clone();
    let url = "https://public-api.birdeye.so/defi/token_trending";
    let headers = vec![("x-api-key", birdeye_api_key.as_str())];
    let body = helpers::fetch_get(url, Some(headers)).await?;
    Ok(body)
}

pub async fn get_top_traders(env_var: &EnvVar) -> Result<serde_json::Value, Box<dyn StdError>> {
    let birdeye_api_key = env_var.birdeye_api_key.clone();
    print!("{}", birdeye_api_key);
    let url = "https://public-api.birdeye.so/defi/v2/tokens/top_traders";
    let headers = vec!(("x-api-key", birdeye_api_key.as_str()));
    println!("headers {:?}", headers);
    let body = helpers::fetch_get(url, Some(headers)).await?;
    Ok(body)
}