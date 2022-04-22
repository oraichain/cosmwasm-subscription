#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::execute::admin::dispatch_admin;
use crate::execute::default::dispatch_default;
use crate::msg::InstantiateMsg;

use crate::execute_messages::msg::ExecuteMsg;
use crate::query::query_messages::QueryMsg;

use crate::error::ContractError;

use crate::state::state_entries::{ADMIN, SUBSCRIPTION_OPTIONS};

use crate::query::query_execute::{query_subscription_options, query_subscription_status};

//use cw2::{set_contract_version, get_contract_version, ContractVersion};
use cw2::set_contract_version;

// version info for migration info
const CONTRACT_NAME: &str = "lightouse:vault";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ADMIN.save(deps.storage, &info.sender)?;
    SUBSCRIPTION_OPTIONS.save(deps.storage, &vec![])?;

    return Ok(Response::default());
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
        QueryMsg::SubcriptionStatus { addr } => {
            to_binary(&query_subscription_status(deps, env, addr)?)
        }
        QueryMsg::SubscriptionOptions {} => to_binary(&query_subscription_options(deps)?),
    }
}
