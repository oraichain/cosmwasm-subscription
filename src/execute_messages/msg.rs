use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use crate::structs::SubscriptionDuration;

use crate::execute_messages::msg_admin::AdminExecuteMsg;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    Subscribe {
        subscription_option: SubscriptionDuration,
    },

    Admin(AdminExecuteMsg),
}
