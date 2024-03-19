use reqwest;
#[tokio::main]

pub async fn get_prices() -> Result<String, Box<dyn std::error::Error>> {
    let client = reqwest::Client::builder()
        .build()?;
        
    let mut headers = reqwest::header::HeaderMap::new();
    headers.insert("X-MBX-APIKEY", "API-KEY".parse()?);

    let request = client.request(reqwest::Method::GET, "https://api.binance.com/api/v3/ticker/price")
        .headers(headers);

    let response = request.send().await?;
    let body = response.text().await?;
    
    return Ok(body);
}
