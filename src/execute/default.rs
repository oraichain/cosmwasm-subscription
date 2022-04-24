use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};

use crate::error::ContractError;
use crate::execute_messages::msg::ExecuteMsg;
use crate::state::{state_reads, state_writes};

pub fn dispatch_default(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Subscribe { id_subscription } => {
            try_subscribe(deps, env, info, id_subscription)
        }
        _ => Err(ContractError::Never {}),
    }
}

fn try_subscribe(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    id_subscription: u32,
) -> Result<Response, ContractError> {
    let subscription_option =
        state_reads::is_valid_subscription_option2(deps.as_ref(), id_subscription)?;

    if info.funds.len() > 1 {
        return Err(ContractError::SingleCurrencyPayable {});
    } else if info.funds.len() == 0 {
        return Err(ContractError::PayableContract {});
    }

    let payment_option = subscription_option.payment_option;
    if payment_option.price.denom != info.funds[0].denom {
        return Err(ContractError::InvalidFundsDenomination {});
    } else if payment_option.price == info.funds[0] {
        let _subscription_expiration =
            state_writes::update_subscription_status(deps, env, info.sender, payment_option)?;
        return Ok(Response::new());
    }

    return Err(ContractError::InvalidFundsAmount {});
}
