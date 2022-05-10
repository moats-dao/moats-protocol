#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Uint128, Uint256,
    QueryRequest, BalanceResponse, BankQuery,
    CosmosMsg, WasmMsg, WasmQuery, BankMsg, Coin
};

use cw2::set_contract_version;
use cw0::maybe_addr;
use cw20::{Cw20ExecuteMsg};

use cw_controllers::AdminResponse;
use schemars::_serde_json::de;

use crate::error::ContractError;
use crate::msg::{OwnerResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::msg_ancliqque::{
    ExecuteMsg as AncLiqQueExecuteMsg, QueryMsg as AncLiqQueQueryMsg,
    ConfigResponse, BidResponse, BidsResponse,
};
use crate::state::{ADMIN, HOOKS, State, STATE, BID_INDICES_MAP};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:liquidproof";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let safe_anc_liq_que_contract = deps.api.addr_validate(msg.anc_liq_que_contract.as_str())?;
    let safe_bluna_contract = deps.api.addr_validate(msg.bluna_contract.as_str())?;
    let safe_astroport_router = deps.api.addr_validate(msg.astroport_router.as_str())?;

    let state = State {
        owner: info.sender.clone(),
        anc_liq_que_contract: safe_anc_liq_que_contract,
        bluna_contract: safe_bluna_contract,
        astroport_router: safe_astroport_router,
        last_bid_idx: Uint128::from(0u128),
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    STATE.save(deps.storage, &state)?;

    let api = deps.api;
    ADMIN.set(deps, maybe_addr(api, Some(msg.admin))?)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateAdmin { new_admin } => try_update_admin(deps, info, new_admin),
        ExecuteMsg::SubmitBid { collateral_token, premium_slot } => try_submit_bid(deps, env, info, collateral_token, premium_slot),
        ExecuteMsg::ActivateBids { collateral_token, bids_idx } => try_activate_bid(deps, info, collateral_token, bids_idx),
        ExecuteMsg::RetractBid { bid_idx, amount } => try_retract_bid(deps, info, bid_idx, amount),
        ExecuteMsg::ClaimLiquidations { collateral_token, bids_idx } => try_claim_liquidation(deps, info, collateral_token, bids_idx),
    }
}

pub fn try_update_admin(deps: DepsMut, info: MessageInfo, new_admin: Addr) -> Result<Response, ContractError> {
    let api = deps.api;
    let result = api.addr_validate(&new_admin.as_str());
    if let Err(_e) = &result {
        return Err(ContractError::ArgumentError {});
    }

    let res = ADMIN.execute_update_admin(deps, info, Some(new_admin));

    match res {
        Ok(_) => Ok(Response::new().add_attribute("method", "try_update_admin")),
        Err(_) => Err(ContractError::ArgumentError {}),
    }
}

pub fn try_submit_bid(deps: DepsMut, env: Env, info: MessageInfo, collateral_token: String, premium_slot: u8) -> Result<Response, ContractError> {
    let api = deps.api;
    let result = api.addr_validate(&collateral_token.as_str());
    if let Err(_e) = &result {
        return Err(ContractError::ArgumentError {});
    }

    let mut state = STATE.load(deps.storage)?;

    let response = Response::new()
        .add_messages(vec![CosmosMsg::Wasm(
            WasmMsg::Execute {
                contract_addr: String::from(state.anc_liq_que_contract),
                msg: to_binary(&AncLiqQueExecuteMsg::SubmitBid {
                    collateral_token: collateral_token.clone(), premium_slot: premium_slot
                }).unwrap(),
                funds: info.funds
        })])
        .add_attribute("action", "deposit to project");

    let bids_by_user = if !state.last_bid_idx.is_zero() {
        query_bids_by_contract(
            deps.as_ref(), env, collateral_token.clone(), Some(state.last_bid_idx), None
        )
    } else {
        query_bids_by_contract(
            deps.as_ref(), env, collateral_token.clone(), None, None
        )
    }?;

    let iter_bid_indices_after_submit = bids_by_user.bids.iter().map(|bid_response| bid_response.idx);
    let bid_indices_after_submit = iter_bid_indices_after_submit.collect::<Vec<Uint128>>();
    match bid_indices_after_submit.last() {
        Some(last_bid_idx) => {
            let key = (collateral_token.clone(), &info.sender);
            if BID_INDICES_MAP.has(deps.storage, key.clone()) {
                let mut bid_indices = BID_INDICES_MAP.load(deps.storage, key.clone())?;
                bid_indices.push(*last_bid_idx);
            } else {
                BID_INDICES_MAP.save(
                    deps.storage,
                    key,
                    &vec![*last_bid_idx],
                )?;
            }

            state.last_bid_idx = *last_bid_idx;
        },
        None => { },
    }

    Ok(response)
}

