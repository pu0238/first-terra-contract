use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use governance_types::errors::ContractError;
use crate::state::{ VoteStatus, store_vote, update_config, may_load_vote, update_vote };
use cosmwasm_std::{Addr, Coin};
use crate::assert::{is_admin, is_owner, is_vote, already_participate, is_whitelisted};
use crate::stats::{add_in_progress};

pub fn execute_new_vote(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    title: String,
    min_votes_count: i32,
    required_votes_percentage: i32,
    whitelist_on: bool,
    whitelist: Vec<Addr>,
    required_coins_on: bool,
    required_coins: Coin,
) -> Result<Response, ContractError> {
//  TODO: ADD COINS
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
        required_coins_on,
        required_coins,
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
    add_in_progress(deps.storage)?;

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
    let vote = vote.unwrap();
    if Ok(true) != already_participate(vote.clone(), info.sender.clone()) {
        return Err(ContractError::VoterAlreadyParticipate {});
    }
    if vote.paused == true {
        return Err(ContractError::VoteIsPaused {});
    }
    if vote.whitelist_on {
        if Ok(true) != is_whitelisted(vote, info.sender.clone()){
            return Err(ContractError::SenderIsNotWhitelisted {});
        }
    }
    let a = info.funds;
    /*
        TODO: VOTE BASE ON COIN
    if vote.required_balance == info.funds {
        return Err(ContractError::VoteIsPaused {});
    }*/
    return match user_vote.as_str() {
        "For" => vote_for(deps, info.sender, title),
        "Against" => vote_against(deps, info.sender, title),
        "Abstain" => vote_abstain(deps, info.sender, title),
        _ => return Err(ContractError::VoteNotValid {}),
    };
}

fn vote_for(deps: DepsMut, sender: Addr, title: String) -> Result<Response, ContractError> {
    update_vote(deps.storage, &title, |mut _vote_status | -> Result<_, ContractError> {
        let mut vote_status = _vote_status.unwrap();
        vote_status.votes_for+=1;
        vote_status.already_participate.push(sender);
        Ok(vote_status)
    })?;
    Ok(Response::new().add_attribute("action", "execute vote for"))
}
fn vote_against(deps: DepsMut, sender: Addr, title: String) -> Result<Response, ContractError> {
    update_vote(deps.storage, &title, |mut _vote_status | -> Result<_, ContractError> {
        let mut vote_status = _vote_status.unwrap();
        vote_status.votes_against+=1;
        vote_status.already_participate.push(sender);
        Ok(vote_status)
    })?;
    Ok(Response::new().add_attribute("action", "execute vote against"))
}
fn vote_abstain(deps: DepsMut, sender: Addr, title: String) -> Result<Response, ContractError> {
    update_vote(deps.storage, &title, |mut _vote_status | -> Result<_, ContractError> {
        let mut vote_status = _vote_status.unwrap();
        vote_status.votes_abstain+=1;
        vote_status.already_participate.push(sender);
        Ok(vote_status)
    })?;
    Ok(Response::new().add_attribute("action", "execute vote abstain"))
}//execute_pause
pub fn execute_pause(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    title: String
) -> Result<Response, ContractError> {
    if is_owner(deps.storage, info.sender.clone()) != Ok(true) {
        if is_admin(deps.storage, info.sender.clone()) != Ok(true) {
            return Err(ContractError::SenderIsNotAdmin {});
        }
    }
    let vote = may_load_vote(deps.storage, &title)?;
    if None == vote{
        return Err(ContractError::CannotFindVote {});
    }
    let vote = &vote.unwrap();
    if vote.paused {
        return Ok(Response::new().add_attribute("action", "voting is already paused"))
    }
    update_vote(deps.storage, &title, |mut _vote_status | -> Result<_, ContractError> {
        let mut vote_status = _vote_status.unwrap();
        vote_status.paused=true;
        Ok(vote_status)
    })?;
    return Err(ContractError::VoterAlreadyParticipate {});
}
pub fn execute_unpause(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    title: String
) -> Result<Response, ContractError> {
    if is_owner(deps.storage, info.sender.clone()) != Ok(true) {
        if is_admin(deps.storage, info.sender.clone()) != Ok(true) {
            return Err(ContractError::SenderIsNotAdmin {});
        }
    }
    let vote = may_load_vote(deps.storage, &title)?;
    if None == vote{
        return Err(ContractError::CannotFindVote {});
    }
    let vote = &vote.unwrap();
    if !vote.paused {
        return Ok(Response::new().add_attribute("action", "voting is already unpaused"))
    }
    update_vote(deps.storage, &title, |mut _vote_status | -> Result<_, ContractError> {
        let mut vote_status = _vote_status.unwrap();
        vote_status.paused=false;
        Ok(vote_status)
    })?;
    return Err(ContractError::VoterAlreadyParticipate {});
}
pub fn execute_toogle_whitelist(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    title: String
) -> Result<Response, ContractError> {
    if is_owner(deps.storage, info.sender.clone()) != Ok(true) {
        if is_admin(deps.storage, info.sender.clone()) != Ok(true) {
            return Err(ContractError::SenderIsNotAdmin {});
        }
    }
    let vote = may_load_vote(deps.storage, &title)?;
    if None == vote{
        return Err(ContractError::CannotFindVote {});
    }
    let vote = &vote.unwrap();
    update_vote(deps.storage, &title, |mut _vote_status | -> Result<_, ContractError> {
        let mut vote_status = _vote_status.unwrap();
        vote_status.whitelist_on != vote_status.whitelist_on;
        Ok(vote_status)
    })?;
    return Err(ContractError::VoterAlreadyParticipate {});
}
pub fn execute_toogle_required_coin(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    title: String
) -> Result<Response, ContractError> {
    if is_owner(deps.storage, info.sender.clone()) != Ok(true) {
        if is_admin(deps.storage, info.sender.clone()) != Ok(true) {
            return Err(ContractError::SenderIsNotAdmin {});
        }
    }
    let vote = may_load_vote(deps.storage, &title)?;
    if None == vote{
        return Err(ContractError::CannotFindVote {});
    }
    let vote = &vote.unwrap();
    update_vote(deps.storage, &title, |mut _vote_status | -> Result<_, ContractError> {
        let mut vote_status = _vote_status.unwrap();
        vote_status.required_coins_on != vote_status.required_coins_on;
        Ok(vote_status)
    })?;
    return Err(ContractError::VoterAlreadyParticipate {});
}