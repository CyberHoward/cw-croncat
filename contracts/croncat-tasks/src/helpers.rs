use cosmwasm_std::{
    Addr, BankMsg, Binary, BlockInfo, CosmosMsg, Deps, Empty, Order, QuerierWrapper, StdError,
    StdResult, Storage, WasmMsg, WasmQuery,
};
use croncat_sdk_tasks::types::{
    AmountForOneTask, Boundary, BoundaryHeight, BoundaryTime, Config, CosmosQuery, Interval, Task,
    TaskRequest,
};
use cw20::{Cw20CoinVerified, Cw20ExecuteMsg};
use serde_cw_value::Value;

use crate::{
    state::{tasks_map, BLOCK_SLOTS, EVENTED_TASKS_LOOKUP, TASKS_TOTAL, TIME_SLOTS},
    ContractError,
};

pub(crate) fn validate_boundary(
    block_info: &BlockInfo,
    boundary: Option<Boundary>,
    interval: &Interval,
) -> Result<Boundary, ContractError> {
    match (interval, boundary) {
        (Interval::Cron(_), Some(Boundary::Time(boundary_time))) => {
            let starting_time = boundary_time.start.unwrap_or(block_info.time);
            if boundary_time.end.map_or(false, |e| e <= starting_time) {
                Err(ContractError::InvalidBoundary {})
            } else {
                Ok(Boundary::Time(boundary_time))
            }
        }
        (
            Interval::Block(_) | Interval::Once | Interval::Immediate,
            Some(Boundary::Height(boundary_height)),
        ) => {
            let starting_height = boundary_height
                .start
                .map(Into::into)
                .unwrap_or(block_info.height);
            if boundary_height
                .end
                .map_or(false, |e| e.u64() <= starting_height)
            {
                Err(ContractError::InvalidBoundary {})
            } else {
                Ok(Boundary::Height(boundary_height))
            }
        }
        (Interval::Cron(_), None) => Ok(Boundary::Time(BoundaryTime {
            start: None,
            end: None,
        })),
        (_, None) => Ok(Boundary::Height(BoundaryHeight {
            start: None,
            end: None,
        })),
        _ => Err(ContractError::InvalidBoundary {}),
    }
}

/// Query against all to validate the query is possible, rather than open ended failures
/// This does NOT evaluate the contents which could change, allowing reactivity later.
/// Errors are assessed against contract and method availability
pub(crate) fn validate_queries(deps: &Deps, queries: &Vec<CosmosQuery>) -> bool {
    if queries.is_empty() {
        // no queries, so nothing to check
        return true;
    }

    // Process all the queries
    for query in queries {
        match query {
            CosmosQuery::Croncat(q) => {
                let res: Result<mod_sdk::types::QueryResponse, StdError> = deps.querier.query(
                    &WasmQuery::Smart {
                        contract_addr: q.contract_addr.clone(),
                        msg: q.msg.clone(),
                    }
                    .into(),
                );
                // only handle error
                if res.is_err() {
                    return false;
                }
            }
            CosmosQuery::Wasm(wq) => {
                // Cover all native wasm query types
                match wq {
                    WasmQuery::Smart { contract_addr, msg } => {
                        let data: Result<Value, StdError> = deps.querier.query(
                            &WasmQuery::Smart {
                                contract_addr: contract_addr.clone().to_string(),
                                msg: msg.clone(),
                            }
                            .into(),
                        );
                        // only handle error
                        if data.is_err() {
                            return false;
                        }
                    }
                    WasmQuery::Raw { contract_addr, key } => {
                        let res = deps
                            .querier
                            .query_wasm_raw(contract_addr.clone().to_string(), key.clone());
                        // only handle error
                        if res.is_err() {
                            return false;
                        }
                    }
                    WasmQuery::ContractInfo { contract_addr } => {
                        let res = deps
                            .querier
                            .query_wasm_contract_info(contract_addr.clone().to_string());
                        // only handle error
                        if res.is_err() {
                            return false;
                        }
                    }
                    // // NOTE: This is dependent on features = ["cosmwasm_1_2"]
                    // WasmQuery::CodeInfo { code_id } => {
                    //     let res = deps.querier.query_wasm_code_info(*code_id);
                    // }
                    _ => {
                        return false;
                    }
                }
            }
        }
    }

    true
}

