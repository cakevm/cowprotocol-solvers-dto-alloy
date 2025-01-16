use alloy_primitives::{Address, Bytes, FixedBytes, B256, U256};
use serde::Deserialize;
use {serde::Serialize, serde_with::serde_as, std::collections::HashMap};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Solutions {
    pub solutions: Vec<Solution>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Solution {
    pub id: u64,
    pub prices: HashMap<Address, U256>,
    pub trades: Vec<Trade>,
    #[serde(default)]
    pub pre_interactions: Vec<Call>,
    pub interactions: Vec<Interaction>,
    #[serde(default)]
    pub post_interactions: Vec<Call>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gas: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Trade {
    Fulfillment(Fulfillment),
    Jit(JitTrade),
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fulfillment {
    pub order: FixedBytes<56>,
    pub executed_amount: U256,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee: Option<U256>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JitTrade {
    pub order: JitOrder,
    pub executed_amount: U256,
    pub fee: Option<U256>,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JitOrder {
    pub sell_token: Address,
    pub buy_token: Address,
    pub receiver: Address,
    pub sell_amount: U256,
    pub buy_amount: U256,
    pub valid_to: u32,
    pub app_data: B256,
    pub kind: Kind,
    pub sell_token_balance: SellTokenBalance,
    pub buy_token_balance: BuyTokenBalance,
    pub signing_scheme: SigningScheme,
    pub signature: Bytes,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Kind {
    Sell,
    Buy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Interaction {
    Liquidity(LiquidityInteraction),
    Custom(CustomInteraction),
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub struct Call {
    pub target: Address,
    pub value: U256,
    #[serde(rename = "callData")]
    pub calldata: Bytes,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidityInteraction {
    pub internalize: bool,
    pub id: String,
    pub input_token: Address,
    pub output_token: Address,
    pub input_amount: U256,
    pub output_amount: U256,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CustomInteraction {
    pub internalize: bool,
    pub target: Address,
    pub value: U256,
    #[serde(rename = "callData")]
    pub calldata: Bytes,
    pub allowances: Vec<Allowance>,
    pub inputs: Vec<Asset>,
    pub outputs: Vec<Asset>,
}

/// An interaction that can be executed as part of an order's pre- or
/// post-interactions.
#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInteraction {
    pub target: Address,
    pub value: U256,
    #[serde(rename = "callData")]
    pub calldata: Bytes,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub token: Address,
    pub amount: U256,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Allowance {
    pub token: Address,
    pub spender: Address,
    pub amount: U256,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SellTokenBalance {
    #[default]
    Erc20,
    Internal,
    External,
}

#[derive(Default, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum BuyTokenBalance {
    #[default]
    Erc20,
    Internal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SigningScheme {
    Eip712,
    EthSign,
    PreSign,
    Eip1271,
}
