use cosmwasm_std::{coins, to_binary, Addr, OverflowError, Uint128};
use croncat_sdk_core::{
    balancer::{BalancerMode, RoundRobinBalancer},
    types::{BalancesResponse, Config, UpdateConfig},
};
use cw20::{BalanceResponse, Cw20Coin, Cw20CoinVerified};

use crate::{
    contract::DEFAULT_NOMINATION_DURATION,
    msg::{ExecuteMsg, InstantiateMsg, ReceiveMsg},
    tests::{
        helpers::query_manager_balances,
        helpers::{default_app, default_instantiate_message, init_manager, query_manager_config},
        ADMIN, AGENT0, AGENT1, AGENT2, ANYONE, DENOM,
    },
    ContractError,
};
use cosmwasm_std::{coin, StdError};
use croncat_sdk_core::types::GasPrice;
use cw_multi_test::{BankSudo, Executor};

use super::helpers::{init_cw20, query_users_manager};

mod instantiate_tests {
    use super::*;

    #[test]
    fn default_init() {
        let mut app = default_app();
        let instantiate_msg: InstantiateMsg = default_instantiate_message();

        let manager_addr = init_manager(&mut app, instantiate_msg, &[]).unwrap();
        let config = query_manager_config(&app, &manager_addr);

        let expected_config = Config {
            paused: false,
            owner_addr: Addr::unchecked(ADMIN),
            min_tasks_per_agent: 3,
            agents_eject_threshold: 600,
            agent_nomination_duration: DEFAULT_NOMINATION_DURATION,
            croncat_factory_addr: Addr::unchecked("croncat_factory_addr"),
            croncat_tasks_key: ("croncat_tasks_name".to_owned(), [0, 1]),
            croncat_agents_key: ("croncat_agents_name".to_owned(), [0, 1]),
            agent_fee: 5,
            gas_price: Default::default(),
            cw20_whitelist: vec![],
            native_denom: DENOM.to_owned(),
            balancer: Default::default(),
            limit: 100,
            treasury_addr: None,
        };
        assert_eq!(config, expected_config)
    }

    #[test]
    fn custom_init() {
        let mut app = default_app();
        let instantiate_msg: InstantiateMsg = InstantiateMsg {
            denom: "cron".to_owned(),
            croncat_factory_addr: AGENT0.to_owned(),
            croncat_tasks_key: (AGENT1.to_owned(), [0, 1]),
            croncat_agents_key: (AGENT2.to_owned(), [0, 1]),
            owner_addr: Some(ANYONE.to_owned()),
            gas_price: Some(GasPrice {
                numerator: 10,
                denominator: 20,
                gas_adjustment_numerator: 30,
            }),
            agent_nomination_duration: Some(20),
            treasury_addr: Some(AGENT2.to_owned()),
        };
        let attach_funds = vec![coin(5000, "denom"), coin(2400, DENOM)];
        app.sudo(
            BankSudo::Mint {
                to_address: ADMIN.to_owned(),
                amount: attach_funds.clone(),
            }
            .into(),
        )
        .unwrap();
        let manager_addr = init_manager(&mut app, instantiate_msg, &attach_funds).unwrap();

        let config = query_manager_config(&app, &manager_addr);

        let expected_config = Config {
            paused: false,
            owner_addr: Addr::unchecked(ANYONE),
            min_tasks_per_agent: 3,
            agents_eject_threshold: 600,
            agent_nomination_duration: 20,
            croncat_factory_addr: Addr::unchecked(AGENT0),
            croncat_tasks_key: (AGENT1.to_owned(), [0, 1]),
            croncat_agents_key: (AGENT2.to_owned(), [0, 1]),
            agent_fee: 5,
            gas_price: GasPrice {
                numerator: 10,
                denominator: 20,
                gas_adjustment_numerator: 30,
            },
            cw20_whitelist: vec![],
            native_denom: "cron".to_owned(),
            balancer: Default::default(),
            limit: 100,
            treasury_addr: Some(Addr::unchecked(AGENT2)),
        };
        assert_eq!(config, expected_config);

        let manager_balances = query_manager_balances(&app, &manager_addr);
        for coin in attach_funds {
            assert!(manager_balances.native_balance.contains(&coin))
        }
        assert_eq!(manager_balances.cw20_balance, vec![]);
    }

