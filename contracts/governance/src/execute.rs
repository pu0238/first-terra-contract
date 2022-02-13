use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use governance_types::errors::ContractError;
use crate::state::{ VoteStatus, store_vote };
use cosmwasm_std::Addr;
use crate::assert::{is_admin, is_owner};

pub fn execute_new_vote(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    title: String,
    required_balance: i32,
    min_votes_count: i32,
    required_votes_percentage: i32,
    whitelist_on: bool,
    whitelist: Vec<Addr>,
) -> Result<Response, ContractError> {
    if required_balance < 0{
        return Err(ContractError::BalanceCannotBeNegative {});
    }
    if min_votes_count < 0{
        return Err(ContractError::VoteCountCannotBeNegative {});
    }
    if required_votes_percentage < 0 || required_votes_percentage > 100 {
        return Err(ContractError::WrongVotesPercentage {});
    }
    if is_owner(deps.storage, info.sender.clone()) != Ok(true) {
        if is_admin(deps.storage, info.sender.clone()) != Ok(true) {
            return Err(ContractError::SenderIsNotAdmin {});
        }
    }
    let voter = VoteStatus {
        creator: info.sender.clone(),
        paused: false,
        votes_for: 0,
        votes_against: 0,
        votes_abstain: 0,
        required_balance,
        min_votes_count,
        required_votes_percentage,
        whitelist_on,
        whitelist,
    };
    store_vote(deps.storage, title.as_str() , voter)?;
    Ok(Response::new().add_attribute("action", "Added"))
}