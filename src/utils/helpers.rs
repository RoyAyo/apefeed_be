use reqwest::Client;
use std::error::Error as StdError;

pub async fn fetch_get(url: &str, headers: Option<Vec<(&str, &str)>>) -> Result<serde_json::Value, Box<dyn StdError>> {
    let client = Client::new();
    let mut request = client.get(url);
    
    if let Some(headers) = headers {
        for (key, value) in headers {
            request = request.header(key, value);
            println!("{}: {}", key, value);
        }
    }
    
    let res = request.send().await?;
    if res.status().is_success() {
        println!("Success: {}", res.status());
        let body = res.json().await?;
        return Ok(body);
    } else {
        println!("Error: {}", res.status());
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to fetch top wallets")));
    }
}

pub async fn fetch_post(url: &str, body: &str, headers: Option<Vec<(&str, &str)>>) -> Result<serde_json::Value, Box<dyn StdError>> {
    let client = Client::new();
    let mut req = client.post(url);

    if let Some(headers) = headers {
        for (key, value) in headers {
            req = req.header(key, value);
        }
    }

    let res = req.body(body.to_string())  
        .send()
        .await?;

    if res.status().is_success() {
        println!("Success: {}", res.status());
        let body = res.json().await?;
        return Ok(body);
    } else {
        println!("Error: {}", res.status());
        return Err(Box::new(std::io::Error::new(std::io::ErrorKind::Other, "Failed to fetch top wallets")));
    }
}