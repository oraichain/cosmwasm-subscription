#[cfg(test)]
mod tests {
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info, MockApi, MockQuerier};
    use cosmwasm_std::{
        coins, from_binary, Coin, DepsMut, MemoryStorage, OwnedDeps, Response, Uint128,
    };

    use crate::error::ContractError;
    use crate::execute_messages::msg::ExecuteMsg;
    use crate::execute_messages::msg_admin::AdminExecuteMsg;
    use crate::instantiation::instantiate_messages::InstantiateMsg;
    use crate::query::query_messages::QueryMsg;

    use crate::contract::{execute, instantiate, query};
    use crate::query::query_responses::SubscriptionStatusResponse;
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
        let mut deps = mock_dependencies(&[Coin {
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

    fn add_subscription_option(deps: DepsMut) -> Result<Response, ContractError> {
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
            subscription_option: subscription_option,
        };
        let msg = ExecuteMsg::Admin(temp);

        let info = mock_info(TEST_CREATOR, &[]); //&coins(1000, TEST_DENOM));

        return execute(deps, mock_env(), info, msg);
    }

    fn subscribe(deps: DepsMut, funds: Coin) -> Result<Response, ContractError> {
        let subscription_duration = SubscriptionDuration {
            duration_unit: TEST_DURATION_UNIT, //DurationUnit::Day,
            amount_units: TEST_DURATION_UNIT_AMOUNT,
        };
        let msg = ExecuteMsg::Subscribe {
            subscription_option: subscription_duration,
        };

        let info = mock_info(TEST_CREATOR, &[funds]); //&coins(1000, TEST_DENOM));

        return execute(deps, mock_env(), info, msg);
    }

    #[test]
    fn instantiate_success() {
        let mut _deps = instantiate_contract();
    }

    #[test]
    fn add_subcription_option_successful() {
        let mut deps = instantiate_contract();
        add_subscription_option(deps.as_mut()).unwrap();
    }

    #[test]
    fn subscribe_successful() {
        let mut deps = instantiate_contract();
        add_subscription_option(deps.as_mut()).unwrap();

        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let _res = subscribe(deps.as_mut(), funds).unwrap();

        let msg = QueryMsg::SubcriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let _data: SubscriptionStatusResponse = from_binary(&res).unwrap();
    }

    #[test]
    fn subscribe_expire() {
        let mut deps = instantiate_contract();
        add_subscription_option(deps.as_mut()).unwrap();

        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let _res = subscribe(deps.as_mut(), funds).unwrap();

        let msg = QueryMsg::SubcriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let _res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let mut env = mock_env();
        env.block.time = env.block.time.plus_seconds(100000);

        let msg = QueryMsg::SubcriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let res = query(deps.as_ref(), env, msg).unwrap();

        let data: SubscriptionStatusResponse = from_binary(&res).unwrap();

        assert!(!data.is_valid);
    }

    #[test]
    fn subscribe_lengthened() {
        let mut deps = instantiate_contract();
        add_subscription_option(deps.as_mut()).unwrap();

        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let _res = subscribe(deps.as_mut(), funds).unwrap();

        let msg = QueryMsg::SubcriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let _res = query(deps.as_ref(), mock_env(), msg).unwrap();

        let mut env = mock_env();
        env.block.time = env.block.time.plus_seconds(100000);

        let msg = QueryMsg::SubcriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let res = query(deps.as_ref(), env.clone(), msg).unwrap();

        let data: SubscriptionStatusResponse = from_binary(&res).unwrap();

        assert!(!data.is_valid);

        let funds = Coin {
            denom: TEST_DENOM.to_string(),
            amount: Uint128::from(TEST_PRICE),
        };

        let _res = subscribe(deps.as_mut(), funds).unwrap();

        let msg = QueryMsg::SubcriptionStatus {
            addr: TEST_CREATOR.to_string(),
        };
        let res = query(deps.as_ref(), env, msg).unwrap();

        let data: SubscriptionStatusResponse = from_binary(&res).unwrap();

        println!("{:?}", data);
        assert!(data.is_valid);
    }
}
