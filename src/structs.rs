use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Coin;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DurationUnit {
    Second,
    Minute,
    Hour,
    Day,
    Week,
    Month,
    Year,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SubscriptionDuration {
    pub amount_units: u64,
    pub duration_unit: DurationUnit,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SubscriptionOptionRecord {
    pub id: u32,
    pub payment_option: PaymentOption,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct PaymentOption {
    pub subscription_duration: SubscriptionDuration,
    pub price: Coin,
}

impl PaymentOption {
    pub fn get_day_duration(&self) -> u64 {
        let amount_days: u64 = match self.subscription_duration.duration_unit {
            DurationUnit::Day => 1,
            DurationUnit::Week => 7,
            DurationUnit::Month => 30,
            DurationUnit::Year => 365,
            _ => 0,
        };

        return self.subscription_duration.amount_units * amount_days;
    }

    pub fn get_seconds_duration(&self) -> u64 {
        let amount_seconds: u64 = match self.subscription_duration.duration_unit {
            DurationUnit::Second => 1,
            DurationUnit::Minute => 60,
            DurationUnit::Hour => 3600,
            // 24*60*60 seconds in a day: 86400
            DurationUnit::Day => 1 * 86400,
            DurationUnit::Week => 7 * 86400,
            DurationUnit::Month => 30 * 86400,
            DurationUnit::Year => 365 * 86400,
        };

        return self.subscription_duration.amount_units * amount_seconds;
    }
}
