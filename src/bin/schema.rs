use cosmwasm_schema::write_api;

use subscription::{
    execute_messages::msg::ExecuteMsg, instantiation::instantiate_messages::InstantiateMsg,
    migrate::migrate_messages::MigrateMsg, query::query_messages::QueryMsg,
};

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
        migrate:MigrateMsg,
    }
}
