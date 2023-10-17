use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize)]
pub struct Query {
    pub chain: String,
    pub token_in: String,
    pub token_out: String,
    pub amount_in: String,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct SwapResponse {
    pub code: i64,
    pub message: String,
    pub data: Data,
    //pub request_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    pub route_summary: RouteSummary,
    router_address: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RouteSummary {
    token_in: String,
    amount_in: String,
    amount_in_usd: String,
    token_in_market_price_available: bool,
    token_out: String,
    pub amount_out: String,
    amount_out_usd: String,
    token_out_market_price_available: bool,
    gas: String,
    gas_price: String,
    gas_usd: String,
    extra_fee: ExtraFee,
    route: Vec<Vec<Route>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExtraFee {
    fee_amount: String,
    charge_fee_by: String,
    is_in_bps: bool,
    fee_receiver: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Route {
    pool: String,
    token_in: String,
    token_out: String,
    limit_return_amount: String,
    swap_amount: String,
    amount_out: String,
    exchange: String,
    pool_length: i64,
    pool_type: String,
    pool_extra: Option<PoolExtra>,
    extra: Option<Extra>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Extra {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PoolExtra {
    vault: Option<String>,
    pool_id: Option<String>,
    map_token_address_to_index: Option<HashMap<String, i64>>,
    stable: Option<bool>,
    swap_fee: Option<SwapFee>,
    fee_precision: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SwapFee {
    Integer(i64),
    String(String),
}

pub struct Token {
    pub id: i32,
    pub symbol: String,
    pub address: String,
    pub decimals: i32,
}
