#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    QueryRequest, BalanceResponse, BankQuery, Uint128,
};

use cw2::set_contract_version;
use cw0::maybe_addr;

use cw_controllers::AdminResponse;

use crate::error::ContractError;
use crate::msg::{OwnerResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ADMIN, HOOKS, State, STATE};

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
    let state = State {
        owner: info.sender.clone(),
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
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateAdmin { new_admin } => try_update_admin(deps, info, new_admin),
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


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetOwner {} => to_binary(&query_owner(deps)?),
        QueryMsg::GetAdmin {} =>  to_binary(&query_admin(deps)?),
        //QueryMsg::GetUSTBalance{} => to_binary(&query_balance(deps, _env.contract.address, "uusd".to_string())?),
        QueryMsg::GetUSTBalance { account_addr } => to_binary(&query_balance(deps, account_addr, "uusd".to_string())?),
    }
}

pub fn query_balance(deps: Deps, account_addr: Addr, denom: String) -> StdResult<Uint128> {
    let balance: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: account_addr.to_string(),
        denom,
    }))?;
    println!("{}: {} {}", account_addr.to_string(), balance.amount.denom, balance.amount.amount.to_string());
    Ok(balance.amount.amount.into())
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OwnerResponse { owner: state.owner })
}

fn query_admin(deps: Deps) -> StdResult<AdminResponse> {
    ADMIN.query_admin(deps)
}