/// Various helper functions allowing integrating projects to properly write tests
use crate::{contracts, AGENT, ALICE, BOB, CHARLIZE, DENOM, PAUSE_ADMIN, VERSION, VERY_RICH};
use cosmwasm_std::{coin, coins, to_binary, Addr, BlockInfo, Coin};
use croncat_sdk_factory::msg::{ContractMetadataResponse, ModuleInstantiateInfo, VersionKind};
use cw_multi_test::{App, AppBuilder, Executor};

pub struct CronCatTestEnv {
    pub app: App,
    pub factory: Addr,
    pub manager: Addr,
    pub tasks: Addr,
    pub agents: Addr,
}

pub fn set_up_croncat_contracts(app: Option<App>) -> CronCatTestEnv {
    let mut app = if let Some(passed_in_app) = app {
        passed_in_app
    } else {
        default_app()
    };

    let factory_addr = init_factory(&mut app);

    let manager_instantiate_msg: croncat_sdk_manager::msg::ManagerInstantiateMsg =
        default_manager_instantiate_message();
    let manager_addr = init_manager(
        &mut app,
        &manager_instantiate_msg,
        &factory_addr,
        &[coin(1, DENOM)],
    );

    let agents_instantiate_msg: croncat_sdk_agents::msg::InstantiateMsg =
        default_agents_instantiate_message();
    let agents_addr = init_agents(&mut app, &agents_instantiate_msg, &factory_addr, &[]);

    let tasks_instantiate_msg: croncat_sdk_tasks::msg::TasksInstantiateMsg =
        default_tasks_instantiate_msg();
    let tasks_addr = init_tasks(&mut app, &tasks_instantiate_msg, &factory_addr);

    CronCatTestEnv {
        app,
        factory: factory_addr,
        manager: manager_addr,
        tasks: tasks_addr,
        agents: agents_addr,
    }
}

pub fn default_app() -> App {
    AppBuilder::new().build(|router, _, storage| {
        let accounts: Vec<(u128, String)> = vec![
            (6_000_000, ALICE.to_string()),
            (6_000_000, AGENT.to_string()),
            (600_000, BOB.to_string()),
            (666_000, CHARLIZE.to_string()),
            (u128::MAX.saturating_sub(1000), VERY_RICH.to_string()),
        ];
        for (amt, address) in accounts {
            router
                .bank
                .init_balance(storage, &Addr::unchecked(address), coins(amt, DENOM))
                .unwrap();
        }
    })
}

pub fn init_agents(
    app: &mut App,
    msg: &croncat_sdk_agents::msg::InstantiateMsg,
    factory_addr: &Addr,
    funds: &[Coin],
) -> Addr {
    let code_id = app.store_code(contracts::croncat_agents_contract());

    let module_instantiate_info = ModuleInstantiateInfo {
        code_id,
        version: [0, 1],
        commit_id: "commit1".to_owned(),
        checksum: "checksum2".to_owned(),
        changelog_url: None,
        schema: None,
        msg: to_binary(msg).unwrap(),
        contract_name: "agents".to_owned(),
    };

    app.execute_contract(
        Addr::unchecked(ALICE),
        factory_addr.to_owned(),
        &croncat_sdk_factory::msg::FactoryExecuteMsg::Deploy {
            kind: VersionKind::Manager,
            module_instantiate_info,
        },
        funds,
    )
    .unwrap();

    let metadata: ContractMetadataResponse = app
        .wrap()
        .query_wasm_smart(
            factory_addr,
            &croncat_sdk_factory::msg::FactoryQueryMsg::LatestContract {
                contract_name: "agents".to_owned(),
            },
        )
        .unwrap();
    metadata.metadata.unwrap().contract_addr
}

pub fn init_factory(app: &mut App) -> Addr {
    let code_id = app.store_code(contracts::croncat_factory_contract());
    app.instantiate_contract(
        code_id,
        Addr::unchecked(ALICE),
        &croncat_sdk_factory::msg::FactoryInstantiateMsg { owner_addr: None },
        &[],
        "croncat_factory",
        None,
    )
    .unwrap()
}

