use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::structs::SubscriptionOptionRecord;

pub const ADMIN: Item<Addr> = Item::new("admin");

pub const SUBSCRIPTIONS: Map<Addr, u64> = Map::new("subscriptions");

pub const SUBSCRIPTION_OPTIONS_ID_TRACKER: Item<u32> = Item::new("subscription_options_id_tracker");
pub const SUBSCRIPTION_OPTIONS_RECORDS: Item<Vec<SubscriptionOptionRecord>> =
    Item::new("subscription_options_records");
