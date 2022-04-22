# Subscription    

## Overview  

Contract that implements a subscription system. Admin defines subscription options for a given price.   

## Subscription Options  

Subscription options are defined by the following enums and structs:  

```rust 
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
```

This gives the admin flexibility in defining subscription durations.   
The contract also handles the definition of multiple subscription options, which means it is possible to create discounts for longer term subscriptions.  


## Improvements  
For now, the contract only handles lump sum payments, might be possible to create some kind of recurring payment instead. Using allowances?  




