use std::error::Error as StdError;
use crate::utils;

pub async fn fetch_tokens() -> Result<serde_json::Value, Box<dyn StdError>> {
    let url = "https://api.dexscreener.com/token-profiles/latest/v1";
    match utils::helpers::fetch_get(url, None).await {
        Ok(body) => {
            return Ok(body);
        },
        Err(e) => {
            println!("Unable to fetch tokens from dexscreener: {}", e);
            return Err(e);
        },
    }
}