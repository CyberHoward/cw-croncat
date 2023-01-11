#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, CosmosMsg, StdError, WasmMsg};
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
#[cfg(not(feature = "library"))]
use cw2::set_contract_version;
use dao_voting::multiple_choice::CheckedMultipleChoiceOption;
use mod_sdk::types::QueryResponse;

use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::types::dao::{ProposalListResponse, ProposalResponse, QueryDao, Status};
use crate::types::CheckProposalStatus;
use crate::ContractError;

// version info for migration info
const CONTRACT_NAME: &str = "croncat:mod-dao";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, StdError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    Ok(Response::new().add_attribute("method", "instantiate"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    Err(ContractError::Noop)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::CheckProposalStatus(CheckProposalStatus {
            dao_address,
            proposal_id,
            status,
        }) => to_binary(&query_dao_proposal_status(
            deps,
            dao_address,
            proposal_id,
            status,
        )?),
        QueryMsg::CheckPassedProposals { dao_address } => {
            to_binary(&query_dao_proposals(deps, dao_address)?)
        }
        QueryMsg::CheckWithMigration { dao_address } => {
            to_binary(&query_proposals_with_migration(deps, dao_address)?)
        }
    }
}

/// Query: CheckProposalStatus
/// Used as a helper method to check the proposals status
///
/// Response: QueryResponse
/// Returns true if the proposal status matches with the given `status`
/// Data is the information about the proposal
fn query_dao_proposal_status(
    deps: Deps,
    dao_address: String,
    proposal_id: u64,
    status: Status,
) -> StdResult<QueryResponse> {
    let dao_addr = deps.api.addr_validate(&dao_address)?;
    let resp: ProposalResponse = deps
        .querier
        .query_wasm_smart(dao_addr, &QueryDao::Proposal { proposal_id })?;
    Ok(QueryResponse {
        result: resp.proposal.status == status,
        data: to_binary(&resp)?,
    })
}

/// Query: CheckPassedProposals
/// Used as a helper method to check if there're any passed proposals
///
/// Response: QueryResponse
/// Returns true if there's at least one passed proposal
/// Data contains a vector of passed proposals
fn query_dao_proposals(deps: Deps, dao_address: String) -> StdResult<QueryResponse> {
    let dao_addr = deps.api.addr_validate(&dao_address)?;
    // Query the amount of proposals
    let proposal_count = deps
        .querier
        .query_wasm_smart(dao_addr.clone(), &QueryDao::ProposalCount {})?;
    let res: ProposalListResponse = deps.querier.query_wasm_smart(
        dao_addr,
        &QueryDao::ListProposals {
            start_after: None,
            limit: Some(proposal_count),
        },
    )?;

    for proposal_response in &res.proposals {
        if proposal_response.proposal.status == Status::Passed {
            return Ok(QueryResponse {
                result: true,
                data: to_binary(&proposal_response.id)?,
            });
        }
    }
    Ok(QueryResponse {
        result: false,
        data: to_binary(&res.proposals)?,
    })
}

/// Query: CheckWithMigration
/// Used as a helper method to check if there're any passed proposals with Migration message
///
/// Response: QueryResponse
/// Returns true if there's at least one passed proposal with Migration message
/// Data contains a vector of ids of passed proposals with Migration message
fn query_proposals_with_migration(deps: Deps, dao_address: String) -> StdResult<QueryResponse> {
    let dao_addr = deps.api.addr_validate(&dao_address)?;
    let mut with_migration = vec![];

    // Query the amount of proposals
    let proposal_count = deps
        .querier
        .query_wasm_smart(dao_addr.clone(), &QueryDao::ProposalCount {})?;

    // Try to list proposals for the case of single choice proposal
    let res_opt: StdResult<dao_proposal_single::query::ProposalListResponse> =
        deps.querier.query_wasm_smart(
            dao_addr.clone(),
            &QueryDao::ListProposals {
                start_after: None,
                limit: Some(proposal_count),
            },
        );

    if let Ok(res) = res_opt {
        for proposal_response in &res.proposals {
            if proposal_response.proposal.status == dao_voting::status::Status::Passed {
                for msg in &proposal_response.proposal.msgs {
                    if let CosmosMsg::Wasm(WasmMsg::Migrate { .. }) = &msg {
                        with_migration.push(proposal_response.id);
                        break;
                    }
                }
            }
        }
    } else {
        // If it's not single choice proposal contract, try to query from multiple choice proposal
        let res: dao_proposal_multiple::query::ProposalListResponse =
            deps.querier.query_wasm_smart(
                dao_addr,
                &QueryDao::ListProposals {
                    start_after: None,
                    limit: Some(proposal_count),
                },
            )?;
        for proposal_response in &res.proposals {
            let proposal = &proposal_response.proposal;
            if proposal.status == dao_voting::status::Status::Passed {
                let vote_result = proposal.calculate_vote_result()?;
                match vote_result {
                    dao_proposal_multiple::proposal::VoteResult::SingleWinner(
                        CheckedMultipleChoiceOption { msgs, .. },
                    ) => {
                        for msg in msgs {
                            if let CosmosMsg::Wasm(WasmMsg::Migrate { .. }) = &msg {
                                with_migration.push(proposal_response.id);
                                break;
                            }
                        }
                    }
                    // This shouldn't happen
                    dao_proposal_multiple::proposal::VoteResult::Tie => {
                        return Err(StdError::generic_err("Tie is impossible"))
                    }
                }
            }
        }
    }

    Ok(QueryResponse {
        result: !with_migration.is_empty(),
        data: to_binary(&with_migration)?,
    })
}
