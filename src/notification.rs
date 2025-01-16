use alloy_primitives::{Address, Bytes, B256, U256};
use alloy_rpc_types::AccessList;
use serde::Serialize;
use {
    serde::Deserialize,
    serde_with::{serde_as, DisplayFromStr},
    std::collections::BTreeSet,
};

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Notification {
    #[serde_as(as = "Option<DisplayFromStr>")]
    pub auction_id: Option<i64>,
    pub solution_id: Option<SolutionId>,
    #[serde(flatten)]
    pub kind: Kind,
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SolutionId {
    Single(u64),
    Merged(Vec<u64>),
}

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", tag = "kind")]
pub enum Kind {
    Timeout,
    EmptySolution,
    DuplicatedSolutionId,
    #[serde(rename_all = "camelCase")]
    SimulationFailed {
        block: BlockNo,
        tx: Tx,
        succeeded_once: bool,
    },
    InvalidClearingPrices,
    #[serde(rename_all = "camelCase")]
    MissingPrice {
        token_address: Address,
    },
    InvalidExecutedAmount,
    NonBufferableTokensUsed {
        tokens: BTreeSet<Address>,
    },
    SolverAccountInsufficientBalance {
        required: U256,
    },
    Success {
        transaction: B256,
    },
    Revert {
        transaction: B256,
    },
    DriverError {
        reason: String,
    },
    Cancelled,
    Fail,
    PostprocessingTimedOut,
}

type BlockNo = u64;

#[serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tx {
    pub from: Address,
    pub to: Address,
    pub input: Bytes,
    pub value: U256,
    pub access_list: AccessList,
}