pub fn try_activate_bid(deps: DepsMut, info: MessageInfo, collateral_token: String, bids_idx: Option<Vec<Uint128>>) -> Result<Response, ContractError> {
    let api = deps.api;
    let result = api.addr_validate(&collateral_token.as_str());
    if let Err(_e) = &result {
        return Err(ContractError::ArgumentError {});
    }

    let state = STATE.load(deps.storage)?;

    Ok(Response::new()
        .add_messages(vec![CosmosMsg::Wasm(
            WasmMsg::Execute {
                contract_addr: String::from(state.anc_liq_que_contract),
                msg: to_binary(&AncLiqQueExecuteMsg::ActivateBids {
                    collateral_token: collateral_token, bids_idx: bids_idx
                }).unwrap(),
                funds: vec![], 
        })])
        .add_attribute("action", "activate bid"))
}

pub fn try_retract_bid(deps: DepsMut, info: MessageInfo, bid_idx: Uint128, amount: Option<Uint256>) -> Result<Response, ContractError> {
    let api = deps.api;

    let state = STATE.load(deps.storage)?;

    Ok(Response::new()
        .add_messages(vec![CosmosMsg::Wasm(
            WasmMsg::Execute {
                contract_addr: String::from(state.anc_liq_que_contract),
                msg: to_binary(&AncLiqQueExecuteMsg::RetractBid {
                    bid_idx: bid_idx, amount: amount
                }).unwrap(),
                funds: vec![],
        })])
        .add_attribute("action", "deposit to project"))
}

pub fn try_claim_liquidation(deps: DepsMut, info: MessageInfo, collateral_token: String, bids_idx: Option<Vec<Uint128>>) -> Result<Response, ContractError> {
    let api = deps.api;
    let result = api.addr_validate(&collateral_token.as_str());
    if let Err(_e) = &result {
        return Err(ContractError::ArgumentError {});
    }

    let state = STATE.load(deps.storage)?;

    Ok(Response::new()
        .add_messages(vec![CosmosMsg::Wasm(
            WasmMsg::Execute {
                contract_addr: String::from(state.anc_liq_que_contract),
                msg: to_binary(&AncLiqQueExecuteMsg::ClaimLiquidations {
                    collateral_token: collateral_token, bids_idx: bids_idx
                }).unwrap(),
                funds: vec![],
        })])
        .add_attribute("action", "claim liquidation"))
}

// function for withdrawing bLuna from contract balance to specified address/wallet
pub fn withdraw_bluna(deps: DepsMut) -> Result<Response, ContractError>{
    let state = STATE.load(deps.storage)?;

    let bluna_token_addr = state.bluna_contract.to_string(); // bluna token addr

    let bluna_withdraw_addr = "".to_string(); // specify address later on

    let msg = Cw20ExecuteMsg::Transfer {
        recipient: bluna_withdraw_addr,
        amount: Uint128::from(10_000_000u128), // specify amount later on
    };

    Ok(
        Response::new()
        .add_message(
            CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: bluna_token_addr,
            msg: to_binary(&msg)?,
            funds: vec![],
            })
        )
    )
}



