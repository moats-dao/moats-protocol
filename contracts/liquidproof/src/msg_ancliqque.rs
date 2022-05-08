//https://docs.anchorprotocol.com/smart-contracts/liquidations/liquidation-queue-contract

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128, Uint256, Decimal256};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    SubmitBid {
        collateral_token: String, 
        premium_slot: u8, 
    },
    ActivateBids {
        collateral_token: String,
        bids_idx: Option<Vec<Uint128>>, 
    },
    ClaimLiquidations {
        collateral_token: String,
        bids_idx: Option<Vec<Uint128>>,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Bid {
        bid_idx: Uint128, 
    },
    BidsByUser {
        collateral_token: String,
        bidder: String, 
        start_after: Option<Uint128>, 
        limit: Option<u8>, 
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BidResponse {
    pub idx: Uint128,
    pub collateral_token: String,
    pub premium_slot: u8,
    pub bidder: String,
    pub amount: Uint256,
    pub product_snapshot: Decimal256,
    pub sum_snapshot: Decimal256,
    pub pending_liquidated_collateral: Uint256,
    pub wait_end: Option<u64>,
    pub epoch_snapshot: Uint128,
    pub scale_snapshot: Uint128,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct BidsResponse {
    pub bids: Vec<BidResponse>, 
}