/// Transforms need valid indexes, valid action paths
/// NOTE: Cannot validate the query response path, as it is determined at time of execution
pub(crate) fn validate_transforms(task: &Task) -> bool {
    for transform in task.transforms.iter() {
        // Validate transform index range
        if (transform.action_idx as usize > task.actions.len() - 1)
            || (transform.query_idx as usize > task.queries.len() - 1)
        {
            return false;
        }

        // Validate action path
        if let Some(action) = task.actions.get(transform.action_idx as usize) {
            // NOTE: This only covers the supported methods known to valid task actions!
            match &action.msg {
                CosmosMsg::Wasm(WasmMsg::Execute { msg, .. }) => {
                    let mut action_value = cosmwasm_std::from_binary(msg)
                        .map_err(|e| StdError::generic_err(e.to_string()))
                        .unwrap();
                    if transform.action_path.find_value(&mut action_value).is_err() {
                        return false;
                    }
                }
                CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
                    let mut action_value = serde_json_wasm::from_str::<Value>(&format!(
                        r#"{{"bank":{{"send":{{"to_address": "{}", "amount": {}}}}}}}"#,
                        to_address,
                        serde_json_wasm::to_string(amount).unwrap()
                    ))
                    .unwrap();
                    if transform.action_path.find_value(&mut action_value).is_err() {
                        return false;
                    }
                }
                _ => return false,
            }
        }
    }
    true
}

/// Check for calls of our contracts
pub(crate) fn check_for_self_calls(
    tasks_addr: &Addr,
    manager_addr: &Addr,
    agents_addr: &Addr,
    owner_addr: &Addr,
    sender: &Addr,
    contract_addr: &Addr,
    msg: &Binary,
) -> Result<(), ContractError> {
    // If its one of the our contracts it should be a agents contract only
    if contract_addr == tasks_addr || contract_addr == manager_addr {
        return Err(ContractError::InvalidAction {});
    } else if contract_addr == agents_addr {
        // Check if caller is manager owner
        if sender != owner_addr {
            return Err(ContractError::InvalidAction {});
        } else if let Ok(msg) = cosmwasm_std::from_binary(msg) {
            // Check if it's tick
            if !matches!(msg, croncat_sdk_agents::msg::ExecuteMsg::Tick {}) {
                return Err(ContractError::InvalidAction {});
            }
            // Other messages not allowed
        } else {
            return Err(ContractError::InvalidAction {});
        }
    }
    Ok(())
}

