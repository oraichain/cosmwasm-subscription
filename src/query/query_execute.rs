use cosmwasm_std::{Deps, Env, StdResult};

use crate::query::query_responses::{SubscriptionOptionsResponse, SubscriptionStatusResponse};
use crate::state::state_reads;

pub fn query_subscription_options(deps: Deps) -> StdResult<SubscriptionOptionsResponse> {
    let rslt = state_reads::get_subscription_options(deps).unwrap();

    return Ok(rslt);
}

pub fn query_subscription_status(
    deps: Deps,
    env: Env,
    addr: String,
) -> StdResult<SubscriptionStatusResponse> {
    let addr = deps.api.addr_validate(&addr)?;

    let status = state_reads::get_subscription_status(deps, env, addr).unwrap();

    return Ok(status);
}
