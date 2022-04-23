use cosmwasm_std::{DepsMut, MessageInfo, Response};
use cw2::set_contract_version;

use crate::state::state_entries::{ADMIN, SUBSCRIPTION_OPTIONS};
use crate::ContractError;

// version info for migration info
const CONTRACT_NAME: &str = "azoyalabs:subscription";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn instantiate_contract(deps: DepsMut, info: MessageInfo) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    ADMIN.save(deps.storage, &info.sender)?;
    SUBSCRIPTION_OPTIONS.save(deps.storage, &vec![])?;

    return Ok(Response::default());
}
