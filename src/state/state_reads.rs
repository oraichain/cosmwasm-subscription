use cosmwasm_std::{Addr, Deps, Env};

use crate::query::query_responses::SubscriptionOptionsResponse;
use crate::state::state_entries::{ADMIN, SUBSCRIPTIONS, SUBSCRIPTION_OPTIONS_RECORDS};
use crate::structs::{SubscriptionDuration, SubscriptionOptionRecord};

use crate::{query::query_responses::SubscriptionStatusResponse, ContractError};

use super::state_entries::SUBSCRIPTION_OPTIONS_ID_TRACKER;

pub fn is_valid_subscription_option(
    deps: Deps,
    subscription_option: SubscriptionDuration,
) -> Result<SubscriptionOptionRecord, ContractError> {
    let subscription_options = SUBSCRIPTION_OPTIONS_RECORDS.load(deps.storage)?;

    for opt in subscription_options {
        if subscription_option == opt.payment_option.subscription_duration {
            return Ok(opt);
        }
    }

    return Err(ContractError::InvalidSubcriptionOption {});
}

pub fn get_subscription_id_tracker(deps: Deps) -> Result<u32, ContractError> {
    let curr_id = SUBSCRIPTION_OPTIONS_ID_TRACKER.load(deps.storage)?;

    return Ok(curr_id);
}

pub fn is_valid_subscription_option2(
    deps: Deps,
    id_subscription: u32,
) -> Result<SubscriptionOptionRecord, ContractError> {
    let subscription_options = SUBSCRIPTION_OPTIONS_RECORDS.load(deps.storage)?;

    for opt in subscription_options {
        if id_subscription == opt.id {
            return Ok(opt);
        }
    }

    return Err(ContractError::InvalidSubcriptionId {});
}

pub fn get_subscription_status(
    deps: Deps,
    env: Env,
    addr: Addr,
) -> Result<SubscriptionStatusResponse, ContractError> {
    let curr_val = SUBSCRIPTIONS.may_load(deps.storage, addr)?;

    match curr_val {
        None => {
            let resp = SubscriptionStatusResponse {
                is_valid: false,
                expiration_timestamp: 0,
            };
            return Ok(resp);
        }
        Some(timestamp) => {
            let resp = SubscriptionStatusResponse {
                is_valid: timestamp > env.block.time.seconds(),
                expiration_timestamp: timestamp,
            };
            return Ok(resp);
        }
    }
}

pub fn get_subscription_options(deps: Deps) -> Result<SubscriptionOptionsResponse, ContractError> {
    let sub_options = SUBSCRIPTION_OPTIONS_RECORDS.load(deps.storage)?;

    return Ok(SubscriptionOptionsResponse {
        subscription_options: sub_options,
    });
}

pub fn is_contract_admin(deps: Deps, addr: Addr) -> Result<bool, ContractError> {
    let admin = ADMIN.load(deps.storage)?;

    return Ok(admin == addr);
}
