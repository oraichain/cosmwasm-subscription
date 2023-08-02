use cosmwasm_schema::cw_serde;

use crate::execute_messages::msg_admin::AdminExecuteMsg;

#[cw_serde]
pub enum ExecuteMsg {
    Subscribe { id_subscription: u32 },

    Admin(AdminExecuteMsg),
}