pub(crate) fn validate_msg_calculate_usage(
    deps: Deps,
    task: &TaskRequest,
    self_addr: &Addr,
    sender: &Addr,
    config: &Config,
) -> Result<AmountForOneTask, ContractError> {
    let manager_addr = get_manager_addr(&deps.querier, config)?;
    let agents_addr = get_agents_addr(&deps.querier, config)?;

    let manager_config: croncat_sdk_manager::types::Config = deps.querier.query_wasm_smart(
        &manager_addr,
        &croncat_sdk_manager::msg::ManagerQueryMsg::Config {},
    )?;

    let mut amount_for_one_task = AmountForOneTask {
        cw20: None,
        coin: [None, None],
        gas: config.gas_base_fee,
        agent_fee: manager_config.agent_fee,
        treasury_fee: manager_config.treasury_fee,
        gas_price: manager_config.gas_price,
    };

    if task.actions.is_empty() {
        return Err(ContractError::InvalidAction {});
    }
    for action in task.actions.iter() {
        amount_for_one_task.add_gas(action.gas_limit.unwrap_or(config.gas_action_fee));

        match &action.msg {
            CosmosMsg::Wasm(WasmMsg::Execute {
                contract_addr,
                funds,
                msg,
            }) => {
                if action.gas_limit.is_none() {
                    return Err(ContractError::NoGasLimit {});
                }
                for coin in funds {
                    if coin.amount.is_zero() || !amount_for_one_task.add_coin(coin.clone())? {
                        return Err(ContractError::InvalidAction {});
                    }
                }
                check_for_self_calls(
                    &deps.api.addr_validate(self_addr.as_str())?,
                    &deps.api.addr_validate(manager_addr.as_str())?,
                    &deps.api.addr_validate(agents_addr.as_str())?,
                    &deps.api.addr_validate(config.owner_addr.as_str())?,
                    &deps.api.addr_validate(sender.as_str())?,
                    &deps.api.addr_validate(contract_addr.as_str())?,
                    msg,
                )?;
                if let Ok(cw20_msg) = cosmwasm_std::from_binary(msg) {
                    match cw20_msg {
                        Cw20ExecuteMsg::Send { amount, .. } if !amount.is_zero() => {
                            let c = amount_for_one_task.add_cw20(Cw20CoinVerified {
                                address: deps.api.addr_validate(contract_addr)?,
                                amount,
                            });
                            if c.is_err() || !c.unwrap() {
                                return Err(ContractError::InvalidAction {});
                            }
                        }
                        Cw20ExecuteMsg::Transfer { amount, .. } if !amount.is_zero() => {
                            let c = amount_for_one_task.add_cw20(Cw20CoinVerified {
                                address: deps.api.addr_validate(contract_addr)?,
                                amount,
                            });
                            if c.is_err() || !c.unwrap() {
                                return Err(ContractError::InvalidAction {});
                            }
                        }
                        _ => {
                            return Err(ContractError::InvalidAction {});
                        }
                    }
                }
            }
            CosmosMsg::Bank(BankMsg::Send { to_address, amount }) => {
                // Only valid addresses
                if deps.api.addr_validate(to_address).is_err() {
                    return Err(ContractError::InvalidAddress {});
                }
                // Restrict no-coin transfer
                if amount.is_empty() {
                    return Err(ContractError::InvalidAction {});
                }
                for coin in amount {
                    // Zero coins will fail the transaction
                    if coin.amount.is_zero() || !amount_for_one_task.add_coin(coin.clone())? {
                        return Err(ContractError::InvalidAction {});
                    }
                }
            }
            // Disallow unknown messages
            _ => {
                return Err(ContractError::InvalidAction {});
            }
        }
    }

    if let Some(queries) = &task.queries {
        amount_for_one_task.add_gas(queries.len() as u64 * config.gas_query_fee)
    }
    Ok(amount_for_one_task)
}

