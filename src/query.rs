use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::PaymentOption;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SubscriptionStatusResponse {
    pub is_valid: bool,
    pub expiration_timestamp: u64,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SubscriptionOptionsResponse {
    pub subscription_options: Vec<PaymentOption>,
}
