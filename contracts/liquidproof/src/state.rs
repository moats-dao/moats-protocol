use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{Addr, Uint128};
use cw_storage_plus::{Item, Map};
use cw_controllers::{Admin, Hooks};

pub const ADMIN: Admin = Admin::new("admin");
pub const HOOKS: Hooks = Hooks::new("hooks");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct State {
    pub owner: Addr,
    pub anc_liq_que_contract: Addr,
    pub bluna_contract: Addr,
    pub astroport_router: Addr,
    pub last_bid_idx: Uint128,
}

pub const STATE: Item<State> = Item::new("state");

// Customer Info
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct CustInfo {
    pub wallet_addr: Addr,
    pub ust_bal: Uint128,
}

pub const CUST_INFO_MAP: Map<&Addr, CustInfo> = Map::new("cust_info_map");

pub const BID_INDICES_MAP: Map<(String, &Addr), Vec<Uint128>> = Map::new("bid_indices_map");