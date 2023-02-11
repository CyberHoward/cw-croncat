use crate::error::ContractError;
use crate::msg::*;
use crate::state::{
    DEFAULT_MAX_SLOTS_PASSOVER, DEFAULT_MIN_ACTIVE_RESERVE,
    DEFAULT_MIN_COINS_FOR_AGENT_REGISTRATION, DEFAULT_NOMINATION_BLOCK_DURATION,
};
use crate::tests::common::*;
use cosmwasm_std::{coins, Addr, BankMsg, Coin, Uint128, Uint64};
use croncat_sdk_agents::msg::{AgentResponse, GetAgentIdsResponse};
use croncat_sdk_agents::types::Config;
use croncat_sdk_tasks::types::{Action, Interval, TaskRequest};

use cw_multi_test::{App, AppResponse, Executor};

#[test]
fn test_contract_initialize_is_successfull() {
    let mut app = default_app();
    let contract_code_id = app.store_code(croncat_agents_contract());

    let init_msg = InstantiateMsg {
        version: Some("0.1".to_owned()),
        owner_addr: Some(ADMIN.to_string()),
        agent_nomination_duration: None,
        min_tasks_per_agent: None,
        croncat_manager_key: ("manager".to_owned(), [4, 2]),
        croncat_tasks_key: ("tasks".to_owned(), [42, 0]),
        min_coin_for_agent_registration: None,
        max_slot_passover: Some(DEFAULT_MAX_SLOTS_PASSOVER),
        min_active_reserve: Some(DEFAULT_MIN_ACTIVE_RESERVE),
    };
    let croncat_agents_addr = app
        .instantiate_contract(
            contract_code_id,
            Addr::unchecked(ADMIN),
            &init_msg,
            &[],
            "agents",
            None,
        )
        .unwrap();

    let config: Config = app
        .wrap()
        .query_wasm_smart(croncat_agents_addr, &QueryMsg::Config {})
        .unwrap();
    assert_eq!(config.owner_addr, Addr::unchecked(ADMIN));

    let init_msg = InstantiateMsg {
        version: Some("0.1".to_owned()),
        owner_addr: Some(ANYONE.to_string()),
        agent_nomination_duration: None,
        min_tasks_per_agent: None,
        croncat_manager_key: ("manager".to_owned(), [4, 2]),
        croncat_tasks_key: ("tasks".to_owned(), [42, 0]),
        min_coin_for_agent_registration: None,
        max_slot_passover: Some(DEFAULT_MAX_SLOTS_PASSOVER),
        min_active_reserve: Some(DEFAULT_MIN_ACTIVE_RESERVE),
    };

    let croncat_agents_addr = app
        .instantiate_contract(
            contract_code_id,
            Addr::unchecked(ANYONE),
            &init_msg,
            &[],
            "agents",
            None,
        )
        .unwrap();

    let config: Config = app
        .wrap()
        .query_wasm_smart(croncat_agents_addr, &QueryMsg::Config {})
        .unwrap();
    assert_eq!(config.owner_addr, Addr::unchecked(ANYONE));
}

//RegisterAgent
#[test]
fn test_register_agent_is_successfull() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr: _,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr: _,
    } = init_test_scope(&mut app);

    app.execute_contract(
        Addr::unchecked(ADMIN),
        croncat_agents_addr.clone(),
        &ExecuteMsg::RegisterAgent {
            payable_account_id: Some(ANYONE.to_string()),
        },
        &[],
    )
    .unwrap();

    let agent_response: AgentResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr,
            &QueryMsg::GetAgent {
                account_id: ADMIN.to_string(),
            },
        )
        .unwrap();

    let agent = agent_response.agent.unwrap();
    assert_eq!(agent.status, AgentStatus::Active);
    assert_eq!(agent.balance, Uint128::new(0));
}

#[test]
fn test_register_agent_fails() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr: _,
    } = init_test_scope(&mut app);
    let error: ContractError = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            croncat_agents_addr.clone(),
            &ExecuteMsg::RegisterAgent {
                payable_account_id: Some(ANYONE.to_string()),
            },
            &[Coin {
                denom: NATIVE_DENOM.to_string(),
                amount: Uint128::new(10),
            }],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(error, ContractError::NoFundsShouldBeAttached);

    //Check contract is paused and failing
    let mut config = mock_update_config(croncat_factory_addr.as_str());
    config.paused = Some(true);
    app.execute_contract(
        Addr::unchecked(ADMIN),
        croncat_agents_addr.clone(),
        &ExecuteMsg::UpdateConfig { config },
        &[],
    )
    .unwrap();

    let error: ContractError = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            croncat_agents_addr,
            &ExecuteMsg::RegisterAgent {
                payable_account_id: Some(ANYONE.to_string()),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();
    assert_eq!(error, ContractError::ContractPaused);
}

