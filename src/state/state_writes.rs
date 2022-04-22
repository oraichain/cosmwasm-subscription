use cosmwasm_std::{Addr, DepsMut, Env};

use crate::state::state_entries::{SUBSCRIPTIONS, SUBSCRIPTION_OPTIONS};
use crate::structs::PaymentOption;

use crate::ContractError;

pub fn add_subcription_option(
    deps: DepsMut,
    subscription_option: PaymentOption,
) -> Result<(), ContractError> {
    SUBSCRIPTION_OPTIONS.update(
        deps.storage,
        |mut subscription_options| -> Result<_, ContractError> {
            subscription_options.push(subscription_option);

            return Ok(subscription_options);
        },
    )?;

    return Ok(());
}

pub fn update_subscription_status(
    deps: DepsMut,
    env: Env,
    payer: Addr,
    payment_option: PaymentOption,
) -> Result<u64, ContractError> {
    let curr_subscription = SUBSCRIPTIONS.may_load(deps.storage, payer.clone())?;
    let subscription_duration = payment_option.get_seconds_duration();

    match curr_subscription {
        None => _new_subscription(deps, env, payer, subscription_duration),
        Some(value) => {
            if value < env.block.time.seconds() {
                return _new_subscription(deps, env, payer, subscription_duration);
            } else {
                return _lengthen_subscription(deps, payer, subscription_duration);
            }
        }
    }
}

fn _lengthen_subscription(
    deps: DepsMut,
    payer: Addr,
    subscription_duration: u64,
) -> Result<u64, ContractError> {
    let expiration_time = SUBSCRIPTIONS.load(deps.storage, payer.clone())? + subscription_duration;
    SUBSCRIPTIONS.save(deps.storage, payer, &expiration_time)?;

    return Ok(expiration_time);
}

fn _new_subscription(
    deps: DepsMut,
    env: Env,
    payer: Addr,
    subscription_duration: u64,
) -> Result<u64, ContractError> {
    let expiration_time = env.block.time.seconds() + subscription_duration;
    SUBSCRIPTIONS.save(deps.storage, payer, &expiration_time)?;

    return Ok(expiration_time);
}
