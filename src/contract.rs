#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Coin, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::agent::{
    accept_nomination_agent, query_get_agent, query_get_agent_ids, query_get_agent_tasks,
    register_agent, unregister_agent, update_agent, withdraw_task_balance,
};
use crate::error::ContractError;
use crate::helpers::GenericBalance;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::owner::{move_balances, query_balances, query_config, update_settings};
use crate::state::{Config, CONFIG};
use crate::tasks::{
    create_task, proxy_call, proxy_callback, query_get_task, refill_task, remove_task,
};
use cw20::Balance;

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:cw-croncat";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let mut available_balance = GenericBalance::default();

    // keep tally of balances initialized
    let state_balances = deps.querier.query_all_balances(&env.contract.address)?;
    available_balance.add_tokens(Balance::from(state_balances));
    available_balance.add_tokens(Balance::from(info.funds.clone()));

    let owner_acct = msg.owner_id.unwrap_or_else(|| info.sender.clone());
    // let owner_valid = deps.api.addr_validate(owner_acct.as_str())?;
    // println!("owner_valid {:?}", owner_valid);
    assert!(
        deps.api.addr_validate(owner_acct.as_str()).is_ok(),
        "Invalid address"
    );

    let config = Config {
        paused: false,
        owner_id: owner_acct,
        // treasury_id: None,
        agent_task_ratio: [1, 2],
        agent_active_index: 0,
        agents_eject_threshold: 600, // how many slots an agent can miss before being ejected. 10 * 60 = 1hr
        available_balance,
        staked_balance: GenericBalance::default(),
        agent_fee: Coin::new(5, msg.denom.clone()), // TODO: CHANGE AMOUNT HERE!!! 0.0005 Juno (2000 tasks = 1 Juno)
        gas_price: 1,
        proxy_callback_gas: 3,
        slot_granularity: 60_000_000_000,
        native_denom: msg.denom,
        cw20_whitelist: vec![],
        // TODO: ????
        // cw20_fees: vec![],
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &config)?;

    // all instantiated data
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("paused", config.paused.to_string())
        .add_attribute("owner_id", config.owner_id.to_string())
        // .add_attribute(
        //     "treasury_id",
        //     config
        //         .treasury_id
        //         .unwrap_or_else(|| Addr::unchecked(""))
        //         .to_string(),
        // )
        .add_attribute(
            "agent_task_ratio",
            config
                .agent_task_ratio
                .iter()
                .copied()
                .map(|i| i.to_string())
                .collect::<String>(),
        )
        .add_attribute("agent_active_index", config.agent_active_index.to_string())
        .add_attribute(
            "agents_eject_threshold",
            config.agents_eject_threshold.to_string(),
        )
        .add_attribute("native_denom", config.native_denom)
        .add_attribute("agent_fee", config.agent_fee.to_string())
        .add_attribute("gas_price", config.gas_price.to_string())
        .add_attribute("proxy_callback_gas", config.proxy_callback_gas.to_string())
        .add_attribute("slot_granularity", config.slot_granularity.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::UpdateSettings { .. } => update_settings(deps, info, msg),
        ExecuteMsg::MoveBalances {
            balances,
            account_id,
        } => move_balances(deps, info, env, balances, account_id),

        ExecuteMsg::RegisterAgent { payable_account_id } => {
            register_agent(deps, info, env, payable_account_id)
        }
        ExecuteMsg::UpdateAgent { payable_account_id } => {
            update_agent(deps, info, env, payable_account_id)
        }
        ExecuteMsg::UnregisterAgent {} => unregister_agent(deps, info, env),
        ExecuteMsg::WithdrawReward {} => withdraw_task_balance(deps, info, env),
        ExecuteMsg::CheckInAgent {} => accept_nomination_agent(deps, info, env),

        ExecuteMsg::CreateTask { task } => create_task(deps, info, env, task),
        ExecuteMsg::RemoveTask { task_hash } => remove_task(deps, info, env, task_hash),
        ExecuteMsg::RefillTaskBalance { task_hash } => refill_task(deps, info, env, task_hash),
        ExecuteMsg::ProxyCall {} => proxy_call(deps, info, env),
        ExecuteMsg::ProxyCallback {
            task_hash,
            current_slot,
        } => proxy_callback(deps, info, env, task_hash, current_slot),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query_config(deps)?),
        QueryMsg::GetBalances {} => to_binary(&query_balances(deps)?),

        QueryMsg::GetAgent { account_id } => to_binary(&query_get_agent(deps, account_id)?),
        QueryMsg::GetAgentIds {} => to_binary(&query_get_agent_ids(deps)?),
        QueryMsg::GetAgentTasks { account_id } => {
            to_binary(&query_get_agent_tasks(deps, account_id)?)
        }

        // slot,
        // from_index,
        // limit,
        QueryMsg::GetTasks { .. } => Ok(Binary::default()),
        QueryMsg::GetTasksByOwner { owner_id: _ } => Ok(Binary::default()),
        QueryMsg::GetTask { task_hash } => to_binary(&query_get_task(deps, task_hash)?),
        QueryMsg::GetTaskHash { task: _ } => Ok(Binary::default()),
        QueryMsg::ValidateInterval { interval: _ } => Ok(Binary::default()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::msg::{ConfigResponse, QueryMsg};
    use cosmwasm_std::testing::{mock_dependencies_with_balance, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, from_binary};

    #[test]
    fn configure() {
        let mut deps = mock_dependencies_with_balance(&coins(200, ""));

        let msg = InstantiateMsg {
            denom: "atom".to_string(),
            owner_id: None,
        };
        let info = mock_info("creator", &coins(1000, "meow"));

        // we can just call .unwrap() to assert this was a success
        let res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        assert_eq!(0, res.messages.len());

        // it worked, let's query the state
        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetConfig {}).unwrap();
        let value: ConfigResponse = from_binary(&res).unwrap();
        assert_eq!(false, value.paused);
        assert_eq!(info.sender, value.owner_id);
        // assert_eq!(None, value.treasury_id);
        assert_eq!([1, 2], value.agent_task_ratio);
        assert_eq!(0, value.agent_active_index);
        assert_eq!(600, value.agents_eject_threshold);
        assert_eq!("atom", value.native_denom);
        assert_eq!(coin(5, "atom"), value.agent_fee);
        assert_eq!(1, value.gas_price);
        assert_eq!(3, value.proxy_callback_gas);
        assert_eq!(60_000_000_000, value.slot_granularity);
    }
}
