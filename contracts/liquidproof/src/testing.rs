use cosmwasm_std::{
    to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
    QueryRequest, BalanceResponse, BankQuery, Uint128,
    OwnedDeps,
};

use cw2::set_contract_version;
use cw0::maybe_addr;

use cw_controllers::AdminResponse;

use crate::contract::{instantiate, execute, query};
use crate::error::ContractError;
use crate::msg::{OwnerResponse, ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::state::{ADMIN, HOOKS, State, STATE};

use cosmwasm_std::testing::{
    mock_dependencies, mock_env, mock_info, MockApi, MockStorage, MockQuerier, MOCK_CONTRACT_ADDR
};
use cosmwasm_std::{coins, from_binary};

fn setup_test(info: &MessageInfo) -> OwnedDeps<MockStorage, MockApi, MockQuerier> {
    let mut deps = mock_dependencies(&[]);

    let msg = InstantiateMsg {
        admin: "admin".to_string(),
        anc_liq_que_contract: "terra18j0wd0f62afcugw2rx5y8e6j5qjxd7d6qsc87r".to_string(),
        bluna_contract: "terra1u0t35drzyy0mujj8rkdyzhe264uls4ug3wdp3x".to_string(),
        astroport_router: "terra13wf295fj9u209nknz2cgqmmna7ry3d3j5kv7t4".to_string(),
    };

    // we can just call .unwrap() to assert this was a success
    let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
    assert_eq!(0, res.messages.len());

    deps
}

#[test]
fn proper_initialization() {
    let info = mock_info("creator", &coins(2, "token"));
    let sender = info.sender.clone();
    let mut deps = setup_test(&info);

    let old_admin_addr = Addr::unchecked("admin");

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

#[test]
fn balance_related() {
    let info = mock_info("creator", &coins(99, "uusd"));
    let sender = info.sender.clone();
    let mut deps = setup_test(&info);
    assert_eq!(info.funds, coins(99, "uusd"));

    // // query the UST balance (NOT WORKING)
    // let msg = QueryMsg::GetUstBalance { account_addr: sender };
    // let res = query(deps.as_ref(), mock_env(), msg).unwrap();
    // let bal: Uint128 = from_binary(&res).unwrap();
    // assert_eq!(Uint128::from(99u128), bal);
}