#[test]
fn test_update_agent_is_successfull() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr: _,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr: _,
    } = init_test_scope(&mut app);
    app.execute_contract(
        Addr::unchecked(ADMIN),
        croncat_agents_addr.clone(),
        &ExecuteMsg::RegisterAgent {
            payable_account_id: Some(ANYONE.to_string()),
        },
        &[],
    )
    .unwrap();

    app.execute_contract(
        Addr::unchecked(ADMIN),
        croncat_agents_addr.clone(),
        &ExecuteMsg::UpdateAgent {
            payable_account_id: ADMIN.to_string(),
        },
        &[],
    )
    .unwrap();

    let agent_response: AgentResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr,
            &QueryMsg::GetAgent {
                account_id: ADMIN.to_string(),
            },
        )
        .unwrap();
    assert_eq!(
        agent_response.agent.unwrap().payable_account_id.to_string(),
        ADMIN.to_string()
    );
}

//UpdateAgent tests
#[test]
fn test_update_agent_fails() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr: _,
    } = init_test_scope(&mut app); //Check contract fails when agent does not exist
    app.execute_contract(
        Addr::unchecked(ADMIN),
        croncat_agents_addr.clone(),
        &ExecuteMsg::RegisterAgent {
            payable_account_id: Some(ANYONE.to_string()),
        },
        &[],
    )
    .unwrap();

    let error: ContractError = app
        .execute_contract(
            Addr::unchecked(ANYONE),
            croncat_agents_addr.clone(),
            &ExecuteMsg::UpdateAgent {
                payable_account_id: ADMIN.to_string(),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();

    assert_eq!(error, ContractError::AgentNotRegistered);

    //Check contract is paused and failing
    let mut config = mock_update_config(croncat_factory_addr.as_str());
    config.paused = Some(true);
    app.execute_contract(
        Addr::unchecked(ADMIN),
        croncat_agents_addr.clone(),
        &ExecuteMsg::UpdateConfig { config },
        &[],
    )
    .unwrap();

    let error: ContractError = app
        .execute_contract(
            Addr::unchecked(ADMIN),
            croncat_agents_addr,
            &ExecuteMsg::UpdateAgent {
                payable_account_id: ADMIN.to_string(),
            },
            &[],
        )
        .unwrap_err()
        .downcast()
        .unwrap();

    assert_eq!(error, ContractError::ContractPaused);
}

//UpdateAgent tests
#[test]
fn test_agent_check_in_successfull() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr: _,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr,
    } = init_test_scope(&mut app);

    register_agent(&mut app, &croncat_agents_addr, ANYONE, PARTICIPANT0).unwrap();
    register_agent(&mut app, &croncat_agents_addr, ADMIN, PARTICIPANT0).unwrap();
    app.update_block(|block| add_seconds_to_block(block, 500));
    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, PARTICIPANT0).unwrap();
    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, PARTICIPANT1).unwrap();
    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, PARTICIPANT2).unwrap();
    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, PARTICIPANT3).unwrap();
    app.update_block(|block| increment_block_height(block, Some(30)));

    check_in_agent(&mut app, &croncat_agents_addr, ADMIN).unwrap();

    let agent_response: AgentResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr,
            &QueryMsg::GetAgent {
                account_id: ADMIN.to_string(),
            },
        )
        .unwrap();

    assert_eq!(agent_response.agent.unwrap().status, AgentStatus::Active);
}
#[test]
fn test_accept_nomination_agent() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr: _,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr,
    } = init_test_scope(&mut app);


    // Register AGENT1, who immediately becomes active
    register_agent(&mut app, &croncat_agents_addr, AGENT1, AGENT_BENEFICIARY).unwrap();

    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, PARTICIPANT1).unwrap();

    let total_tasks = get_total_tasks(&mut app, &croncat_tasks_addr).unwrap();
    assert_eq!(total_tasks, 1);

    // Register two agents
    register_agent(&mut app, &croncat_agents_addr, AGENT2, AGENT_BENEFICIARY).unwrap();
    register_agent(&mut app, &croncat_agents_addr, AGENT3, AGENT_BENEFICIARY).unwrap();

    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, PARTICIPANT2).unwrap();
    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, PARTICIPANT3).unwrap();
    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, PARTICIPANT4).unwrap();

    //Should be active 1 and pending 2 agents
    let (agent_ids_res, num_active_agents, _) = get_agent_ids(&app, &croncat_agents_addr);
    assert_eq!(1, num_active_agents);
    assert_eq!(2, agent_ids_res.pending.len());

    let mut agent_status = get_agent_status(&mut app, &croncat_agents_addr, AGENT1)
        .unwrap()
        .agent
        .unwrap()
        .status;
    assert_eq!(AgentStatus::Active, agent_status);
    agent_status = get_agent_status(&mut app, &croncat_agents_addr, AGENT2)
        .unwrap()
        .agent
        .unwrap()
        .status;
    assert_eq!(AgentStatus::Pending, agent_status);
    agent_status = get_agent_status(&mut app, &croncat_agents_addr, AGENT3)
        .unwrap()
        .agent
        .unwrap()
        .status;
    assert_eq!(AgentStatus::Pending, agent_status);

    // Attempt to accept nomination
    // First try with the agent second in line in the pending queue.
    // This should fail because it's not time for them yet.
    let mut check_in_res = check_in_agent(&mut app, &croncat_agents_addr, AGENT3);
    assert!(
        &check_in_res.is_err(),
        "Should throw error when agent in second position tries to nominate before their time."
    );
    assert_eq!(
        ContractError::TryLaterForNomination,
        check_in_res.unwrap_err().downcast().unwrap()
    );

    //Create few tasks for next agent nomination
    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, PARTICIPANT5).unwrap();
    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, PARTICIPANT6).unwrap();
    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, ANYONE).unwrap();
    app.update_block(|block| increment_block_height(block, Some(20)));

    // Now try from person at the beginning of the pending queue
    // This agent should succeed
    check_in_res = check_in_agent(&mut app, &croncat_agents_addr, AGENT2);

    assert!(
        check_in_res.is_ok(),
        "Agent at the front of the pending queue should be allowed to nominate themselves"
    );

    // Check that active and pending queues are correct
    let (agent_ids_res, num_active_agents, _) = get_agent_ids(&app, &croncat_agents_addr);
    assert_eq!(2, num_active_agents);
    assert_eq!(1, agent_ids_res.pending.len());

    // The agent that was second in the queue is now first,
    // tries again, but there aren't enough tasks
    check_in_res = check_in_agent(&mut app, &croncat_agents_addr, AGENT3);

    let error_msg = check_in_res.unwrap_err();
    assert_eq!(
        ContractError::TryLaterForNomination,
        error_msg.downcast().unwrap()
    );

    agent_status = get_agent_status(&mut app, &croncat_agents_addr, AGENT3)
        .unwrap()
        .agent
        .unwrap()
        .status;
    assert_eq!(AgentStatus::Pending, agent_status);

    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, ADMIN).unwrap();
    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, AGENT0).unwrap();
    create_task(&mut app, croncat_tasks_addr.as_str(), ADMIN, AGENT2).unwrap();
    app.update_block(|block| increment_block_height(block, Some(20)));

    // Agent second in line nominates themself
    check_in_res = check_in_agent(&mut app, &croncat_agents_addr, AGENT3);
    assert!(
        check_in_res.is_ok(),
        "Agent second in line should be able to nominate themselves"
    );

    let (_, _, num_pending_agents) = get_agent_ids(&app, &croncat_agents_addr);

    // Ensure the pending list is empty, having the earlier index booted
    assert_eq!(
        num_pending_agents, 0,
        "Expect the pending queue to be empty"
    );
}

