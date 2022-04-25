# Subscription    

## Overview  

Contract that implements a subscription system. Admin defines subscription options for a given price. These options are given an ID and registered as SubscriptionOptionRecord.  
User therefore call Subscribe{ id_subscription } with necessary funds to subscribe for a given duration.  

Contract is live on Testnet  
CodeId: 66741
Address: terra1cdt0gmccw3dlncaw7y7zj670kyy488mc4tnk4t    

See it on  
TerraFinder: https://finder.terra.money/testnet/address/terra1cdt0gmccw3dlncaw7y7zj670kyy488mc4tnk4t     
Terrascope:  https://terrasco.pe/testnet/address/terra1cdt0gmccw3dlncaw7y7zj670kyy488mc4tnk4t     



## How to Use  
Admin defines subscription durations   
```json
// example: 30 days for 1 UST  
{
    "admin": {
        "add_subscription_option": {
            "payment_option": {
                "subscription_duration": {
                    "amount_units": 30,
                    "duration_unit": "day"
                },
                "price": {
                    "denom": "uusd",
                    "amount": "5000000"
                }
            }
        }
    }
}

```

This subscription option will be given an ID. User can subscribe by referencing it, and sending the appropriate funds.  
This will set the user as subscribed, with an expiry at current timestamp + 30 days (since the subscription is for 30 days).      

```json
{
    "subscribe": {
        "id_subscription": 0
    }
}

```

The admin can also delete subscription options 

```json
{
    "admin": {
        "remove_subscription_option": {
            "id_to_remove": 0
        }
    }
}
```

See execute_messages folder for rust implementation of messages.  
Structs and enums are in structs.rs 

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

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
pub struct SubscriptionOptionRecord {
    pub id: u32,
    pub payment_option: PaymentOption,
}

```

This gives the admin flexibility in defining subscription durations.   
The contract also handles the definition of multiple subscription options, which means it is possible to create discounts for longer term subscriptions.  


## Improvements  
For now, the contract only handles lump sum payments, might be possible to create some kind of recurring payment instead. Using allowances?  




