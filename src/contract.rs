#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

use crate::error::ContractError;
use crate::query::{SubscriptionOptionsResponse, SubscriptionStatusResponse};
use crate::state::{PaymentOption, SubscriptionDuration, ADMIN, SUBSCRIPTION_OPTIONS};
use crate::{state_reads, state_writes};

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
        ExecuteMsg::Subscribe {
            subscription_option,
        } => execute_subscribe(deps, env, info, subscription_option),
        ExecuteMsg::AddSubscriptionOption {
            subscription_option,
        } => execute_add_subscription_option(deps, info, subscription_option),
    }
}

fn execute_add_subscription_option(
    deps: DepsMut,
    info: MessageInfo,
    subscription_option: PaymentOption,
) -> Result<Response, ContractError> {
    if !state_reads::is_admin(deps.as_ref(), info.sender)? {
        return Err(ContractError::Unauthorized {});
    }

    state_writes::add_subcription_option(deps, subscription_option)?;

    return Ok(Response::new());
}

fn execute_subscribe(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    subscription_option: SubscriptionDuration,
) -> Result<Response, ContractError> {
    let payment_option =
        state_reads::is_valid_subscription_option(deps.as_ref(), subscription_option)?;

    if payment_option.price == info.funds[0] {
        let _subscription_expiration =
            state_writes::update_subscription_status(deps, env, info.sender, payment_option)?;
        return Ok(Response::new());
    }

    return Err(ContractError::InvalidFundsAmount {});
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

fn query_subscription_options(deps: Deps) -> StdResult<SubscriptionOptionsResponse> {
    let rslt = state_reads::get_subscription_options(deps).unwrap();

    return Ok(rslt);
}

fn query_subscription_status(
    deps: Deps,
    env: Env,
    addr: String,
) -> StdResult<SubscriptionStatusResponse> {
    let addr = deps.api.addr_validate(&addr)?;

    let status = state_reads::get_subscription_status(deps, env, addr).unwrap();

    return Ok(status);
}