#[test]
fn test_get_agent_status() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr: _,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr: _,
    } = init_test_scope(&mut app);
    let agent_status_res = get_agent_status(&mut app, &croncat_agents_addr, AGENT1).unwrap();
    assert_eq!(None, agent_status_res.agent);

    // Register AGENT1, who immediately becomes active
    let register_agent_res =
        register_agent(&mut app, &croncat_agents_addr, AGENT0, AGENT_BENEFICIARY);
    // First registered agent becomes active
    assert!(
        register_agent_res.is_ok(),
        "Registering agent should succeed"
    );

    let agent_status_res = get_agent_status(&mut app, &croncat_agents_addr, AGENT0).unwrap();
    assert_eq!(AgentStatus::Active, agent_status_res.agent.unwrap().status);

    // Register an agent and make sure the status comes back as pending
    let register_agent_res = register_agent(&mut app, &croncat_agents_addr, AGENT1, PARTICIPANT1);
    assert!(
        register_agent_res.is_ok(),
        "Registering agent should succeed"
    );
    let agent_status_res = get_agent_status(&mut app, &croncat_agents_addr, AGENT1).unwrap();
    assert_eq!(
        AgentStatus::Pending,
        agent_status_res.agent.unwrap().status,
        "New agent should be pending"
    );
}
#[test]
fn test_last_unregistered_active_agent_promotes_first_pending() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr: _,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr: _,
    } = init_test_scope(&mut app);
    // Register agents
    register_agent(&mut app, &croncat_agents_addr, AGENT1, AGENT_BENEFICIARY).unwrap();
    register_agent(&mut app, &croncat_agents_addr, AGENT2, AGENT_BENEFICIARY).unwrap();
    register_agent(&mut app, &croncat_agents_addr, AGENT3, AGENT_BENEFICIARY).unwrap();
    register_agent(&mut app, &croncat_agents_addr, AGENT4, AGENT_BENEFICIARY).unwrap();

    // Check if one is active and rest is pending
    let agent_ids: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(
        agent_ids,
        GetAgentIdsResponse {
            active: vec![Addr::unchecked(AGENT1)],
            pending: vec![
                Addr::unchecked(AGENT2),
                Addr::unchecked(AGENT3),
                Addr::unchecked(AGENT4)
            ]
        }
    );

    // Unregister agent
    let unreg_msg = ExecuteMsg::UnregisterAgent {};
    app.execute_contract(
        Addr::unchecked(AGENT1),
        croncat_agents_addr.clone(),
        &unreg_msg,
        &[],
    )
    .unwrap();
    let agent_ids: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(
        agent_ids,
        GetAgentIdsResponse {
            active: vec![],
            pending: vec![
                Addr::unchecked(AGENT2),
                Addr::unchecked(AGENT3),
                Addr::unchecked(AGENT4)
            ]
        }
    );

    // Check in
    app.execute_contract(
        Addr::unchecked(AGENT2),
        croncat_agents_addr.clone(),
        &ExecuteMsg::CheckInAgent {},
        &[],
    )
    .unwrap();
    let agent_ids: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(
        agent_ids,
        GetAgentIdsResponse {
            active: vec![Addr::unchecked(AGENT2)],
            pending: vec![Addr::unchecked(AGENT3), Addr::unchecked(AGENT4)]
        }
    );
}
#[test]
fn test_removing_agent_from_any_side_is_working() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr: _,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr: _,
    } = init_test_scope(&mut app);
    // Register agents
    register_agent(&mut app, &croncat_agents_addr, AGENT0, AGENT_BENEFICIARY).unwrap();
    register_agent(&mut app, &croncat_agents_addr, AGENT1, AGENT_BENEFICIARY).unwrap();
    register_agent(&mut app, &croncat_agents_addr, AGENT2, AGENT_BENEFICIARY).unwrap();
    register_agent(&mut app, &croncat_agents_addr, AGENT3, AGENT_BENEFICIARY).unwrap();
    register_agent(&mut app, &croncat_agents_addr, AGENT4, AGENT_BENEFICIARY).unwrap();

    let agent_ids: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(
        agent_ids,
        GetAgentIdsResponse {
            active: vec![Addr::unchecked(AGENT0)],
            pending: vec![
                Addr::unchecked(AGENT1),
                Addr::unchecked(AGENT2),
                Addr::unchecked(AGENT3),
                Addr::unchecked(AGENT4)
            ]
        }
    );

    // Unregister agent from the front
    app.execute_contract(
        Addr::unchecked(AGENT2),
        croncat_agents_addr.clone(),
        &ExecuteMsg::UnregisterAgent {},
        &[],
    )
    .unwrap();

    let agent_ids: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(
        agent_ids,
        GetAgentIdsResponse {
            active: vec![Addr::unchecked(AGENT0)],
            pending: vec![
                Addr::unchecked(AGENT1),
                Addr::unchecked(AGENT3),
                Addr::unchecked(AGENT4)
            ]
        }
    );

    // Unregister agent from the behind
    app.execute_contract(
        Addr::unchecked(AGENT3),
        croncat_agents_addr.clone(),
        &ExecuteMsg::UnregisterAgent {},
        &[],
    )
    .unwrap();

    let agent_ids: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(
        agent_ids,
        GetAgentIdsResponse {
            active: vec![Addr::unchecked(AGENT0)],
            pending: vec![Addr::unchecked(AGENT1), Addr::unchecked(AGENT4)]
        }
    );

    // Should work even if it's first person in the queue
    app.execute_contract(
        Addr::unchecked(AGENT1),
        croncat_agents_addr.clone(),
        &ExecuteMsg::UnregisterAgent {},
        &[],
    )
    .unwrap();

    let agent_ids: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(
        agent_ids,
        GetAgentIdsResponse {
            active: vec![Addr::unchecked(AGENT0)],
            pending: vec![Addr::unchecked(AGENT4)]
        }
    );

    // return one agent
    register_agent(&mut app, &croncat_agents_addr, AGENT1, AGENT_BENEFICIARY).unwrap();
    let agent_ids: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(
        agent_ids,
        GetAgentIdsResponse {
            active: vec![Addr::unchecked(AGENT0)],
            pending: vec![Addr::unchecked(AGENT4), Addr::unchecked(AGENT1)]
        }
    );
    // Or the last
    app.execute_contract(
        Addr::unchecked(AGENT1),
        croncat_agents_addr.clone(),
        &ExecuteMsg::UnregisterAgent {},
        &[],
    )
    .unwrap();

    let agent_ids: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(
        agent_ids,
        GetAgentIdsResponse {
            active: vec![Addr::unchecked(AGENT0)],
            pending: vec![Addr::unchecked(AGENT4)]
        }
    );
}

