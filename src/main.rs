use serde::{Deserialize, Serialize};
mod query_struct;

use query_struct::{Query, SwapResponse, Token};
use tokio_postgres::{Error, NoTls};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // load tokens data from db
    let (client, connection) = tokio_postgres::connect(
        "postgresql://dboperator:operatorpass123@localhost:5243/query",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let mut tokens: Vec<Token> = vec![];

    for row in client.query("select * from tokens", &[]).await? {
        let id: i32 = row.get(0);
        let symbol: String = row.get(1);
        let address: String = row.get(2);
        let decimals: i32 = row.get(3);

        tokens.push(Token {
            id,
            symbol,
            address,
            decimals,
        });
    }

    println!("{:#?}", tokens[0].symbol);

    let client = reqwest::Client::new();
    let _query = Query {
        chain: "arbitrum".into(),
        token_in: "0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1".into(),
        token_out: "0x894134a25a5faC1c2C26F1d8fBf05111a3CB9487".into(),
        amount_in: "3000000000000000000000".into(),
    };
    let res = client
        .get("https://aggregator-api.kyberswap.com/arbitrum/api/v1/routes?tokenIn=0xDA10009cBd5D07dd0CeCc66161FC93D7c9000da1&tokenOut=0x894134a25a5faC1c2C26F1d8fBf05111a3CB9487&amountIn=5000000000000000000000&saveGas=false&gasInclude=true")
        .header("Content-Type", "application/json")
        .header("User-Agent", "Mozilla")
        .send()
        .await?;

    let result: SwapResponse = res.json().await?;
    println!("{:#?}", result.data.route_summary.amount_out);

    Ok(())
}
