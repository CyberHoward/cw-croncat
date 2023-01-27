use crate::types::AgentStatus;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Timestamp, Uint128, Uint64};
use croncat_sdk_core::internal_messages::agents::AgentOnTaskCreated;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner_addr: Option<String>,
    /// Name of the key for raw querying Manager address from the factory
    pub croncat_manager_key: (String, [u8; 2]),
    /// Name of the key for raw querying Tasks address from the factory
    pub croncat_tasks_key: (String, [u8; 2]),

    pub agent_nomination_duration: Option<u16>,
    pub min_tasks_per_agent: Option<u64>,
    pub min_coin_for_agent_registration: Option<u64>,
}

#[cw_serde]
pub enum ExecuteMsg {
    RegisterAgent { payable_account_id: Option<String> },
    UpdateAgent { payable_account_id: String },
    CheckInAgent {},
    UnregisterAgent { from_behind: Option<bool> },
    //Task contract will send message when task is created
    OnTaskCreated(AgentOnTaskCreated),
    UpdateConfig { config: UpdateConfig },
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    #[returns[Option<AgentResponse>]]
    GetAgent { account_id: String },
    #[returns[Option<GetAgentIdsResponse>]]
    GetAgentIds {
        from_index: Option<u64>,
        limit: Option<u64>,
    },
    #[returns[Option<AgentTaskResponse>]]
    GetAgentTasks {
        account_id: String,
        block_slots: Option<u64>,
        cron_slots: Option<u64>,
    },
    #[returns[crate::types::Config]]
    Config {},
}

#[cw_serde]
pub struct GetAgentIdsResponse {
    pub active: Vec<Addr>,
    pub pending: Vec<Addr>,
}

#[cw_serde]
pub struct AgentResponse {
    pub status: AgentStatus,
    pub payable_account_id: Addr,
    pub balance: Uint128,
    pub total_tasks_executed: u64,
    pub last_executed_slot: u64,
    pub register_start: Timestamp,
}

#[cw_serde]
pub struct AgentTaskResponse {
    pub num_block_tasks: Uint64,
    pub num_cron_tasks: Uint64,
}

#[cw_serde]
pub struct UpdateConfig {
    pub owner_addr: Option<String>,
    pub paused: Option<bool>,
    /// Address of the factory contract
    pub croncat_factory_addr: Option<String>,
    /// Name of the key for raw querying Manager address from the factory
    pub croncat_manager_key: Option<(String, [u8; 2])>,
    /// Name of the key for raw querying Tasks address from the factory
    pub croncat_tasks_key: Option<(String, [u8; 2])>,

    pub min_tasks_per_agent: Option<u64>,
    pub agent_nomination_duration: Option<u16>,
    pub min_coins_for_agent_registration: Option<u64>,
}