#[test]
fn test_withdraw_rewards_balances_on_unregister() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr: _,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr: _,
    } = init_test_scope(&mut app);

    // Register agents
    register_agent(&mut app, &croncat_agents_addr, AGENT0, AGENT_BENEFICIARY).unwrap();
    let old_balance = app
        .wrap()
        .query_balance(AGENT_BENEFICIARY, NATIVE_DENOM)
        .unwrap()
        .amount
        .u128();
    unregister_agent(&mut app, &croncat_agents_addr, AGENT0).unwrap();
    let new_balance = app
        .wrap()
        .query_balance(AGENT_BENEFICIARY, NATIVE_DENOM)
        .unwrap()
        .amount
        .u128();

    //Check balances are not changed, as we don't have any rewards to withdraw
    assert_eq!(old_balance, 500000);
    assert_eq!(new_balance, 500000);
}

#[test]
fn test_query_get_agent_tasks() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr: _,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr,
    } = init_test_scope(&mut app);

    let block_info = app.block_info();

    // Register AGENT0, who immediately becomes active
    register_agent(&mut app, &croncat_agents_addr, AGENT0, AGENT_BENEFICIARY).unwrap();

    // Add five tasks total
    // Three of them are block-based
    add_block_task_exec(&mut app, &croncat_tasks_addr, ANYONE, block_info.height + 6);
    add_block_task_exec(
        &mut app,
        &croncat_tasks_addr,
        ANYONE,
        block_info.height + 66,
    );
    add_block_task_exec(
        &mut app,
        &croncat_tasks_addr,
        ANYONE,
        block_info.height + 67,
    );

    // Two tasks use Cron instead of Block (for task interval)
    add_cron_task_exec(&mut app, &croncat_tasks_addr, ANYONE, 6); // 3 minutes
    add_cron_task_exec(&mut app, &croncat_tasks_addr, ANYONE, 53); // 53 minutes
    let total_tasks = get_total_tasks(&mut app, &croncat_tasks_addr).unwrap();
    assert_eq!(total_tasks, 5);

    // Fast forward time a little
    app.update_block(|block| add_seconds_to_block(block, 6 * 666));
    app.update_block(|block| increment_block_height(block, Some(666)));

    // What happens when the only active agent queries to see if there's work for them
    // calls:
    // fn query_get_agent_tasks
    let agent_tasks_res = get_agent_tasks(&mut app, &croncat_agents_addr, AGENT0);
    assert!(agent_tasks_res.is_ok(),);
    // Agent gets all tasks
    assert_eq!(
        agent_tasks_res.unwrap(),
        AgentTasksResponse {
            total_block_tasks: 3u64.into(),
            total_cron_tasks: 2u64.into()
        }
    );

    // Now the task ratio is 1:2 (one agent per two tasks)
    // Register two agents, the first one succeeding
    register_agent(&mut app, &croncat_agents_addr, AGENT1, AGENT_BENEFICIARY).unwrap();
    let check_in_res = check_in_agent(&mut app, &croncat_agents_addr, AGENT1);
    assert!(check_in_res.is_ok());
    // This next agent should fail because there's no enough tasks yet
    // Later, we'll have this agent try to nominate themselves before their time
    register_agent(&mut app, &croncat_agents_addr, AGENT2, AGENT_BENEFICIARY).unwrap();
    let failed_check_in_res = check_in_agent(&mut app, &croncat_agents_addr, AGENT2).unwrap_err();
    assert_eq!(
        ContractError::TryLaterForNomination,
        failed_check_in_res.downcast().unwrap()
    );

    let (_, num_active_agents, num_pending_agents) = get_agent_ids(&app, &croncat_agents_addr);
    assert_eq!(2, num_active_agents);
    assert_eq!(1, num_pending_agents);

    // Fast forward time a little
    app.update_block(|block| add_seconds_to_block(block, 6 * 666));
    app.update_block(|block| increment_block_height(block, Some(666)));

    // What happens when the first active agent queries to see if there's work for them
    // calls:
    // fn query_get_agent_tasks
    let agent_tasks_res = get_agent_tasks(&mut app, &croncat_agents_addr, AGENT0);
    assert!(agent_tasks_res.is_ok());
    assert_eq!(
        agent_tasks_res.unwrap(),
        AgentTasksResponse {
            total_block_tasks: 2u64.into(),
            total_cron_tasks: 1u64.into()
        }
    );

    // For the second agent
    let agent_tasks_res = get_agent_tasks(&mut app, &croncat_agents_addr, AGENT1);
    assert!(agent_tasks_res.is_ok());
    assert_eq!(
        agent_tasks_res.unwrap(),
        AgentTasksResponse {
            total_block_tasks: 1u64.into(),
            total_cron_tasks: 1u64.into()
        }
    );

    // Should fail for random user not in the active queue
    let agent_tasks_res = get_agent_tasks(&mut app, &croncat_agents_addr, AGENT2);
    let result = agent_tasks_res.unwrap();
    assert_eq!(
        result,
        AgentTasksResponse {
            total_block_tasks: Uint64::zero(),
            total_cron_tasks: Uint64::zero(),
        }
    );
}

