use cosmwasm_schema::{cw_serde, QueryResponses};
use cw20::Cw20Coin;

use crate::types::{SlotHashesResponse, SlotIdsResponse, Task, TaskRequest, TaskResponse};

#[cw_serde]
pub struct TasksInstantiateMsg {
    pub manager_addr: String,
}

#[cw_serde]
pub enum TasksExecuteMsg {
    CreateTask {
        task: TaskRequest,
    },
    RemoveTask {
        task_hash: String,
    },
    // TODO: how we interact balances with the manager
    RefillTaskBalance {
        task_hash: String,
    },
    RefillTaskCw20Balance {
        task_hash: String,
        cw20_coins: Vec<Cw20Coin>,
    },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum TasksQueryMsg {
    #[returns(Vec<TaskResponse>)]
    Tasks {
        from_index: Option<u64>,
        limit: Option<u64>,
    },
    #[returns(Vec<TaskResponse>)]
    TasksWithQueries {
        from_index: Option<u64>,
        limit: Option<u64>,
    },
    #[returns(Vec<TaskResponse>)]
    TasksByOwner { owner_addr: String },
    #[returns(TaskResponse)]
    Task { task_hash: String },
    #[returns(String)]
    TaskHash { task: Task },
    #[returns(SlotHashesResponse)]
    SlotHashes { slot: Option<u64> },
    #[returns(SlotIdsResponse)]
    SlotIds {
        from_index: Option<u64>,
        limit: Option<u64>,
    },
}
