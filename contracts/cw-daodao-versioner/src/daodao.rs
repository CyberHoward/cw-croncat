use cosmwasm_std::{to_binary, CosmosMsg, Empty, Response, WasmMsg};

use crate::error::ContractError;
use cwd_pre_propose_base::msg::ExecuteMsg as ExecuteBase;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
pub type ExecuteMsg = ExecuteBase<ProposeMessage, Empty>;

#[derive(Serialize, JsonSchema, Deserialize, Debug, Clone)]
#[serde(rename_all = "snake_case")]
pub enum ProposeMessage {
    Propose {
        title: String,
        description: String,
        msgs: Vec<CosmosMsg<Empty>>,
    },
}

pub fn create_daodao_proposal(
    daodao_addr: String,
    name: String,
    chain_id: String,
    _proposer: String,
) -> Result<Response, ContractError> {
    let proposal = ProposeMessage::Propose {
        title: format!("{name}{chain_id}"),
        description: format!("{name}{chain_id}"),
        msgs: vec![],
    };
    let msg = CosmosMsg::Wasm(WasmMsg::Execute {
        contract_addr: daodao_addr,
        msg: to_binary(&ExecuteMsg::Propose { msg: proposal })?,
        funds: vec![],
    });

    Ok(Response::new()
        .add_attribute("action", "create_daodao_proposal")
        .add_message(msg))
}
