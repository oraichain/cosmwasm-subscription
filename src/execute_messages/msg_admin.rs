use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::structs::PaymentOption;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AdminExecuteMsg {
    // admin
    AddSubscriptionOption {
        subscription_option: PaymentOption,
    },

    RemoveSubscriptionOption {
        subscription_option: PaymentOption,
    },

    Withdraw {
        amount: String,
        denom: String,
        beneficiary: String,
    },
}
