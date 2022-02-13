use cosmwasm_std::Deps;
use governance_types::errors::ContractError;
use crate::state::{Config, read_config};

pub fn query_config(
    deps: Deps,
) -> Result<Config, ContractError> {
    let config = read_config(deps.storage)?;
    Ok(config)
}