pub(crate) fn remove_task(
    storage: &mut dyn Storage,
    hash: &[u8],
    is_block: bool,
    is_evented: bool,
) -> StdResult<()> {
    tasks_map().remove(storage, hash)?;
    TASKS_TOTAL.update(storage, |total| StdResult::Ok(total - 1))?;
    if is_evented {
        let hashes = EVENTED_TASKS_LOOKUP
            .range(storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<_>>>()?;
        for (hid, mut all_hashes) in hashes {
            let mut found = false;
            all_hashes.retain(|h| {
                found = h == hash;
                !found
            });
            if found {
                if all_hashes.is_empty() {
                    EVENTED_TASKS_LOOKUP.remove(storage, hid);
                } else {
                    EVENTED_TASKS_LOOKUP.save(storage, hid, &all_hashes)?;
                }
                break;
            }
        }
    } else if is_block {
        let blocks = BLOCK_SLOTS
            .range(storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<_>>>()?;
        for (bid, mut block_hashes) in blocks {
            let mut found = false;
            block_hashes.retain(|h| {
                found = h == hash;
                !found
            });
            if found {
                if block_hashes.is_empty() {
                    BLOCK_SLOTS.remove(storage, bid);
                } else {
                    BLOCK_SLOTS.save(storage, bid, &block_hashes)?;
                }
                break;
            }
        }
    } else {
        let time_buckets = TIME_SLOTS
            .range(storage, None, None, Order::Ascending)
            .collect::<StdResult<Vec<_>>>()?;
        for (tid, mut time_hashes) in time_buckets {
            let mut found = false;
            time_hashes.retain(|h| {
                found = h == hash;
                !found
            });
            if found {
                if time_hashes.is_empty() {
                    TIME_SLOTS.remove(storage, tid);
                } else {
                    TIME_SLOTS.save(storage, tid, &time_hashes)?;
                }
                break;
            }
        }
    }

    Ok(())
}

pub(crate) fn check_if_sender_is_manager(
    deps_queries: &QuerierWrapper<Empty>,
    config: &Config,
    sender: &Addr,
) -> Result<(), ContractError> {
    let manager_addr = get_manager_addr(deps_queries, config)?;
    if manager_addr != *sender {
        return Err(ContractError::Unauthorized {});
    }

    Ok(())
}

pub(crate) fn get_manager_addr(
    deps_queries: &QuerierWrapper<Empty>,
    config: &Config,
) -> Result<Addr, ContractError> {
    let (manager_name, version) = &config.croncat_manager_key;
    croncat_sdk_factory::state::CONTRACT_ADDRS
        .query(
            deps_queries,
            config.croncat_factory_addr.clone(),
            (manager_name, version),
        )?
        .ok_or(ContractError::InvalidKey {})
}

pub(crate) fn get_agents_addr(
    deps_queries: &QuerierWrapper<Empty>,
    config: &Config,
) -> Result<Addr, ContractError> {
    let (agents_name, version) = &config.croncat_agents_key;
    croncat_sdk_factory::state::CONTRACT_ADDRS
        .query(
            deps_queries,
            config.croncat_factory_addr.clone(),
            (agents_name, version),
        )?
        .ok_or(ContractError::InvalidKey {})
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{Timestamp, Uint64};

    use super::*;

    #[test]
    fn validate_boundary_cases() {
        type ValidateBoundaryChecker = (
            Interval,
            Option<Boundary>,
            // current block height
            u64,
            // current block timestamp
            Timestamp,
            // expected result
            Result<Boundary, ContractError>,
        );
        let cases: Vec<ValidateBoundaryChecker> = vec![
            // Boundary - None
            (
                Interval::Once,
                None,
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: None,
                    end: None,
                })),
            ),
            (
                Interval::Immediate,
                None,
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: None,
                    end: None,
                })),
            ),
            (
                Interval::Block(5),
                None,
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: None,
                    end: None,
                })),
            ),
            (
                Interval::Cron("* * * * * *".to_owned()),
                None,
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Time(BoundaryTime {
                    start: None,
                    end: None,
                })),
            ),
            // Boundary height, start&end - None
            (
                Interval::Once,
                Some(Boundary::Height(BoundaryHeight {
                    start: None,
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: None,
                    end: None,
                })),
            ),
            (
                Interval::Immediate,
                Some(Boundary::Height(BoundaryHeight {
                    start: None,
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: None,
                    end: None,
                })),
            ),
            (
                Interval::Block(5),
                Some(Boundary::Height(BoundaryHeight {
                    start: None,
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: None,
                    end: None,
                })),
            ),
            (
                Interval::Cron("* * * * * *".to_owned()),
                Some(Boundary::Height(BoundaryHeight {
                    start: None,
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Err(ContractError::InvalidBoundary {}),
            ),
            // Boundary Time - start&end - None
            (
                Interval::Once,
                Some(Boundary::Time(BoundaryTime {
                    start: None,
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Err(ContractError::InvalidBoundary {}),
            ),
            (
                Interval::Immediate,
                Some(Boundary::Time(BoundaryTime {
                    start: None,
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Err(ContractError::InvalidBoundary {}),
            ),
            (
                Interval::Block(5),
                Some(Boundary::Time(BoundaryTime {
                    start: None,
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Err(ContractError::InvalidBoundary {}),
            ),
            (
                Interval::Cron("* * * * * *".to_owned()),
                Some(Boundary::Time(BoundaryTime {
                    start: None,
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Time(BoundaryTime {
                    start: None,
                    end: None,
                })),
            ),
            // Start exactly now
            (
                Interval::Once,
                Some(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: None,
                })),
            ),
            (
                Interval::Immediate,
                Some(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: None,
                })),
            ),
            (
                Interval::Block(5),
                Some(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: None,
                })),
            ),
            (
                Interval::Cron("* * * * * *".to_owned()),
                Some(Boundary::Time(BoundaryTime {
                    start: Some(Timestamp::from_nanos(123456)),
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Time(BoundaryTime {
                    start: Some(Timestamp::from_nanos(123456)),
                    end: None,
                })),
            ),
            // Start 1 too early, we shouldn't check it
            (
                Interval::Once,
                Some(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(122)),
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(122)),
                    end: None,
                })),
            ),
            (
                Interval::Immediate,
                Some(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(122)),
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(122)),
                    end: None,
                })),
            ),
            (
                Interval::Block(5),
                Some(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(122)),
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(122)),
                    end: None,
                })),
            ),
            (
                Interval::Cron("* * * * * *".to_owned()),
                Some(Boundary::Time(BoundaryTime {
                    start: Some(Timestamp::from_nanos(123455)),
                    end: None,
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Time(BoundaryTime {
                    start: Some(Timestamp::from_nanos(123455)),
                    end: None,
                })),
            ),
            // Ok ends
            (
                Interval::Once,
                Some(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: Some(Uint64::new(124)),
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: Some(Uint64::new(124)),
                })),
            ),
            (
                Interval::Immediate,
                Some(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: Some(Uint64::new(124)),
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: Some(Uint64::new(124)),
                })),
            ),
            (
                Interval::Block(5),
                Some(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: Some(Uint64::new(124)),
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: Some(Uint64::new(124)),
                })),
            ),
            (
                Interval::Cron("* * * * * *".to_owned()),
                Some(Boundary::Time(BoundaryTime {
                    start: Some(Timestamp::from_nanos(123456)),
                    end: Some(Timestamp::from_nanos(123457)),
                })),
                123,
                Timestamp::from_nanos(123456),
                Ok(Boundary::Time(BoundaryTime {
                    start: Some(Timestamp::from_nanos(123456)),
                    end: Some(Timestamp::from_nanos(123457)),
                })),
            ),
            // End too early
            (
                Interval::Once,
                Some(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: Some(Uint64::new(123)),
                })),
                123,
                Timestamp::from_nanos(123456),
                Err(ContractError::InvalidBoundary {}),
            ),
            (
                Interval::Immediate,
                Some(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: Some(Uint64::new(123)),
                })),
                123,
                Timestamp::from_nanos(123456),
                Err(ContractError::InvalidBoundary {}),
            ),
            (
                Interval::Block(5),
                Some(Boundary::Height(BoundaryHeight {
                    start: Some(Uint64::new(123)),
                    end: Some(Uint64::new(123)),
                })),
                123,
                Timestamp::from_nanos(123456),
                Err(ContractError::InvalidBoundary {}),
            ),
            (
                Interval::Cron("* * * * * *".to_owned()),
                Some(Boundary::Time(BoundaryTime {
                    start: Some(Timestamp::from_nanos(123456)),
                    end: Some(Timestamp::from_nanos(123456)),
                })),
                123,
                Timestamp::from_nanos(123456),
                Err(ContractError::InvalidBoundary {}),
            ),
        ];
        for (interval, boundary, height, time, expected_res) in cases {
            let block_info = BlockInfo {
                height,
                time,
                chain_id: "cron".to_owned(),
            };
            let res = validate_boundary(&block_info, boundary, &interval);
            assert_eq!(res, expected_res)
        }
    }
}
