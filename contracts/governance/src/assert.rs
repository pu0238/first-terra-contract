use cosmwasm_std::{Storage};
use governance_types::errors::ContractError;
use crate::state::{VoteStatus, read_config};
use cosmwasm_std::Addr;

pub fn is_admin(storage: &dyn Storage, user: Addr) -> Result<bool, ContractError>{
    let config = read_config(storage)?;
    for admin in config.admins.clone() {
        if admin == user{
            return Ok(true);
        }
    }
    return Ok(false);
}
pub fn is_vote(storage: &dyn Storage, title: String) -> Result<bool, ContractError>{
    let title = title.to_string();
    let config = read_config(storage)?;
    for some_title in config.votes_titles.clone() {
        if some_title == title{
            return Ok(true);
        }
    }
    return Ok(false);
}
pub fn is_owner(storage: &dyn Storage, user: Addr) -> Result<bool, ContractError>{
    let config = read_config(storage)?;
    return Ok(config.owner == user)
}
pub fn already_participate(vote: VoteStatus, voter: Addr) -> Result<bool, ContractError>{
    for participate in vote.already_participate.clone() {
        if participate == voter {
            return Ok(true);
        }
    }
    return Ok(false);
}
pub fn is_whitelisted(vote: VoteStatus, voter: Addr) -> Result<bool, ContractError>{
    for whitelisted_voter in vote.whitelist.clone() {
        if whitelisted_voter == voter {
            return Ok(true);
        }
    }
    return Ok(false);
}