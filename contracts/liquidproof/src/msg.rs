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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetOwner {},
    GetAdmin {},
    // GetUstBalance returns the current UST balance as a json-encoded number
    GetUstBalance { account_addr: Addr },
    GetBidInfo { bid_idx: Uint128 },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct OwnerResponse {
    pub owner: Addr,
}