    #[test]
    fn invalid_inits() {
        let mut app = default_app();

        // Invalid gas price
        let instantiate_msg: InstantiateMsg = InstantiateMsg {
            gas_price: Some(GasPrice {
                numerator: 0,
                denominator: 1,
                gas_adjustment_numerator: 2,
            }),
            ..default_instantiate_message()
        };

        let error: ContractError = init_manager(&mut app, instantiate_msg, &[])
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(error, ContractError::InvalidGasPrice {});

        // Bad owner_id
        let instantiate_msg: InstantiateMsg = InstantiateMsg {
            owner_addr: Some("BAD_INPUT".to_owned()),
            ..default_instantiate_message()
        };

        let error: ContractError = init_manager(&mut app, instantiate_msg, &[])
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(
            error,
            ContractError::Std(StdError::generic_err(
                "Invalid input: address not normalized"
            ))
        );

        // Bad cw_rules_addr
        let instantiate_msg: InstantiateMsg = InstantiateMsg {
            croncat_factory_addr: "BAD_INPUT".to_owned(),
            ..default_instantiate_message()
        };

        let error: ContractError = init_manager(&mut app, instantiate_msg, &[])
            .unwrap_err()
            .downcast()
            .unwrap();
        assert_eq!(
            error,
            ContractError::Std(StdError::generic_err(
                "Invalid input: address not normalized"
            ))
        );
    }
}

#[test]
fn update_config() {
    let mut app = default_app();
    let instantiate_msg: InstantiateMsg = default_instantiate_message();

    let attach_funds = vec![coin(5000, "denom"), coin(2400, DENOM)];
    app.sudo(
        BankSudo::Mint {
            to_address: ADMIN.to_owned(),
            amount: attach_funds.clone(),
        }
        .into(),
    )
    .unwrap();

    let manager_addr = init_manager(&mut app, instantiate_msg, &attach_funds).unwrap();

    let update_cfg_msg = UpdateConfig {
        owner_addr: Some("new_owner".to_string()),
        paused: Some(true),
        agent_fee: Some(0),
        gas_price: Some(GasPrice {
            numerator: 555,
            denominator: 666,
            gas_adjustment_numerator: 777,
        }),
        min_tasks_per_agent: Some(1),
        agents_eject_threshold: Some(3),
        balancer: Some(RoundRobinBalancer::new(BalancerMode::Equalizer)),
        croncat_tasks_key: Some(("new_key_tasks".to_owned(), [0, 1])),
        croncat_agents_key: Some(("new_key_agents".to_owned(), [0, 1])),
        treasury_addr: Some(ANYONE.to_owned()),
    };

    app.execute_contract(
        Addr::unchecked(ADMIN),
        manager_addr.clone(),
        &ExecuteMsg::UpdateConfig(Box::new(update_cfg_msg)),
        &[],
    )
    .unwrap();
    let config = query_manager_config(&app, &manager_addr);
    let expected_config = Config {
        paused: true,
        owner_addr: Addr::unchecked("new_owner"),
        min_tasks_per_agent: 1,
        agents_eject_threshold: 3,
        agent_nomination_duration: DEFAULT_NOMINATION_DURATION,
        croncat_factory_addr: Addr::unchecked("croncat_factory_addr"),
        croncat_tasks_key: ("new_key_tasks".to_owned(), [0, 1]),
        croncat_agents_key: ("new_key_agents".to_owned(), [0, 1]),
        agent_fee: 0,
        gas_price: GasPrice {
            numerator: 555,
            denominator: 666,
            gas_adjustment_numerator: 777,
        },
        cw20_whitelist: vec![],
        native_denom: DENOM.to_owned(),
        balancer: RoundRobinBalancer {
            mode: BalancerMode::Equalizer,
        },
        limit: 100,
        treasury_addr: Some(Addr::unchecked(ANYONE)),
    };
    assert_eq!(config, expected_config);

    // Shouldn't override any fields to None or anything
    let update_cfg_msg = UpdateConfig {
        owner_addr: None,
        paused: None,
        agent_fee: None,
        gas_price: None,
        min_tasks_per_agent: None,
        agents_eject_threshold: None,
        balancer: None,
        croncat_tasks_key: None,
        croncat_agents_key: None,
        treasury_addr: None,
    };

    app.execute_contract(
        Addr::unchecked("new_owner"),
        manager_addr.clone(),
        &ExecuteMsg::UpdateConfig(Box::new(update_cfg_msg)),
        &[],
    )
    .unwrap();
    let config = query_manager_config(&app, &manager_addr);
    assert_eq!(config, expected_config);
}

