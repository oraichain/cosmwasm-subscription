use cosmwasm_std::{Addr, Deps, Env};

use crate::query::query_responses::SubscriptionOptionsResponse;
use crate::state::state_entries::{ADMIN, SUBSCRIPTIONS, SUBSCRIPTION_OPTIONS};
use crate::structs::{PaymentOption, SubscriptionDuration};

use crate::{query::query_responses::SubscriptionStatusResponse, ContractError};

pub fn is_valid_subscription_option(
    deps: Deps,
    subscription_option: SubscriptionDuration,
) -> Result<PaymentOption, ContractError> {
    let subscription_options = SUBSCRIPTION_OPTIONS.load(deps.storage)?;

    for opt in subscription_options {
        if subscription_option == opt.subscription_duration {
            return Ok(opt);
        }
    }

    return Err(ContractError::InvalidSubcriptionOption {});
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
    let sub_options = SUBSCRIPTION_OPTIONS.load(deps.storage)?;

    return Ok(SubscriptionOptionsResponse {
        subscription_options: sub_options,
    });
}

pub fn is_contract_admin(deps: Deps, addr: Addr) -> Result<bool, ContractError> {
    let admin = ADMIN.load(deps.storage)?;

    return Ok(admin == addr);
}
