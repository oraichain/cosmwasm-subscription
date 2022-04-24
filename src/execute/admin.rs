use std::str::FromStr;

use cosmwasm_std::{BankMsg, Coin, CosmosMsg, DepsMut, Env, MessageInfo, Response, Uint128};

use crate::execute_messages::msg_admin::AdminExecuteMsg;

use crate::error::ContractError;
use crate::state::{state_reads, state_writes};
use crate::structs::PaymentOption;

pub fn dispatch_admin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    admin_msg: AdminExecuteMsg,
) -> Result<Response, ContractError> {
    if !state_reads::is_contract_admin(deps.as_ref(), info.sender.clone())? {
        return Err(ContractError::Unauthorized {});
    }

    match admin_msg {
        AdminExecuteMsg::Withdraw {
            denom,
            amount,
            beneficiary,
        } => try_withdraw(denom, amount, beneficiary),
        AdminExecuteMsg::AddSubscriptionOption { payment_option } => {
            try_add_subscription_option(deps, payment_option)
        }
        AdminExecuteMsg::RemoveSubscriptionOption { id_to_remove } => {
            try_remove_subscription_option(deps, id_to_remove)
        }
    }
}

fn try_add_subscription_option(
    deps: DepsMut,
    payment_option: PaymentOption,
) -> Result<Response, ContractError> {
    let curr_id = state_reads::get_subscription_id_tracker(deps.as_ref())?;
    state_writes::add_subcription_option(deps, payment_option, curr_id)?;

    return Ok(Response::new());
}

fn try_remove_subscription_option(
    deps: DepsMut,
    //subscription_option: PaymentOption,
    id_to_remove: u32,
) -> Result<Response, ContractError> {
    state_writes::remove_subcription_option(deps, id_to_remove)?;

    return Ok(Response::new());
}

fn try_withdraw(
    denom: String,
    amount: String,
    beneficiary: String,
) -> Result<Response, ContractError> {
    let coin = Coin {
        denom,
        amount: Uint128::from_str(amount.as_str()).unwrap(),
    };

    let bank_msg = BankMsg::Send {
        to_address: beneficiary,
        amount: vec![coin],
    };
    let cosmos_msg = CosmosMsg::Bank(bank_msg);

    return Ok(Response::new().add_message(cosmos_msg));
}