pub fn init_manager(
    app: &mut App,
    msg: &croncat_sdk_manager::msg::ManagerInstantiateMsg,
    factory_addr: &Addr,
    funds: &[Coin],
) -> Addr {
    let code_id = app.store_code(contracts::croncat_manager_contract());

    let module_instantiate_info = ModuleInstantiateInfo {
        code_id,
        version: [0, 1],
        commit_id: "commit1".to_owned(),
        checksum: "checksum2".to_owned(),
        changelog_url: None,
        schema: None,
        msg: to_binary(msg).unwrap(),
        contract_name: "manager".to_owned(),
    };

    app.execute_contract(
        Addr::unchecked(ALICE),
        factory_addr.to_owned(),
        &croncat_sdk_factory::msg::FactoryExecuteMsg::Deploy {
            kind: VersionKind::Manager,
            module_instantiate_info,
        },
        funds,
    )
    .unwrap();

    let metadata: ContractMetadataResponse = app
        .wrap()
        .query_wasm_smart(
            factory_addr,
            &croncat_sdk_factory::msg::FactoryQueryMsg::LatestContract {
                contract_name: "manager".to_owned(),
            },
        )
        .unwrap();
    metadata.metadata.unwrap().contract_addr
}

pub fn default_tasks_instantiate_msg() -> croncat_sdk_tasks::msg::TasksInstantiateMsg {
    croncat_sdk_tasks::msg::TasksInstantiateMsg {
        chain_name: "atom".to_owned(),
        version: Some(VERSION.to_string()),
        pause_admin: Addr::unchecked(PAUSE_ADMIN),
        croncat_manager_key: ("manager".to_owned(), [0, 1]),
        croncat_agents_key: ("agents".to_owned(), [0, 1]),
        slot_granularity_time: None,
        gas_base_fee: None,
        gas_action_fee: None,
        gas_query_fee: None,
        gas_limit: None,
    }
}

pub fn default_manager_instantiate_message() -> croncat_sdk_manager::msg::ManagerInstantiateMsg {
    croncat_sdk_manager::msg::ManagerInstantiateMsg {
        version: Some(VERSION.to_owned()),
        croncat_tasks_key: ("tasks".to_owned(), [0, 1]),
        croncat_agents_key: ("agents".to_owned(), [0, 1]),
        pause_admin: Addr::unchecked(PAUSE_ADMIN),
        gas_price: None,
        treasury_addr: None,
        cw20_whitelist: None,
    }
}

pub fn default_agents_instantiate_message() -> croncat_sdk_agents::msg::InstantiateMsg {
    croncat_sdk_agents::msg::InstantiateMsg {
        version: Some(VERSION.to_string()),
        croncat_manager_key: ("manager".to_string(), [0, 1]),
        croncat_tasks_key: ("tasks".to_owned(), [0, 1]),
        agent_nomination_duration: None,
        min_tasks_per_agent: None,
        min_coins_for_agent_registration: None,
        agents_eject_threshold: None,
        min_active_agent_count: None,
        public_registration: false,
        pause_admin: Addr::unchecked(PAUSE_ADMIN),
        allowed_agents: Some(vec![AGENT.to_string()]),
    }
}

pub fn init_tasks(
    app: &mut App,
    msg: &croncat_sdk_tasks::msg::TasksInstantiateMsg,
    factory_addr: &Addr,
) -> Addr {
    let code_id = app.store_code(contracts::croncat_tasks_contract());
    let module_instantiate_info = ModuleInstantiateInfo {
        code_id,
        version: [0, 1],
        commit_id: "commit1".to_owned(),
        checksum: "checksum2".to_owned(),
        changelog_url: None,
        schema: None,
        msg: to_binary(msg).unwrap(),
        contract_name: "tasks".to_owned(),
    };
    app.execute_contract(
        Addr::unchecked(ALICE),
        factory_addr.to_owned(),
        &croncat_sdk_factory::msg::FactoryExecuteMsg::Deploy {
            kind: VersionKind::Tasks,
            module_instantiate_info,
        },
        &[],
    )
    .unwrap();

    let metadata: ContractMetadataResponse = app
        .wrap()
        .query_wasm_smart(
            factory_addr,
            &croncat_sdk_factory::msg::FactoryQueryMsg::LatestContract {
                contract_name: "tasks".to_owned(),
            },
        )
        .unwrap();

    metadata.metadata.unwrap().contract_addr
}

pub fn add_seconds_to_block(block: &mut BlockInfo, seconds: u64) {
    block.time = block.time.plus_seconds(seconds);
}

pub fn increment_block_height(block: &mut BlockInfo, inc_value: Option<u64>) {
    block.height += inc_value.unwrap_or(1);
}
