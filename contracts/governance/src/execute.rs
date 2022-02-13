use cosmwasm_std::{DepsMut, Env, MessageInfo, Response, Storage};
use governance_types::errors::ContractError;
use crate::state::{ VoteStatus, store_vote, update_config, may_load_vote, load_vote, update_vote };
use cosmwasm_std::Addr;
use crate::assert::{is_admin, is_owner, is_vote, already_participate};
use crate::stats::{add_new_vote};

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
    if is_vote(deps.storage, title.clone()) == Ok(true) {
        return Err(ContractError::VoteAlreadyExist {});
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
        already_participate: Vec::new(),
        whitelist_on,
        whitelist,
    };
    update_config(deps.storage, |mut config| -> Result<_, ContractError> {
        config.votes_titles.push(title.clone());
        Ok(config)
    })?;
    store_vote(deps.storage, &title, voter)?;
    add_new_vote(deps.storage)?;

    Ok(Response::new().add_attribute("action", "Added"))
}
pub fn execute_vote (
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    user_vote: String,
    title: String
) -> Result<Response, ContractError> {
    let vote = may_load_vote(deps.storage, &title)?;
    if None == vote{
        return Err(ContractError::CannotFindVote {});
    }
    let vote = &vote.unwrap();
    if Ok(true) != already_participate(vote.clone(), info.sender.clone()) {
        return Err(ContractError::VoterAlreadyParticipate {});
    }
    
    if vote.paused == true {
        return Err(ContractError::VoteIsPaused {});
    }
    return match user_vote.as_str() {
        "For" => vote_for(deps, info.sender, title),
        "Against" => vote_against(deps, info.sender, title),
        "Abstain" => vote_abstain(deps, info.sender, title),
        _ => return Err(ContractError::VoteNotValid {}),
    };
}

fn vote_for(deps: DepsMut, sender: Addr, title: String) -> Result<Response, ContractError> {
    update_vote(deps.storage, &title, |mut vote_status | -> Result<_, ContractError> {
        let mut vote_status = vote_status.unwrap();
        vote_status.votes_for+=1;
        vote_status.already_participate.push(sender);
        Ok(vote_status)
    })?;
    Ok(Response::new().add_attribute("action", "execute vote for"))
}
fn vote_against(deps: DepsMut, sender: Addr, title: String) -> Result<Response, ContractError> {
    update_vote(deps.storage, &title, |mut vote_status | -> Result<_, ContractError> {
        let mut vote_status = vote_status.unwrap();
        vote_status.votes_against+=1;
        vote_status.already_participate.push(sender);
        Ok(vote_status)
    })?;
    Ok(Response::new().add_attribute("action", "execute vote against"))
}
fn vote_abstain(deps: DepsMut, sender: Addr, title: String) -> Result<Response, ContractError> {
    update_vote(deps.storage, &title, |mut vote_status | -> Result<_, ContractError> {
        let mut vote_status = vote_status.unwrap();
        vote_status.votes_abstain+=1;
        vote_status.already_participate.push(sender);
        Ok(vote_status)
    })?;
    Ok(Response::new().add_attribute("action", "execute vote abstain"))
}