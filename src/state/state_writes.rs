use cosmwasm_std::{Addr, DepsMut, Env};

use crate::state::state_entries::{SUBSCRIPTIONS, SUBSCRIPTION_OPTIONS_RECORDS};
use crate::structs::{PaymentOption, SubscriptionOptionRecord};

use crate::ContractError;

use super::state_entries::SUBSCRIPTION_OPTIONS_ID_TRACKER;

pub fn add_subcription_option(
    deps: DepsMut,
    payment_option: PaymentOption,
    curr_id_record: u32,
) -> Result<(), ContractError> {
    let subscription_record = SubscriptionOptionRecord {
        id: curr_id_record,
        payment_option: payment_option,
    };

    SUBSCRIPTION_OPTIONS_RECORDS.update(
        deps.storage,
        |mut subscription_options| -> Result<_, ContractError> {
            subscription_options.push(subscription_record);

            return Ok(subscription_options);
        },
    )?;

    increment_tracking_id_subscription_options(deps)?;

    return Ok(());
}

pub fn remove_subcription_option(
    deps: DepsMut,
    //subscription_option: PaymentOption,
    target_id: u32,
) -> Result<(), ContractError> {
    SUBSCRIPTION_OPTIONS_RECORDS.update(
        deps.storage,
        |subscription_options| -> Result<_, ContractError> {
            //subscription_options.push(subscription_option);

            let subscription_options: Vec<SubscriptionOptionRecord> = subscription_options
                .into_iter()
                .filter(|elem| elem.id != target_id)
                .collect();

            return Ok(subscription_options);
        },
    )?;

    return Ok(());
}

pub fn increment_tracking_id_subscription_options(deps: DepsMut) -> Result<(), ContractError> {
    SUBSCRIPTION_OPTIONS_ID_TRACKER.update(deps.storage, |value| -> Result<_, ContractError> {
        return Ok(value + 1);
    })?;

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
