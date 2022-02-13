use cosmwasm_std::{DepsMut, Env, MessageInfo, Response};
use governance_types::errors::ContractError;

pub fn execute_vote(
    _deps: DepsMut,
    _env: Env,
    info: MessageInfo,
) -> Result<Response, ContractError> {
    // TODO implement voting, and save state

    Ok(Response::new()
        .add_attribute("action", "execute vote")
        .add_attribute("voter", info.sender.as_str())
    )
}