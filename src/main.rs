use reqwest::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Query {
    chain: String,
    tokenIn: String,
    tokenOut: String,
    amountIn: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let query = Query {
        chain: "arbitrum".into(),
        tokenIn: "0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1".into(),
        tokenOut: "0x894134a25a5faC1c2C26F1d8fBf05111a3CB9487".into(),
        amountIn: "3000000000000000000000".into(),
    };
    let res = client
        .get("https://aggregator-api.kyberswap.com/arbitrum/api/v1/routes?tokenIn=0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1&tokenOut=0xd85E038593d7A098614721EaE955EC2022B9B91B&amountIn=5000000000000000000000&saveGas=false&gasInclude=true")
        .header("Content-Type", "application/json")
        .header("User-Agent", "Mozilla")
        .send()
        .await?;
    println!("{:#?}", res.text().await?);
    Ok(())
}