#[test]
fn invalid_updates_config() {
    let mut app = default_app();
    let instantiate_msg: InstantiateMsg = default_instantiate_message();

    let attach_funds = vec![coin(5000, "denom"), coin(2400, DENOM)];
    app.sudo(
        BankSudo::Mint {
            to_address: ADMIN.to_owned(),
            amount: attach_funds.clone(),
        }
        .into(),
    )
    .unwrap();

    let manager_addr = init_manager(&mut app, instantiate_msg, &attach_funds).unwrap();

    // Unauthorized
    let update_cfg_msg = UpdateConfig {
        owner_addr: Some("new_owner".to_string()),
        paused: Some(true),
        agent_fee: Some(0),
        gas_price: Some(GasPrice {
            numerator: 555,
            denominator: 666,
            gas_adjustment_numerator: 777,
        }),
        min_tasks_per_agent: Some(1),
        agents_eject_threshold: Some(3),
        balancer: Some(RoundRobinBalancer::new(BalancerMode::Equalizer)),
        croncat_tasks_key: Some(("new_key_tasks".to_owned(), [0, 1])),
        croncat_agents_key: Some(("new_key_agents".to_owned(), [0, 1])),
        treasury_addr: Some(ANYONE.to_owned()),
    };
    let err: ContractError = app
        .execute_contract(
            // Not admin
            Addr::unchecked(ANYONE),
            manager_addr.clone(),
            &ExecuteMsg::UpdateConfig(Box::new(update_cfg_msg)),
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, ContractError::Unauthorized {});

    // Invalid gas_price
    let update_cfg_msg = UpdateConfig {
        owner_addr: Some("new_owner".to_string()),
        paused: Some(true),
        agent_fee: Some(0),
        gas_price: Some(GasPrice {
            numerator: 555,
            denominator: 0,
            gas_adjustment_numerator: 777,
        }),
        min_tasks_per_agent: Some(1),
        agents_eject_threshold: Some(3),
        balancer: Some(RoundRobinBalancer::new(BalancerMode::Equalizer)),
        croncat_tasks_key: Some(("new_key_tasks".to_owned(), [0, 1])),
        croncat_agents_key: Some(("new_key_agents".to_owned(), [0, 1])),
        treasury_addr: Some(ANYONE.to_owned()),
    };
    let err: ContractError = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            manager_addr.clone(),
            &ExecuteMsg::UpdateConfig(Box::new(update_cfg_msg)),
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, ContractError::InvalidGasPrice {});

    // Invalid owner
    let update_cfg_msg = UpdateConfig {
        owner_addr: Some("New_owner".to_string()),
        paused: Some(true),
        agent_fee: Some(0),
        gas_price: Some(GasPrice {
            numerator: 555,
            denominator: 666,
            gas_adjustment_numerator: 777,
        }),
        min_tasks_per_agent: Some(1),
        agents_eject_threshold: Some(3),
        balancer: Some(RoundRobinBalancer::new(BalancerMode::Equalizer)),
        croncat_tasks_key: Some(("new_key_tasks".to_owned(), [0, 1])),
        croncat_agents_key: Some(("new_key_agents".to_owned(), [0, 1])),
        treasury_addr: Some(ANYONE.to_owned()),
    };
    let err: ContractError = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            manager_addr,
            &ExecuteMsg::UpdateConfig(Box::new(update_cfg_msg)),
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(
        err,
        ContractError::Std(StdError::generic_err(
            "Invalid input: address not normalized"
        ))
    );
}