#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
        QueryMsg::GetAdmin {} =>  to_binary(&query_admin(deps)?),
        //QueryMsg::GetUstBalance{} => to_binary(&query_balance(deps, _env.contract.address, "uusd".to_string())?),
        QueryMsg::GetUstBalance { account_addr } => to_binary(&query_balance(deps, account_addr, "uusd".to_string())?),
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetBidInfo { bid_idx } => to_binary(&query_bid_info(deps, bid_idx)?),
        QueryMsg::GetBidsByContract { collateral_token, start_after, limit } =>
            to_binary(&query_bids_by_contract(
                deps, env, collateral_token, start_after, limit
            )?),
        QueryMsg::GetBidsByUser { collateral_token, bidder, start_after, limit } =>
            to_binary(&query_bids_by_user(
                deps, env, collateral_token, bidder, start_after, limit
            )?),
    }
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let state = STATE.load(deps.storage)?;

    Ok(OwnerResponse { owner: state.owner })
}

fn query_admin(deps: Deps) -> StdResult<AdminResponse> {
    ADMIN.query_admin(deps)
}

fn query_balance(deps: Deps, account_addr: Addr, denom: String) -> StdResult<Uint128> {
    let balance: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: account_addr.to_string(),
        denom,
    }))?;

    println!("{}: {} {}", account_addr.to_string(), balance.amount.denom, balance.amount.amount.to_string());
    Ok(balance.amount.amount.into())
}

fn query_config(deps: Deps) -> StdResult<ConfigResponse> {
    let state = STATE.load(deps.storage)?;

    let config_response = deps.querier
        .query::<ConfigResponse>(&QueryRequest::Wasm(
            WasmQuery::Smart {
                contract_addr: state.anc_liq_que_contract.to_string(),
                msg: to_binary(&AncLiqQueQueryMsg::Config {})?,
            },
        ))?;
    
    Ok(config_response)
}

fn query_bid_info(deps: Deps, bid_idx: Uint128) -> StdResult<BidResponse> {
    let state = STATE.load(deps.storage)?;

    let bid_response = deps.querier
        .query::<BidResponse>(&QueryRequest::Wasm(
            WasmQuery::Smart {
                contract_addr: state.anc_liq_que_contract.to_string(),
                msg: to_binary(&AncLiqQueQueryMsg::Bid {
                    bid_idx: bid_idx
                })?,
            },
        ))?;
    
    Ok(bid_response)
}

fn query_bids_by_contract(
    deps: Deps,
    env: Env,
    collateral_token: String,
    start_after: Option<Uint128>, 
    limit: Option<u8>
) -> StdResult<BidsResponse> {
    let state = STATE.load(deps.storage)?;

    let bids_response = deps.querier
        .query::<BidsResponse>(&QueryRequest::Wasm(
            WasmQuery::Smart {
                contract_addr: state.anc_liq_que_contract.to_string(),
                msg: to_binary(&AncLiqQueQueryMsg::BidsByUser {
                    collateral_token: collateral_token,
                    bidder: env.contract.address.to_string(),
                    start_after: start_after,
                    limit: limit
                })?,
            },
        ))?;
    
    Ok(bids_response)
}

fn query_bids_by_user(
    deps: Deps,
    env: Env,
    collateral_token: String,
    bidder: String, 
    start_after: Option<Uint128>, 
    limit: Option<u8>
) -> StdResult<BidsResponse> {
    let state = STATE.load(deps.storage)?;

    let safe_bidder = deps.api.addr_validate(&bidder)?;

    let bids_response = query_bids_by_contract(deps, env, collateral_token.clone(), start_after, limit)?;

    let key = (collateral_token.clone(), &safe_bidder);
    if BID_INDICES_MAP.has(deps.storage, key.clone()) {
        let bid_indices = BID_INDICES_MAP.load(deps.storage, key.clone())?;
        let mut bids_by_user : Vec<BidResponse> = vec![];
        for bid_response in bids_response.bids.iter() {
            if bid_indices.contains(&bid_response.idx) {
                bids_by_user.push(bid_response.clone());
            }
        }

        Ok(BidsResponse { bids: bids_by_user })
    } else {
        Ok(BidsResponse { bids: vec![] })
    }
}