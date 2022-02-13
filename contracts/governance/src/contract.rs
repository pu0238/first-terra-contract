#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use governance_types::errors::ContractError;
use governance_types::types::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::queries::{query_get_vote, query_config, query_get_votes_titles, query_get_stats};
use crate::state::{Config, Stats, store_config, store_stats};
use crate::execute::{ execute_new_vote, execute_vote };

// Method is executed when a new contract instance is created. You can treat it as a constructor.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let config = Config {
        owner: info.sender.clone(),
        admins: msg.admins,
        votes_titles: vec![]
    };
    store_config(deps.storage, &config)?;
    let stats = Stats {
        rejected_votes: 0,
        accepted: 0,
        not_resolved: 0,
        paused: 0,
    };
    store_stats(deps.storage, &stats)?;
    
    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender)
    )
}

// Methods which are executed when someone send call which changes blockchain state.
// It can be compared to Solidity NON view methods.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    
    match msg {
        ExecuteMsg::CreateNewVote { 
            title,
            required_balance,
            min_votes_count,
            required_votes_percentage,
            whitelist_on,
            whitelist
        } => execute_new_vote(
                deps, 
                _env, 
                info,  
                title,
                required_balance,
                min_votes_count,
                required_votes_percentage,
                whitelist_on,
                whitelist
            ),
        ExecuteMsg::Vote { vote, title } => execute_vote(
            deps, 
            _env, 
            info,  
            vote,
            title
        ),
    }
}

// Methods which are executed when someone send a query (gas free call).
// It can be compared to Solidity view methods.
#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(
    deps: Deps, 
    _env: Env, 
    msg: QueryMsg
) -> Result<Binary, ContractError> {
    match msg {
        // TODO implement missing even handlers
        QueryMsg::Config {} => {
            Ok(to_binary(&query_config(deps)?)?)
            // return config
        }
        QueryMsg::GetVotesTitles {} => {
            Ok(to_binary(&query_get_votes_titles(deps)?)?)
            // return all votes by title
        }
        QueryMsg::GetVote { title } => {
            Ok(to_binary(&query_get_vote(deps, title)?)?)
            // return specific vote
        }
        QueryMsg::GetVoter { .. } => {
            Ok(to_binary(&{})?)
        }
        QueryMsg::GetStats {} => {
            Ok(to_binary(&query_get_stats(deps)?)?)
            // return stats
        }
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn migrate(
    _deps: DepsMut,
    _env: Env,
    _msg: MigrateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}
