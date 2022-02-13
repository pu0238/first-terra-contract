use cosmwasm_std::{Storage};
use governance_types::errors::ContractError;
use crate::state::{ Stats, update_stats };

pub fn add_new_vote (storage: &mut dyn Storage) -> Result<Stats, ContractError> {
    update_stats(storage, |mut stats| -> Result<_, ContractError> {
        stats.not_resolved=1;
        Ok(stats)
    })
}
pub fn set_to_rejected (storage: &mut dyn Storage) -> Result<Stats, ContractError> {
    update_stats(storage, |mut stats| -> Result<_, ContractError> {
        stats.not_resolved-=1;
        stats.rejected_votes+=1;
        Ok(stats)
    })
}
pub fn set_to_accepted (storage: &mut dyn Storage) -> Result<Stats, ContractError> {
    update_stats(storage, |mut stats| -> Result<_, ContractError> {
        stats.not_resolved-=1;
        stats.accepted+=1;
        Ok(stats)
    })
}
pub fn set_to_paused (storage: &mut dyn Storage) -> Result<Stats, ContractError> {
    update_stats(storage, |mut stats| -> Result<_, ContractError> {
        stats.not_resolved-=1;
        stats.paused+=1;
        Ok(stats)
    })
}
pub fn set_to_un_paused (storage: &mut dyn Storage) -> Result<Stats, ContractError> {
    update_stats(storage, |mut stats| -> Result<_, ContractError> {
        stats.paused-=1;
        stats.not_resolved+=1;
        Ok(stats)
    })
}