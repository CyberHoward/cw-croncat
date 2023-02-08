use crate::types::*;
use cosmwasm_schema::{cw_serde, QueryResponses};
use cosmwasm_std::{Addr, Uint64};
use croncat_sdk_core::internal_messages::agents::{AgentOnTaskCompleted, AgentOnTaskCreated};

#[cw_serde]
pub struct InstantiateMsg {
    /// Address of the contract owner, defaults to the sender
    pub owner_addr: Option<String>,

    /// CW2 Version provided by factory
    pub version: Option<String>,

    /// Name of the key for raw querying Manager address from the factory
    pub croncat_manager_key: (String, [u8; 2]),

    /// Name of the key for raw querying Tasks address from the factory
    pub croncat_tasks_key: (String, [u8; 2]),

    /// Sets the amount of time opportunity for a pending agent to become active.
    /// If there is a pending queue, the longer a pending agent waits,
    /// the more pending agents can potentially become active based on this nomination window.
    /// This duration doesn't block the already nominated agent from becoming active,
    /// it only opens the door for more to become active. If a pending agent is nominated,
    /// then is lazy and beat by another agent, they get removed from pending queue and must
    /// register again.
    pub agent_nomination_duration: Option<u16>,

    /// The ratio used to calculate active agents/tasks. Example: "3", requires there are
    /// 4 tasks before letting in another agent to become active. (3 tasks for agent 1, 1 task for agent 2)
    pub min_tasks_per_agent: Option<u64>,

    /// The required amount needed to actually execute a few tasks before withdraw profits.
    /// This helps make sure agent wont get stuck out the gate
    pub min_coin_for_agent_registration: Option<u64>,

    /// How many slots an agent can miss before being removed from the active queue
    pub max_slot_passover: Option<u64>,

    /// Minimum agent count in active queue to be untouched by bad agent verifier
    pub min_active_reserve: Option<u16>,
}

/// Execute messages for agent contract
#[cw_serde]
pub enum ExecuteMsg {
    /// Action registers new agent
    RegisterAgent { payable_account_id: Option<String> },
    /// Action for updating agents
    UpdateAgent { payable_account_id: String },
    /// Action moves agent from pending to active list
    CheckInAgent {},
    /// Actions for removing agent from the system
    UnregisterAgent { },
    /// Task contract will send message when task is created
    OnTaskCreated(AgentOnTaskCreated),
    /// Task contract will send message when task is completed
    OnTaskCompleted(AgentOnTaskCompleted),
    /// Action for updating agent contract configuration
    UpdateConfig { config: UpdateConfig },
    /// Tick action will remove unactive agents periodically or do and any other internal cron tasks
    Tick {},
}

/// Agent request response
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    /// Get an agent by specified account_id, returns AgentInfo if found
    #[returns[AgentResponse]]
    GetAgent { account_id: String },
    /// Gets the id list of agents, pagination is supported
    #[returns[GetAgentIdsResponse]]
    GetAgentIds {
        from_index: Option<u64>,
        limit: Option<u64>,
    },
    /// Gets the specified agent tasks
    #[returns[AgentTasksResponse]]
    GetAgentTasks { account_id: String },

    /// Gets the agent contract configuration
    #[returns[crate::types::Config]]
    Config {},
}
/// Response containing active/pending agents
#[cw_serde]
pub struct GetAgentIdsResponse {
    /// Active agent list
    pub active: Vec<Addr>,
    /// Pending agent list
    pub pending: Vec<Addr>,
}

/// Agent response containing agent information
#[cw_serde]
pub struct AgentResponse {
    /// Agent data
    pub agent: Option<AgentInfo>,
}
/// Agent task response for getting stats and task information
#[cw_serde]
pub struct AgentTasksResponse {
    /// Agent cron tasks
    pub total_block_tasks: Uint64,
    /// Agent cron tasks
    pub total_cron_tasks: Uint64,
}
/// Updatable agents contract configuration
#[cw_serde]
pub struct UpdateConfig {
    /// Contract owner address
    pub owner_addr: Option<String>,

    /// Contract paused state, if contract is paused some action will not be available for execution
    pub paused: Option<bool>,

    /// Address of the factory contract
    pub croncat_factory_addr: Option<String>,

    /// Name of the key for raw querying Manager address from the factory
    pub croncat_manager_key: Option<(String, [u8; 2])>,

    /// Name of the key for raw querying Tasks address from the factory
    pub croncat_tasks_key: Option<(String, [u8; 2])>,

    /// Minimum tasks count to be reached by agent before next agent nomination
    pub min_tasks_per_agent: Option<u64>,

    /// Duration to be passed before next agent nomination
    pub agent_nomination_duration: Option<u16>,

    /// Minimum funds to be attached for agent registration
    pub min_coins_for_agent_registration: Option<u64>,

    /// How many slots an agent can miss before being removed from the active queue
    pub max_slot_passover: Option<u64>,

    /// Minimum agent count in active queue to be untouched by bad agent verifier
    pub min_active_reserve: Option<u16>,
}
