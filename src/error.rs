use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    // payable
    #[error("Payable Contract")]
    RequiresFunds {},

    #[error("Single Currency Accepted")]
    SingleCurrencyPayable {},

    #[error("Funds amount invalid")]
    InvalidFundsAmount {},

    #[error("Subscription Option Does not exist")]
    InvalidSubcriptionOption {},
}
