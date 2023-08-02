use cosmwasm_schema::{cw_serde, QueryResponses};

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns(super::query_responses::SubscriptionOptionsResponse)]
    SubscriptionOptions {},
    #[returns(super::query_responses::SubscriptionStatusResponse)]
    SubscriptionStatus { addr: String },
}
