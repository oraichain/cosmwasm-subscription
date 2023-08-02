use cosmwasm_schema::cw_serde;

use crate::structs::PaymentOption;

#[cw_serde]
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
