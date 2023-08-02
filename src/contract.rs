#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::instantiation::instantiate_messages::InstantiateMsg;
use crate::instantiation::instantiation_execute::instantiate_contract;

use crate::execute::admin::dispatch_admin;
use crate::execute::default::dispatch_default;
use crate::execute_messages::msg::ExecuteMsg;

use crate::migrate::migrate_messages::MigrateMsg;
use crate::query::query_execute::{query_subscription_options, query_subscription_status};
use crate::query::query_messages::QueryMsg;

use crate::error::ContractError;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    return instantiate_contract(deps, info);
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Admin(admin_msg) => dispatch_admin(deps, env, info, admin_msg),
        _ => dispatch_default(deps, env, info, msg),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::SubscriptionStatus { addr } => {
            to_binary(&query_subscription_status(deps, env, addr)?)
        }
        QueryMsg::SubscriptionOptions {} => to_binary(&query_subscription_options(deps)?),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(_deps: DepsMut, _env: Env, _msg: MigrateMsg) -> StdResult<Response> {
    Ok(Response::default())
}