//Tick
#[test]
fn test_tick() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr: _,
    } = init_test_scope(&mut app);

    // Change settings, the agent can miss 1000 blocks
    let update_config_msg = ExecuteMsg::UpdateConfig {
        config: UpdateConfig {
            owner_addr: Some(ADMIN.to_owned()),
            croncat_factory_addr: Some(croncat_factory_addr.to_string()),
            croncat_manager_key: Some(("manager".to_owned(), [0, 1])),
            croncat_tasks_key: Some(("tasks".to_owned(), [0, 1])),
            paused: None,
            min_tasks_per_agent: None,
            min_coins_for_agent_registration: Some(DEFAULT_MIN_COINS_FOR_AGENT_REGISTRATION),
            agent_nomination_duration: Some(DEFAULT_NOMINATION_BLOCK_DURATION),
            max_slot_passover: Some(1000), // allow to miss 1000 slots
            min_active_reserve: Some(0),
        },
    };
    app.execute_contract(
        Addr::unchecked(ADMIN),
        croncat_agents_addr.clone(),
        &update_config_msg,
        &[],
    )
    .unwrap();

    register_agent(&mut app, &croncat_agents_addr, AGENT0, AGENT_BENEFICIARY).unwrap();

    app.update_block(|info| increment_block_height(info, Some(1001)));
    app.update_block(|info| add_seconds_to_block(info, 19));

    let res = tick(&mut app, &croncat_agents_addr, ANYONE).unwrap();
    // Check attributes
    assert!(res.events.iter().any(|ev| ev
        .attributes
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "tick")));
    assert!(res.events.iter().any(|ev| ev
        .attributes
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "unregister_agent")));
    assert!(res.events.iter().any(|ev| ev
        .attributes
        .iter()
        .any(|attr| attr.key == "account_id" && attr.value == AGENT0)));
    assert!(res.events.iter().any(|ev| ev
        .attributes
        .iter()
        .any(|attr| attr.key == "lifecycle" && attr.value == "tick_failure")));

    // The agent missed 1001 blocks and he was unregistered
    // Pending agents weren't deleted
    let agents: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert!(agents.active.is_empty());
    assert!(agents.pending.is_empty());

    register_agent(&mut app, &croncat_agents_addr, AGENT0, AGENT_BENEFICIARY).unwrap();
    register_agent(&mut app, &croncat_agents_addr, AGENT1, AGENT_BENEFICIARY).unwrap();
    register_agent(&mut app, &croncat_agents_addr, AGENT2, AGENT_BENEFICIARY).unwrap();

    // need block advancement
    app.update_block(|info| add_seconds_to_block(info, 19));

    // Call tick
    // Not enough time passed to delete the agent
    let res = tick(&mut app, &croncat_agents_addr, AGENT0).unwrap();

    // Check attributes
    assert!(res.events.iter().any(|ev| ev
        .attributes
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "tick")));
    assert!(!res.events.iter().any(|ev| ev
        .attributes
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "unregister_agent")));

    let agents: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(agents.active.len(), 1);
    assert_eq!(agents.pending.len(), 2);

    // Agent pending agent wasn't nominated
    let err = app
        .execute_contract(
            Addr::unchecked(AGENT1),
            croncat_agents_addr.clone(),
            &ExecuteMsg::CheckInAgent {},
            &[],
        )
        .unwrap_err();
    assert_eq!(
        ContractError::TryLaterForNomination,
        err.downcast().unwrap()
    );

    // Add enough blocks to call tick
    app.update_block(|info| increment_block_height(info, Some(1001)));

    let res = tick(&mut app, &croncat_agents_addr, ANYONE).unwrap();

    // Check attributes
    assert!(res.events.iter().any(|ev| ev
        .attributes
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "tick")));
    assert!(res.events.iter().any(|ev| ev
        .attributes
        .iter()
        .any(|attr| attr.key == "action" && attr.value == "unregister_agent")));
    assert!(res.events.iter().any(|ev| ev
        .attributes
        .iter()
        .any(|attr| attr.key == "account_id" && attr.value == AGENT0)));

    // The agent missed 1001 blocks and he was unregistered
    // Pending agents weren't deleted
    let agents: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert!(agents.active.is_empty());
    assert_eq!(agents.pending.len(), 2);

    // AGENT1 was nominated and can call CheckInAgent
    app.execute_contract(
        Addr::unchecked(AGENT1),
        croncat_agents_addr.clone(),
        &ExecuteMsg::CheckInAgent {},
        &[],
    )
    .unwrap();
    // Second agent wasn't nominated
    let err = app
        .execute_contract(
            Addr::unchecked(AGENT2),
            croncat_agents_addr.clone(),
            &ExecuteMsg::CheckInAgent {},
            &[],
        )
        .unwrap_err();
    assert_eq!(
        ContractError::TryLaterForNomination,
        err.downcast().unwrap()
    );
}
#[test]
fn test_tick_respects_min_active_agent_count() {
    let mut app = default_app();
    let TestScope {
        croncat_factory_addr: _,
        croncat_agents_addr,
        croncat_agents_code_id: _,
        croncat_manager_addr: _,
        croncat_tasks_addr,
    } = init_test_scope(&mut app);

    register_agent(&mut app, &croncat_agents_addr, AGENT0, AGENT_BENEFICIARY).unwrap();
    register_agent(&mut app, &croncat_agents_addr, AGENT1, AGENT_BENEFICIARY).unwrap();

    create_task(&mut app, croncat_tasks_addr.as_ref(), ADMIN, PARTICIPANT0).unwrap();
    create_task(&mut app, croncat_tasks_addr.as_ref(), ADMIN, PARTICIPANT1).unwrap();
    create_task(&mut app, croncat_tasks_addr.as_ref(), ADMIN, PARTICIPANT2).unwrap();
    create_task(&mut app, croncat_tasks_addr.as_ref(), ADMIN, PARTICIPANT3).unwrap();

    app.update_block(|info| increment_block_height(info, Some(1001)));
    app.update_block(|info| add_seconds_to_block(info, 24 * 60));

    check_in_agent(&mut app, &croncat_agents_addr, AGENT1).unwrap();

    // The agent missed 1001 blocks and he was unregistered
    // Pending agents weren't deleted
    let agents: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    assert_eq!(agents.active.len(), 2);

    app.update_block(|info| add_seconds_to_block(info, 1000));

    tick(&mut app, &croncat_agents_addr, ANYONE).unwrap();

    // The agent0 missed 1000 blocks and he was unregistered, but still should not be removed
    let agents: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr.clone(),
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();

    assert_eq!(agents.active.len() as u16, DEFAULT_MIN_ACTIVE_RESERVE);
}
fn register_agent(
    app: &mut App,
    croncat_agents_addr: &Addr,
    agent: &str,
    beneficiary: &str,
) -> Result<AppResponse, anyhow::Error> {
    app.execute_contract(
        Addr::unchecked(agent),
        croncat_agents_addr.clone(),
        &ExecuteMsg::RegisterAgent {
            payable_account_id: Some(beneficiary.to_string()),
        },
        &[],
    )
}
fn unregister_agent(
    app: &mut App,
    croncat_agents_addr: &Addr,
    agent: &str,
) -> Result<AppResponse, anyhow::Error> {
    app.execute_contract(
        Addr::unchecked(agent),
        croncat_agents_addr.clone(),
        &ExecuteMsg::UnregisterAgent {},
        &[],
    )
}
fn tick(
    app: &mut App,
    croncat_agents_addr: &Addr,
    sender: &str,
) -> Result<AppResponse, anyhow::Error> {
    app.execute_contract(
        Addr::unchecked(sender),
        croncat_agents_addr.clone(),
        &ExecuteMsg::Tick {},
        &[],
    )
}
fn get_agent_ids(app: &App, croncat_agents_addr: &Addr) -> (GetAgentIdsResponse, usize, usize) {
    let res: GetAgentIdsResponse = app
        .wrap()
        .query_wasm_smart(
            croncat_agents_addr,
            &QueryMsg::GetAgentIds {
                from_index: None,
                limit: None,
            },
        )
        .unwrap();
    (res.clone(), res.active.len(), res.pending.len())
}

