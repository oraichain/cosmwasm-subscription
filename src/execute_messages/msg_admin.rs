use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::structs::PaymentOption;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum AdminExecuteMsg {
    // admin
    AddSubscriptionOption {
        payment_option: PaymentOption,
    },

    RemoveSubscriptionOption {
        //subscription_option: PaymentOption,
        id_to_remove: u32,
    },

    Withdraw {
        amount: String,
        denom: String,
        beneficiary: String,
    },
}
