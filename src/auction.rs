use crate::order_uid::OrderUid;
use alloy_primitives::{Address, B256, U256, Bytes};
use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use serde_with::DisplayFromStr;
use std::collections::HashMap;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auction {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub id: Option<i64>,
    pub tokens: HashMap<Address, Token>,
    pub orders: Vec<Order>,
    pub liquidity: Vec<Liquidity>,
    pub effective_gas_price: U256,
    pub deadline: chrono::DateTime<chrono::Utc>,
    pub surplus_capturing_jit_order_owners: Vec<Address>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Order {
    pub uid: OrderUid,
    pub sell_token: Address,
    pub buy_token: Address,
    pub sell_amount: U256,
    pub full_sell_amount: U256,
    pub buy_amount: U256,
    pub full_buy_amount: U256,
    pub fee_policies: Option<Vec<FeePolicy>>,
    pub valid_to: u32,
    pub kind: Kind,
    pub receiver: Option<Address>,
    pub owner: Address,
    pub partially_fillable: bool,
    pub pre_interactions: Vec<InteractionData>,
    pub post_interactions: Vec<InteractionData>,
    pub sell_token_source: SellTokenSource,
    pub buy_token_destination: BuyTokenDestination,
    pub class: Class,
    pub app_data: B256,
    pub signing_scheme: SigningScheme,
    pub signature: Bytes,
}

/// Destination for which the buyAmount should be transferred to order's
/// receiver to upon fulfillment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BuyTokenDestination {
    /// Pay trade proceeds as an ERC20 token transfer
    Erc20,
    /// Pay trade proceeds as a Vault internal balance transfer
    Internal,
}

/// Source from which the sellAmount should be drawn upon order fulfillment
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SellTokenSource {
    /// Direct ERC20 allowances to the Vault relayer contract
    Erc20,
    /// Internal balances to the Vault with GPv2 relayer approval
    External,
    /// ERC20 allowances to the Vault with GPv2 relayer approval
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InteractionData {
    pub target: Address,
    pub value: U256,
    pub call_data: Bytes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SigningScheme {
    Eip712,
    EthSign,
    Eip1271,
    PreSign,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    Sell,
    Buy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Class {
    Market,
    Limit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum FeePolicy {
    #[serde(rename_all = "camelCase")]
    Surplus { factor: f64, max_volume_factor: f64 },
    #[serde(rename_all = "camelCase")]
    PriceImprovement { factor: f64, max_volume_factor: f64, quote: Quote },
    #[serde(rename_all = "camelCase")]
    Volume { factor: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Quote {
    pub sell_amount: U256,
    pub buy_amount: U256,
    pub fee: U256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Token {
    pub decimals: Option<u8>,
    pub symbol: Option<String>,
    pub reference_price: Option<U256>,
    pub available_balance: U256,
    pub trusted: bool,
}

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Liquidity {
    ConstantProduct(ConstantProductPool),
    WeightedProduct(WeightedProductPool),
    Stable(StablePool),
    ConcentratedLiquidity(ConcentratedLiquidityPool),
    LimitOrder(ForeignLimitOrder),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstantProductPool {
    pub id: String,
    pub address: Address,
    pub router: Address,
    pub gas_estimate: U256,
    pub tokens: HashMap<Address, ConstantProductReserve>,
    pub fee: BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstantProductReserve {
    pub balance: U256,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightedProductPool {
    pub id: String,
    pub address: Address,
    pub balancer_pool_id: B256,
    pub gas_estimate: U256,
    pub tokens: HashMap<Address, WeightedProductReserve>,
    pub fee: BigDecimal,
    pub version: WeightedProductVersion,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeightedProductReserve {
    pub balance: U256,
    pub scaling_factor: BigDecimal,
    pub weight: BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum WeightedProductVersion {
    V0,
    V3Plus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StablePool {
    pub id: String,
    pub address: Address,
    pub balancer_pool_id: B256,
    pub gas_estimate: U256,
    pub tokens: HashMap<Address, StableReserve>,
    pub amplification_parameter: BigDecimal,
    pub fee: BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct StableReserve {
    pub balance: U256,
    pub scaling_factor: BigDecimal,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConcentratedLiquidityPool {
    pub id: String,
    pub address: Address,
    pub router: Address,
    pub gas_estimate: U256,
    pub tokens: Vec<Address>,
    pub sqrt_price: U256,
    #[serde_as(as = "DisplayFromStr")]
    pub liquidity: u128,
    pub tick: i32,
    #[serde_as(as = "HashMap<DisplayFromStr, DisplayFromStr>")]
    pub liquidity_net: HashMap<i32, i128>,
    pub fee: BigDecimal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ForeignLimitOrder {
    pub id: String,
    pub address: Address,
    pub gas_estimate: U256,
    pub hash: B256,
    pub maker_token: Address,
    pub taker_token: Address,
    pub maker_amount: U256,
    pub taker_amount: U256,
    pub taker_token_fee_amount: U256,
}

#[cfg(test)]
mod tests {
    use super::Auction;
    use std::path::PathBuf;

    #[test]
    fn can_deserialize_auction_response() {
        let filepath = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("resources/response_auction.json");
        let file_content = std::fs::read_to_string(filepath).unwrap();

        let auction: Auction = serde_json::from_str(&file_content).unwrap();

        assert_eq!(auction.id, Some(9008466));
    }

    #[test]
    fn can_serialize_auction_response() {
        let auction: Auction = Auction {
            id: Some(123),
            tokens: Default::default(),
            orders: vec![],
            liquidity: vec![],
            effective_gas_price: Default::default(),
            deadline: Default::default(),
            surplus_capturing_jit_order_owners: vec![],
        };
        let serialized_auction = serde_json::to_string(&auction).unwrap();
        let auction: Auction = serde_json::from_str(&serialized_auction).unwrap();

        assert_eq!(auction.id, Some(123));
    }
}
