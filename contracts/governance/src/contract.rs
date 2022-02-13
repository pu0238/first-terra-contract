#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_binary};
use governance_types::errors::ContractError;
use governance_types::types::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg};
use crate::queries::{query_get_vote, query_config, query_get_votes_titles, query_get_stats};
use crate::state::{Config, Stats, store_config, store_stats};
use crate::execute::{ execute_new_vote, execute_vote, execute_pause, execute_unpause, execute_toogle_whitelist, execute_toogle_required_coin};

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
        in_progress: 0
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
            min_votes_count,
            required_votes_percentage,
            whitelist_on,
            whitelist,
            required_coins_on,
            required_coin,
        } => execute_new_vote(
                deps, 
                _env, 
                info,  
                title,
                min_votes_count,
                required_votes_percentage,
                whitelist_on,
                whitelist,
                required_coins_on,
                required_coin,
            ),
        ExecuteMsg::Vote { vote, title } => execute_vote(
            deps, 
            _env, 
            info,  
            vote,
            title
        ),
        ExecuteMsg::Pause { title } => execute_pause(
            deps,
            _env,
            info,
            title
        ),
        ExecuteMsg::Unpause { title } => execute_unpause(
            deps,
            _env,
            info,
            title
        ),
        ExecuteMsg::ToogleWhitelist { title } => execute_toogle_whitelist(
            deps,
            _env,
            info,
            title
        ),
        ExecuteMsg::ToogleRequiredCoin { title } => execute_toogle_required_coin(
            deps,
            _env,
            info,
            title
        )
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
