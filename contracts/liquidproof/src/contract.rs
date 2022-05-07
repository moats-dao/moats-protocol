#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    QueryRequest, BalanceResponse, BankQuery, Uint256,
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
    }
}

pub fn query_balance(deps: Deps, account_addr: Addr, denom: String) -> StdResult<Uint256> {
    // load price form the oracle
    let balance: BalanceResponse = deps.querier.query(&QueryRequest::Bank(BankQuery::Balance {
        address: account_addr.to_string(),
        denom,
    }))?;
    Ok(balance.amount.amount.into())
}

fn query_owner(deps: Deps) -> StdResult<OwnerResponse> {
    let state = STATE.load(deps.storage)?;
    Ok(OwnerResponse { owner: state.owner })
}

fn query_admin(deps: Deps) -> StdResult<AdminResponse> {
    ADMIN.query_admin(deps)
}


#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies(&[]);

        let old_admin_addr = Addr::unchecked("admin");
        let msg = InstantiateMsg { admin: "admin".to_string() };
        //let info = mock_info("creator", &coins(1000, "earth"));
        let info = mock_info("creator", &coins(2, "token"));
        let sender = info.sender.clone();

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        // query the owner
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetOwner {}).unwrap();
        let value: OwnerResponse = from_binary(&res).unwrap();
        assert_eq!(sender, value.owner);

        // query the admin
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetAdmin {}).unwrap();
        let value: AdminResponse = from_binary(&res).unwrap();
        assert_eq!(old_admin_addr.clone(), Addr::unchecked(value.admin.clone().unwrap()));

        // update the admin
        //let new_admin_addr = Addr::unchecked("admin2");
        let new_admin_info =mock_info("admin2", &coins(2, "token"));
        //let msg = ExecuteMsg::UpdateAdmin { new_admin: new_admin_addr.clone() };
        let msg = ExecuteMsg::UpdateAdmin { new_admin: new_admin_info.sender.clone() };
        //let admin_info = mock_info("admin", &coins(1000, "earth"));
        let old_admin_info = mock_info("admin", &coins(2, "token"));
        let res = execute(deps.as_mut(), mock_env(), old_admin_info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // query the admin
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetAdmin {}).unwrap();
        let value = from_binary::<AdminResponse>(&res).unwrap();
        assert_eq!(new_admin_info.sender.clone(), Addr::unchecked(value.admin.clone().unwrap()));
    }
}
