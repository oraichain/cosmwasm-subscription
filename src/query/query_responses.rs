use cosmwasm_schema::cw_serde;

use crate::structs::SubscriptionOptionRecord;

#[cw_serde]
pub struct SubscriptionStatusResponse {
    pub is_valid: bool,
    pub expiration_timestamp: u64,
}

#[cw_serde]
pub struct SubscriptionOptionsResponse {
    pub subscription_options: Vec<SubscriptionOptionRecord>,
}
