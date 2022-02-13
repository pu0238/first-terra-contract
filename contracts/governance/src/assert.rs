use cosmwasm_std::{Storage};
use governance_types::errors::ContractError;
use crate::state::{read_config};
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
pub fn is_owner(storage: &dyn Storage, user: Addr) -> Result<bool, ContractError>{
    let config = read_config(storage)?;
    return Ok(config.owner == user)
}