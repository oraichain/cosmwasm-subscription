use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Never")]
    Never {},

    #[error("Unauthorized")]
    Unauthorized {},

    // payable
    #[error("Payable Contract")]
    PayableContract {},

    #[error("Single Currency Accepted")]
    SingleCurrencyPayable {},

    #[error("Funds amount invalid")]
    InvalidFundsAmount {},

    #[error("Invalid Funds Denomination")]
    InvalidFundsDenomination {},

    #[error("Subscription Option Does not exist")]
    InvalidSubcriptionOption {},

    #[error("No subscription available with given id not exist")]
    InvalidSubcriptionId {},
}