fn get_agent_status(
    app: &mut App,
    croncat_agents_addr: &Addr,
    agent: &str,
) -> Result<AgentResponse, anyhow::Error> {
    let agent_info: AgentResponse = app.wrap().query_wasm_smart(
        croncat_agents_addr,
        &QueryMsg::GetAgent {
            account_id: agent.to_string(),
        },
    )?;

    Ok(agent_info)
}

fn get_agent_tasks(
    app: &mut App,
    croncat_agents_addr: &Addr,
    agent: &str,
) -> Result<AgentTasksResponse, anyhow::Error> {
    let agent_info: AgentTasksResponse = app.wrap().query_wasm_smart(
        croncat_agents_addr,
        &QueryMsg::GetAgentTasks {
            account_id: agent.to_string(),
        },
    )?;

    Ok(agent_info)
}

fn get_total_tasks(app: &mut App, croncat_agents_addr: &Addr) -> Result<u64, anyhow::Error> {
    let total_tasks: Uint64 = app.wrap().query_wasm_smart(
        croncat_agents_addr,
        &croncat_sdk_tasks::msg::TasksQueryMsg::TasksTotal {},
    )?;

    Ok(total_tasks.u64())
}
fn check_in_agent(
    app: &mut App,
    croncat_agents_addr: &Addr,
    agent: &str,
) -> Result<AppResponse, anyhow::Error> {
    app.execute_contract(
        Addr::unchecked(agent),
        croncat_agents_addr.clone(),
        &ExecuteMsg::CheckInAgent {},
        &[],
    )
}

