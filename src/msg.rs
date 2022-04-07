use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::state::{PaymentOption, SubscriptionDuration};

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Subscribe {
        subscription_option: SubscriptionDuration,
    },

    // admin
    AddSubscriptionOption {
        subscription_option: PaymentOption,
    },

    Withdraw {
        amount: String,
        denom: String,
        beneficiary: String,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    SubscriptionOptions {},
    SubcriptionStatus { addr: String },
}
