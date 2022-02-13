use cosmwasm_std::Deps;
use governance_types::errors::ContractError;
use crate::state::{Config, VoteStatus, Stats, read_config, may_load_vote, read_stats};

pub fn query_config(
    deps: Deps,
) -> Result<Config, ContractError> {
    let config = read_config(deps.storage)?;
    Ok(config)
}

pub fn query_get_vote(
    deps: Deps,
    title: String
) -> Result<Option<VoteStatus>, ContractError> {
    let vote_status = may_load_vote(deps.storage, &title)?;
    Ok(vote_status)
}

pub fn query_get_votes_titles(
    deps: Deps,
) -> Result<Vec<String>, ContractError> {
    let config = read_config(deps.storage)?;
    Ok(config.votes_titles)
}

pub fn query_get_stats(
    deps: Deps,
) -> Result<Stats, ContractError> {
    let stats = read_stats(deps.storage)?;
    Ok(stats)
}