fn create_task(
    app: &mut App,
    tasks_addr: &str,
    sender: &str,
    receiver: &str,
) -> Result<AppResponse, anyhow::Error> {
    let send_funds = coins(100_000, NATIVE_DENOM);
    let action = Action {
        msg: BankMsg::Send {
            to_address: receiver.to_owned(),
            amount: coins(5, NATIVE_DENOM),
        }
        .into(),
        gas_limit: Some(50_000),
    };
    let request = TaskRequest {
        interval: Interval::Immediate,
        boundary: None,
        stop_on_fail: false,
        actions: vec![action],
        queries: None,
        transforms: None,
        cw20: None,
    };
    app.execute_contract(
        Addr::unchecked(sender),
        Addr::unchecked(tasks_addr),
        &croncat_tasks::msg::ExecuteMsg::CreateTask {
            task: Box::new(request),
        },
        &send_funds,
    )
}

fn add_block_task_exec(
    app: &mut App,
    task_contract_addr: &Addr,
    sender: &str,
    block_num: u64,
) -> AppResponse {
    let send_funds = coins(50_000, NATIVE_DENOM);
    let action = Action {
        msg: BankMsg::Send {
            to_address: PARTICIPANT0.to_owned(),
            amount: coins(5, NATIVE_DENOM),
        }
        .into(),
        gas_limit: Some(50_000),
    };
    let request = TaskRequest {
        interval: Interval::Block(block_num),
        boundary: None,
        stop_on_fail: false,
        actions: vec![action],
        queries: None,
        transforms: None,
        cw20: None,
    };
    app.execute_contract(
        Addr::unchecked(sender),
        task_contract_addr.clone(),
        &croncat_tasks::msg::ExecuteMsg::CreateTask {
            task: Box::new(request),
        },
        send_funds.as_ref(),
    )
    .expect("Error adding task")
}

fn add_cron_task_exec(
    app: &mut App,
    task_contract_addr: &Addr,
    sender: &str,
    num_minutes: u64,
) -> AppResponse {
    let send_funds = coins(50_000, NATIVE_DENOM);
    let action = Action {
        msg: BankMsg::Send {
            to_address: PARTICIPANT0.to_owned(),
            amount: coins(5, NATIVE_DENOM),
        }
        .into(),
        gas_limit: Some(50_000),
    };
    let request = TaskRequest {
        interval: Interval::Cron(format!("* {} * * * *", num_minutes)),
        boundary: None,
        stop_on_fail: false,
        actions: vec![action],
        queries: None,
        transforms: None,
        cw20: None,
    };
    app.execute_contract(
        Addr::unchecked(sender),
        task_contract_addr.clone(),
        &croncat_tasks::msg::ExecuteMsg::CreateTask {
            task: Box::new(request),
        },
        send_funds.as_ref(),
    )
    .expect("Error adding task")
}
