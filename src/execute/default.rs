use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::execute_messages::msg::ExecuteMsg;
use crate::state::{state_reads, state_writes};

use crate::structs::SubscriptionDuration;

pub fn dispatch_default(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Subscribe {
            subscription_option,
        } => try_subscribe(deps, env, info, subscription_option),
        _ => Err(ContractError::Never {}),
    }
}

fn try_subscribe(
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