#[test]
fn cw20_receive() {
    let mut app = default_app();

    let instantiate_msg: InstantiateMsg = default_instantiate_message();
    let manager_addr = init_manager(&mut app, instantiate_msg, &coins(100, DENOM)).unwrap();

    let cw20_addr = init_cw20(&mut app);
    app.execute_contract(
        Addr::unchecked(ADMIN),
        cw20_addr.clone(),
        &cw20::Cw20ExecuteMsg::Send {
            contract: manager_addr.to_string(),
            amount: Uint128::new(555),
            msg: to_binary(&ReceiveMsg::RefillCw20Balance {}).unwrap(),
        },
        &[],
    )
    .unwrap();

    let wallet_balances = query_users_manager(&app, &manager_addr, ADMIN);
    assert_eq!(
        wallet_balances.cw20_balance,
        vec![Cw20CoinVerified {
            address: cw20_addr.clone(),
            amount: Uint128::new(555),
        }]
    );

    let available_balances = query_manager_balances(&app, &manager_addr);
    assert_eq!(
        available_balances,
        BalancesResponse {
            native_balance: coins(100, DENOM),
            cw20_balance: vec![Cw20CoinVerified {
                address: cw20_addr,
                amount: Uint128::new(555),
            }],
        }
    )
}

#[test]
fn cw20_bad_messages() {
    let mut app = default_app();

    let instantiate_msg: InstantiateMsg = default_instantiate_message();
    let manager_addr = init_manager(&mut app, instantiate_msg, &[]).unwrap();

    let cw20_addr = init_cw20(&mut app);
    let err: ContractError = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            cw20_addr.clone(),
            &cw20::Cw20ExecuteMsg::Send {
                contract: manager_addr.to_string(),
                amount: Uint128::new(555),
                msg: Default::default(),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();

    assert_eq!(
        err,
        ContractError::Std(StdError::parse_err(
            "croncat_sdk_core::msg::ManagerReceiveMsg",
            "EOF while parsing a JSON value."
        ))
    );

    let err: ContractError = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            cw20_addr,
            &cw20::Cw20ExecuteMsg::Send {
                contract: manager_addr.to_string(),
                amount: Uint128::new(555),
                msg: to_binary(&true).unwrap(),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();

    assert_eq!(
        err,
        ContractError::Std(StdError::parse_err(
            "croncat_sdk_core::msg::ManagerReceiveMsg",
            "Expected to parse either a `true`, `false`, or a `null`."
        ))
    );
}

#[test]
fn users_withdraws() {
    let mut app = default_app();

    let instantiate_msg: InstantiateMsg = default_instantiate_message();
    let manager_addr = init_manager(&mut app, instantiate_msg, &[]).unwrap();

    // refill balances
    let cw20_addr = init_cw20(&mut app);
    app.execute_contract(
        Addr::unchecked(ADMIN),
        cw20_addr.clone(),
        &cw20::Cw20ExecuteMsg::Send {
            contract: manager_addr.to_string(),
            amount: Uint128::new(1000),
            msg: to_binary(&ReceiveMsg::RefillCw20Balance {}).unwrap(),
        },
        &[],
    )
    .unwrap();
    app.execute_contract(
        Addr::unchecked(ADMIN),
        manager_addr.clone(),
        &ExecuteMsg::RefillNativeBalance {},
        &[coin(100_000, DENOM)],
    )
    .unwrap();

    // Withdraw half
    let user_native_balance = app.wrap().query_balance(ADMIN, DENOM).unwrap();
    let user_cw20_balance: cw20::BalanceResponse = app
        .wrap()
        .query_wasm_smart(
            cw20_addr.clone(),
            &cw20::Cw20QueryMsg::Balance {
                address: ADMIN.to_owned(),
            },
        )
        .unwrap();

    app.execute_contract(
        Addr::unchecked(ADMIN),
        manager_addr.clone(),
        &ExecuteMsg::UserWithdraw {
            cw20_balances: vec![Cw20Coin {
                address: cw20_addr.to_string(),
                amount: Uint128::new(500),
            }],
            native_balances: vec![coin(50_000, DENOM)],
        },
        &[],
    )
    .unwrap();

    // Check it got withdrawn
    let new_native_balance = app.wrap().query_balance(ADMIN, DENOM).unwrap();
    assert_eq!(
        new_native_balance.amount,
        user_native_balance.amount + Uint128::new(50_000)
    );
    // Check it updated on cw20 state
    let new_cw20_balance: cw20::BalanceResponse = app
        .wrap()
        .query_wasm_smart(
            cw20_addr.clone(),
            &cw20::Cw20QueryMsg::Balance {
                address: ADMIN.to_owned(),
            },
        )
        .unwrap();
    assert_eq!(
        new_cw20_balance.balance,
        user_cw20_balance.balance + Uint128::new(500)
    );

    // Check it updated on manager
    let manager_wallet_balance = query_users_manager(&app, &manager_addr, ADMIN);
    assert_eq!(
        manager_wallet_balance,
        BalancesResponse {
            native_balance: vec![coin(50_000, DENOM)],
            cw20_balance: vec![Cw20CoinVerified {
                address: cw20_addr.clone(),
                amount: Uint128::new(500),
            }]
        }
    );

    // Check available got updated too
    let available_balances = query_manager_balances(&app, &manager_addr);
    assert_eq!(
        available_balances,
        BalancesResponse {
            native_balance: vec![coin(50_000, DENOM)],
            cw20_balance: vec![Cw20CoinVerified {
                address: cw20_addr.clone(),
                amount: Uint128::new(500),
            }]
        }
    );

    // Withdraw rest
    // can withdraw cw20 without native and vice versa
    app.execute_contract(
        Addr::unchecked(ADMIN),
        manager_addr.clone(),
        &ExecuteMsg::UserWithdraw {
            cw20_balances: vec![Cw20Coin {
                address: cw20_addr.to_string(),
                amount: Uint128::new(500),
            }],
            native_balances: vec![],
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked(ADMIN),
        manager_addr.clone(),
        &ExecuteMsg::UserWithdraw {
            cw20_balances: vec![],
            native_balances: vec![coin(50_000, DENOM)],
        },
        &[],
    )
    .unwrap();

    let fully_withdrawn_user_balance: cw20::BalanceResponse = app
        .wrap()
        .query_wasm_smart(
            cw20_addr,
            &cw20::Cw20QueryMsg::Balance {
                address: ADMIN.to_owned(),
            },
        )
        .unwrap();
    assert_eq!(
        fully_withdrawn_user_balance.balance,
        user_cw20_balance.balance + Uint128::new(1000)
    );

    // Check it updated on manager
    let manager_wallet_balance = query_users_manager(&app, &manager_addr, ADMIN);
    assert_eq!(
        manager_wallet_balance,
        BalancesResponse {
            native_balance: vec![],
            cw20_balance: vec![]
        }
    );

    // Check available got updated too
    let available_balances = query_manager_balances(&app, &manager_addr);
    assert_eq!(
        available_balances,
        BalancesResponse {
            native_balance: vec![],
            cw20_balance: vec![]
        }
    );
}

#[test]
fn failed_users_withdraws() {
    let mut app = default_app();

    let instantiate_msg: InstantiateMsg = default_instantiate_message();
    let manager_addr = init_manager(&mut app, instantiate_msg, &[]).unwrap();

    let cw20_addr = init_cw20(&mut app);

    // try to withdraw empty balances
    let err: ContractError = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            manager_addr.clone(),
            &ExecuteMsg::UserWithdraw {
                cw20_balances: vec![Cw20Coin {
                    address: cw20_addr.to_string(),
                    amount: Uint128::new(500),
                }],
                native_balances: vec![],
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();

    assert_eq!(err, ContractError::EmptyBalance {});

    let err: ContractError = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            manager_addr.clone(),
            &ExecuteMsg::UserWithdraw {
                cw20_balances: vec![],
                native_balances: vec![coin(10, DENOM)],
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();

    assert_eq!(err, ContractError::EmptyBalance {});

    // refill balances
    app.execute_contract(
        Addr::unchecked(ADMIN),
        cw20_addr.clone(),
        &cw20::Cw20ExecuteMsg::Send {
            contract: manager_addr.to_string(),
            amount: Uint128::new(1000),
            msg: to_binary(&ReceiveMsg::RefillCw20Balance {}).unwrap(),
        },
        &[],
    )
    .unwrap();
    app.execute_contract(
        Addr::unchecked(ADMIN),
        manager_addr.clone(),
        &ExecuteMsg::RefillNativeBalance {},
        &[coin(10_000, DENOM)],
    )
    .unwrap();

    // try to withdraw too much balances
    let err: ContractError = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            manager_addr.clone(),
            &ExecuteMsg::UserWithdraw {
                cw20_balances: vec![Cw20Coin {
                    address: cw20_addr.to_string(),
                    amount: Uint128::new(1001),
                }],
                native_balances: vec![],
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();

    assert_eq!(
        err,
        ContractError::Std(StdError::overflow(OverflowError::new(
            cosmwasm_std::OverflowOperation::Sub,
            "1000",
            "1001"
        )))
    );

    let err: ContractError = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            manager_addr.clone(),
            &ExecuteMsg::UserWithdraw {
                cw20_balances: vec![],
                native_balances: vec![coin(10_001, DENOM)],
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();

    assert_eq!(
        err,
        ContractError::Std(StdError::overflow(OverflowError::new(
            cosmwasm_std::OverflowOperation::Sub,
            "10000",
            "10001"
        )))
    );

    // Another user tries to withdraw
    // No steals here
    let err: ContractError = app
        .execute_contract(
            Addr::unchecked(ANYONE),
            manager_addr.clone(),
            &ExecuteMsg::UserWithdraw {
                cw20_balances: vec![Cw20Coin {
                    address: cw20_addr.to_string(),
                    amount: Uint128::new(500),
                }],
                native_balances: vec![],
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();

    assert_eq!(err, ContractError::EmptyBalance {});

    let err: ContractError = app
        .execute_contract(
            Addr::unchecked(ANYONE),
            manager_addr,
            &ExecuteMsg::UserWithdraw {
                cw20_balances: vec![],
                native_balances: vec![coin(1, DENOM)],
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();

    assert_eq!(err, ContractError::EmptyBalance {});
}

#[test]
fn withdraw_balances() {
    let mut app = default_app();

    let instantiate_msg: InstantiateMsg = default_instantiate_message();

    let attach_funds = vec![coin(2400, DENOM), coin(5000, "denom")];
    app.sudo(
        BankSudo::Mint {
            to_address: ADMIN.to_owned(),
            amount: attach_funds.clone(),
        }
        .into(),
    )
    .unwrap();

    let manager_addr = init_manager(&mut app, instantiate_msg, &attach_funds).unwrap();

    // refill balance
    let cw20_addr = init_cw20(&mut app);
    app.execute_contract(
        Addr::unchecked(ADMIN),
        cw20_addr.clone(),
        &cw20::Cw20ExecuteMsg::Send {
            contract: manager_addr.to_string(),
            amount: Uint128::new(1000),
            msg: to_binary(&ReceiveMsg::RefillCw20Balance {}).unwrap(),
        },
        &[],
    )
    .unwrap();

    let available_balances = query_manager_balances(&app, &manager_addr);
    assert_eq!(
        available_balances,
        BalancesResponse {
            native_balance: attach_funds.clone(),
            cw20_balance: vec![Cw20CoinVerified {
                address: cw20_addr.clone(),
                amount: Uint128::new(1000),
            }]
        }
    );
    let owner_native1_balance = app
        .wrap()
        .query_balance(Addr::unchecked(ADMIN), DENOM)
        .unwrap();
    let owner_native2_balance = app
        .wrap()
        .query_balance(Addr::unchecked(ADMIN), "denom")
        .unwrap();
    let owner_cw20_balance: BalanceResponse = app
        .wrap()
        .query_wasm_smart(
            cw20_addr.clone(),
            &cw20_base::msg::QueryMsg::Balance {
                address: ADMIN.to_owned(),
            },
        )
        .unwrap();

    // Withdraw all of balances
    app.execute_contract(
        Addr::unchecked(ADMIN),
        manager_addr.clone(),
        &ExecuteMsg::OwnerWithdraw {},
        &[],
    )
    .unwrap();
    let available_balances = query_manager_balances(&app, &manager_addr);
    assert_eq!(
        available_balances,
        BalancesResponse {
            native_balance: vec![],
            cw20_balance: vec![]
        }
    );

    let updated_owner_native1_balance = app
        .wrap()
        .query_balance(Addr::unchecked(ADMIN), DENOM)
        .unwrap();
    let updated_owner_native2_balance = app
        .wrap()
        .query_balance(Addr::unchecked(ADMIN), "denom")
        .unwrap();
    let updated_owner_cw20_balance: BalanceResponse = app
        .wrap()
        .query_wasm_smart(
            cw20_addr,
            &cw20_base::msg::QueryMsg::Balance {
                address: ADMIN.to_owned(),
            },
        )
        .unwrap();

    assert_eq!(
        updated_owner_native1_balance.amount,
        owner_native1_balance.amount + Uint128::new(2400)
    );
    assert_eq!(
        updated_owner_native2_balance.amount,
        owner_native2_balance.amount + Uint128::new(5000)
    );
    assert_eq!(
        updated_owner_cw20_balance.balance,
        owner_cw20_balance.balance + Uint128::new(1000)
    );

    // Withdraw on empty balances should do nothing
    app.execute_contract(
        Addr::unchecked(ADMIN),
        manager_addr.clone(),
        &ExecuteMsg::OwnerWithdraw {},
        &[],
    )
    .unwrap();
}

#[test]
fn failed_move_balances() {
    let mut app = default_app();

    let instantiate_msg: InstantiateMsg = default_instantiate_message();

    let attach_funds = vec![coin(2400, DENOM), coin(5000, "denom")];
    app.sudo(
        BankSudo::Mint {
            to_address: ADMIN.to_owned(),
            amount: attach_funds.clone(),
        }
        .into(),
    )
    .unwrap();

    let manager_addr = init_manager(&mut app, instantiate_msg, &attach_funds).unwrap();

    // refill balance
    let cw20_addr = init_cw20(&mut app);
    app.execute_contract(
        Addr::unchecked(ADMIN),
        cw20_addr.clone(),
        &cw20::Cw20ExecuteMsg::Send {
            contract: manager_addr.to_string(),
            amount: Uint128::new(1000),
            msg: to_binary(&ReceiveMsg::RefillCw20Balance {}).unwrap(),
        },
        &[],
    )
    .unwrap();

    // Withdraw not by owner
    let err: ContractError = app
        .execute_contract(
            Addr::unchecked(ANYONE),
            manager_addr.clone(),
            &ExecuteMsg::OwnerWithdraw {},
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(err, ContractError::Unauthorized {});
}
