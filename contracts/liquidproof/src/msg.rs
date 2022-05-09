use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admin: String,
    pub anc_liq_que_contract: String,
    pub bluna_contract: String,
    pub astroport_router: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    UpdateAdmin { new_admin: Addr },
    SubmitBid { collateral_token: String, premium_slot: u8 },
    ClaimLiquidations { collateral_token: String, bids_idx: Option<Vec<Uint128>> },
    ActivateBids { collateral_token: String, bids_idx: Option<Vec<Uint128>> },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOwner {},
    GetAdmin {},
    // GetUstBalance returns the current UST balance as a json-encoded number
    GetUstBalance { account_addr: Addr },
    GetBidInfo { bid_idx: Uint128 },
    GetBidsByUser {
        collateral_token: String,
        bidder: String, 
        start_after: Option<Uint128>, 
        limit: Option<u8>
    },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
    pub owner: Addr,
}