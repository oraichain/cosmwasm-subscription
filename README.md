# cw20_elevated  

## Overview  

CW20 that has been extended to allow the minter to exectue some arbitrary changes to the state of balances.  
Uses a lighthouse as minter. Arbitrary changes include burn and transfer.  

## Dependencies  

Dependencies are the same as the base CW20 repository. 
We use version 0.8.X of the CW repository to enable compatibility with cosmwasm_std 0.16, which is the version of the std used by the Terra Blockchain. 

## How to use 

This uses the same methods as the cw20 token, you can refer to the cw20 documentation for details.  

Be careful some data must be set at instantiation time:   
- Minter address must be defined in Instantiate  
- Marketing data can only be changed by one allowed address which is defined independently from maintainer  


The additional messages, which can only be called by the minter, are:  

```rust

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteElevatedMsg {
    TransferToMinter { from: String, amount: Uint128},  
    Burn { from: String, amount: Uint128 },
}

```  

Typically the only message called in the Vault + cw20_elevated setup will be Burn.    


To call these messages, we have added a new ExecuteMsg entry:  

```rust  

#[derive(Serialize, Deserialize, Clone, PartialEq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    // EXTENSIONS
    ElevatedHook{elevated_msg: ExecuteElevatedMsg},
    ...

} 
```