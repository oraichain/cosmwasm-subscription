use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

use crate::structs::PaymentOption;

pub const ADMIN: Item<Addr> = Item::new("admin");

pub const SUBSCRIPTIONS: Map<Addr, u64> = Map::new("subscriptions");
pub const SUBSCRIPTION_OPTIONS: Item<Vec<PaymentOption>> = Item::new("subscription_options");
