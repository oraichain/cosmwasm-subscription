#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{
        mock_dependencies_with_balance, mock_env, mock_info, MockApi, MockQuerier,
    };
    use cosmwasm_std::{
        coins, from_binary, Coin, DepsMut, MemoryStorage, OwnedDeps, Response, Uint128,
    };

    use crate::error::ContractError;
    use crate::execute_messages::msg::ExecuteMsg;
    use crate::execute_messages::msg_admin::AdminExecuteMsg;
    use crate::instantiation::instantiate_messages::InstantiateMsg;
    use crate::query::query_messages::QueryMsg;

    use crate::contract::{execute, instantiate, query};
    use crate::query::query_responses::{SubscriptionOptionsResponse, SubscriptionStatusResponse};
    use crate::structs::{DurationUnit, PaymentOption, SubscriptionDuration};

    const TEST_DENOM: &str = "uusd";
    const TEST_CREATOR: &str = "creator";
    const TEST_USER: &str = "user";
    const TEST_USER2: &str = "user2";

    const TEST_PRICE: u64 = 10000000;

    const TEST_DURATION_UNIT: DurationUnit = DurationUnit::Day;
    const TEST_DURATION_UNIT_AMOUNT: u64 = 1;

    const TEST_INVALID_DENOM: &str = "notuusd";

    fn instantiate_contract() -> OwnedDeps<MemoryStorage, MockApi, MockQuerier> {
        let mut deps = mock_dependencies_with_balance(&[Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(100u64),
        }]);
        let msg = InstantiateMsg {};
        let info = mock_info(TEST_CREATOR, &coins(1000, TEST_DENOM));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        return deps;
    }

    fn add_default_subscription_option(deps: DepsMut) -> Result<Response, ContractError> {
        let subscription_duration = SubscriptionDuration {
            duration_unit: TEST_DURATION_UNIT, //DurationUnit::Day,
            amount_units: TEST_DURATION_UNIT_AMOUNT,
        };

        let subscription_option = PaymentOption {
            subscription_duration: subscription_duration,
            price: Coin {
                denom: TEST_DENOM.to_string(),
                amount: Uint128::from(TEST_PRICE),
            },
        };

        let temp = AdminExecuteMsg::AddSubscriptionOption {
            payment_option: subscription_option,
        };
        let msg = ExecuteMsg::Admin(temp);

        let info = mock_info(TEST_CREATOR, &[]); //&coins(1000, TEST_DENOM));

        return execute(deps, mock_env(), info, msg);
    }

    fn subscribe(
        deps: DepsMut,
        subscriber: &str,
        id_subscription: u32,
        funds: Coin,
    ) -> Result<Response, ContractError> {
        let msg = ExecuteMsg::Subscribe {
            //subscription_option: subscription_duration,
            id_subscription: id_subscription,
        };

        let info = mock_info(subscriber, &[funds]); //&coins(1000, TEST_DENOM));

        return execute(deps, mock_env(), info, msg);
    }

    #[test]
    fn instantiate_success() {
        let mut _deps = instantiate_contract();
    }

    #[test]
    fn add_subcription_option_successful() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();
    }

    #[test]
    fn add_subcription_option_rejected_unauthorized() {
        let mut deps = instantiate_contract();
        let subscription_duration = SubscriptionDuration {
            duration_unit: TEST_DURATION_UNIT, //DurationUnit::Day,
            amount_units: TEST_DURATION_UNIT_AMOUNT,
        };

        let subscription_option = PaymentOption {
            subscription_duration: subscription_duration,
            price: Coin {
                denom: TEST_DENOM.to_string(),
                amount: Uint128::from(TEST_PRICE),
            },
        };

        let temp = AdminExecuteMsg::AddSubscriptionOption {
            payment_option: subscription_option,
        };
        let msg = ExecuteMsg::Admin(temp);

        let info = mock_info(TEST_USER2, &[]); //&coins(1000, TEST_DENOM));

        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

        assert_eq!(res, ContractError::Unauthorized {});
    }

    #[test]
    fn subscribe_successful() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();

        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let _res = subscribe(deps.as_mut(), TEST_CREATOR, 0, funds).unwrap();

        let msg = QueryMsg::SubscriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let _data: SubscriptionStatusResponse = from_binary(&res).unwrap();
    }

    #[test]
    fn subscribe_rejected_invalid_currency() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();

        let funds = Coin {
            denom: TEST_INVALID_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let res = subscribe(deps.as_mut(), TEST_CREATOR, 0, funds).unwrap_err();

        assert_eq!(res, ContractError::InvalidFundsDenomination {});
    }

    #[test]
    fn subscribe_rejected_invalid_funds_amoun() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();

        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE + 1000),
        };

        let res = subscribe(deps.as_mut(), TEST_CREATOR, 0, funds).unwrap_err();

        assert_eq!(res, ContractError::InvalidFundsAmount {});

        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE - 1000),
        };

        let res = subscribe(deps.as_mut(), TEST_CREATOR, 0, funds).unwrap_err();

        assert_eq!(res, ContractError::InvalidFundsAmount {});
    }

    #[test]
    fn subscribe_rejected_payable() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();

        let msg = ExecuteMsg::Subscribe {
            //subscription_option: subscription_duration,
            id_subscription: 0,
        };

        let info = mock_info(TEST_CREATOR, &[]); //&coins(1000, TEST_DENOM));
        let res = execute(deps.as_mut(), mock_env(), info, msg).unwrap_err();

        assert_eq!(res, ContractError::PayableContract {});
    }

    #[test]
    fn subscribe_expire() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();

        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let _res = subscribe(deps.as_mut(), TEST_CREATOR, 0, funds).unwrap();

        let msg = QueryMsg::SubscriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let _res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let mut env = mock_env();
        env.block.time = env.block.time.plus_seconds(100000);

        let msg = QueryMsg::SubscriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let res = query(deps.as_ref(), env, msg).unwrap();

        let data: SubscriptionStatusResponse = from_binary(&res).unwrap();

        assert!(!data.is_valid);
    }

    #[test]
    fn subscribe_error_wrong_id() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();

        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let res = subscribe(deps.as_mut(), TEST_CREATOR, 1, funds).unwrap_err();

        assert_eq!(res, ContractError::InvalidSubcriptionId {});
    }

    #[test]
    fn subscribe_lengthened() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();

        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let _res = subscribe(deps.as_mut(), TEST_CREATOR, 0, funds).unwrap();

        let msg = QueryMsg::SubscriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let _res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let mut env = mock_env();
        env.block.time = env.block.time.plus_seconds(100000);

        let msg = QueryMsg::SubscriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let res = query(deps.as_ref(), env.clone(), msg).unwrap();

        let data: SubscriptionStatusResponse = from_binary(&res).unwrap();

        assert!(!data.is_valid);

        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let _res = subscribe(deps.as_mut(), TEST_CREATOR, 0, funds).unwrap();

        let msg = QueryMsg::SubscriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let res = query(deps.as_ref(), env, msg).unwrap();

        let data: SubscriptionStatusResponse = from_binary(&res).unwrap();

        println!("{:?}", data);
        assert!(data.is_valid);
    }

    #[test]
    fn withdraw_successful() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();

        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let _res = subscribe(deps.as_mut(), TEST_USER, 0, funds).unwrap();

        let msg = ExecuteMsg::Admin(AdminExecuteMsg::Withdraw {
            amount: TEST_PRICE.to_string(),
            denom: TEST_DENOM.to_string(),
            beneficiary: TEST_CREATOR.to_string(),
        });
        let info = mock_info(TEST_CREATOR, &[]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg);
    }

    #[test]
    fn query_subscription_options_successful() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();

        let sub_option = PaymentOption {
            subscription_duration: SubscriptionDuration {
                amount_units: 10,
                duration_unit: DurationUnit::Hour,
            },
            price: Coin {
                denom: TEST_DENOM.to_string(),
                amount: Uint128::from(TEST_PRICE),
            },
        };
        let msg = ExecuteMsg::Admin(AdminExecuteMsg::AddSubscriptionOption {
            payment_option: sub_option,
        });
        let info = mock_info(TEST_CREATOR, &[]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = QueryMsg::SubscriptionOptions {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let data: SubscriptionOptionsResponse = from_binary(&res).unwrap();

        assert_eq!(data.subscription_options.len(), 2);
    }

    #[test]
    fn remove_subscription_successful() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();

        let msg = QueryMsg::SubscriptionOptions {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let data: SubscriptionOptionsResponse = from_binary(&res).unwrap();
        assert_eq!(data.subscription_options.len(), 1);

        let msg = ExecuteMsg::Admin(AdminExecuteMsg::RemoveSubscriptionOption { id_to_remove: 0 });
        let info = mock_info(TEST_CREATOR, &[]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        let msg = QueryMsg::SubscriptionOptions {};
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let data: SubscriptionOptionsResponse = from_binary(&res).unwrap();
        assert_eq!(data.subscription_options.len(), 0);
    }

    #[test]
    /// subscribe to option 2 of subscriptions
    fn subscribe_two_options_successful() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();

        // add subscription option 2
        let subscription_duration = SubscriptionDuration {
            duration_unit: TEST_DURATION_UNIT, //DurationUnit::Day,
            amount_units: TEST_DURATION_UNIT_AMOUNT + 100,
        };

        let subscription_option = PaymentOption {
            subscription_duration: subscription_duration,
            price: Coin {
                denom: TEST_DENOM.to_string(),
                amount: Uint128::from(TEST_PRICE),
            },
        };

        let temp = AdminExecuteMsg::AddSubscriptionOption {
            payment_option: subscription_option,
        };
        let msg = ExecuteMsg::Admin(temp);

        let info = mock_info(TEST_CREATOR, &[]); //&coins(1000, TEST_DENOM));

        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // subscribe to second one
        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let _res = subscribe(deps.as_mut(), TEST_CREATOR, 1, funds).unwrap();

        let msg = QueryMsg::SubscriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let _data: SubscriptionStatusResponse = from_binary(&res).unwrap();
    }

    #[test]
    /// subscribe to option 2 of subscriptions after removing first
    fn subscribe_option_2_remove_first_successful() {
        let mut deps = instantiate_contract();
        add_default_subscription_option(deps.as_mut()).unwrap();

        // add subscription option 2
        let subscription_duration = SubscriptionDuration {
            duration_unit: TEST_DURATION_UNIT, //DurationUnit::Day,
            amount_units: TEST_DURATION_UNIT_AMOUNT + 100,
        };

        let subscription_option = PaymentOption {
            subscription_duration: subscription_duration,
            price: Coin {
                denom: TEST_DENOM.to_string(),
                amount: Uint128::from(TEST_PRICE),
            },
        };

        let temp = AdminExecuteMsg::AddSubscriptionOption {
            payment_option: subscription_option,
        };
        let msg = ExecuteMsg::Admin(temp);

        let info = mock_info(TEST_CREATOR, &[]); //&coins(1000, TEST_DENOM));

        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // remove first option
        let msg = ExecuteMsg::Admin(AdminExecuteMsg::RemoveSubscriptionOption { id_to_remove: 0 });
        let info = mock_info(TEST_CREATOR, &[]);
        let _res = execute(deps.as_mut(), mock_env(), info, msg).unwrap();

        // subscribe to second one
        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let _res = subscribe(deps.as_mut(), TEST_CREATOR, 1, funds).unwrap();

        let msg = QueryMsg::SubscriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let _data: SubscriptionStatusResponse = from_binary(&res).unwrap();
    }
}
