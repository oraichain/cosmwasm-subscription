use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Coin;

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DurationUnit {
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
        };

        return self.subscription_duration.amount_units * amount_days;
    }

    pub fn get_seconds_duration(&self) -> u64 {
        let amount_days: u64 = match self.subscription_duration.duration_unit {
            DurationUnit::Day => 1,
            DurationUnit::Week => 7,
            DurationUnit::Month => 30,
            DurationUnit::Year => 365,
        };

        // 24*60*60 seconds in a day: 86400
        return self.subscription_duration.amount_units * amount_days * 86400;
    }
}
