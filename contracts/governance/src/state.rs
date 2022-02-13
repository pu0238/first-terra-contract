use cosmwasm_std::{StdResult, Storage, StdError};
use cw_storage_plus::{Item, Map};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::Addr;

// cs-storage-plus docs: https://crates.io/crates/cw-storage-plus

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub owner: Addr,
    pub admins: Vec<Addr>,
    pub votes_titles: Vec<String>,
}
const CONFIG: Item<Config> = Item::new("\u{0}\u{6}config");
pub fn store_config(storage: &mut dyn Storage, config: &Config) -> StdResult<()> {
    CONFIG.save(storage, config)
}
pub fn read_config(storage: &dyn Storage) -> StdResult<Config> {
    CONFIG.load(storage)
}
pub fn update_config <A, E> (storage: &mut dyn Storage, action: A) -> Result<Config, E> where
A: FnOnce(Config) -> Result<Config, E>,
E: From<StdError>,
{
    CONFIG.update(storage, action)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Stats {
    pub rejected_votes: i32,
    pub accepted: i32,
    pub not_resolved: i32,
    pub paused: i32,
}
const STATS: Item<Stats> = Item::new("\u{0}\u{6}stats");
pub fn store_stats(storage: &mut dyn Storage, config: &Stats) -> StdResult<()> {
    STATS.save(storage, config)
}
pub fn read_stats(storage: &dyn Storage) -> StdResult<Stats> {
    STATS.load(storage)
}
pub fn update_stats <A, E> (storage: &mut dyn Storage, action: A) -> Result<Stats, E> where
A: FnOnce(Stats) -> Result<Stats, E>,
E: From<StdError>,
{
    STATS.update(storage, action)
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct VoteStatus {
    pub paused: bool,
    pub votes_for: i32,
    pub votes_against: i32,
    pub votes_abstain: i32,

    pub creator: Addr,
    pub required_balance: i32,
    pub min_votes_count: i32,
    pub required_votes_percentage: i32,
    pub already_participate: Vec<Addr>,
    pub whitelist_on: bool,
    pub whitelist: Vec<Addr>,
}

const VOTES: Map<&str, VoteStatus> = Map::new("VOTES");

pub fn may_load_vote (storage: &dyn Storage, key: &String) -> StdResult<Option<VoteStatus>> {
    VOTES.may_load(storage, key)
}
pub fn load_vote (storage: &dyn Storage, key: &String) -> StdResult<VoteStatus> {
    VOTES.load(storage, key)
}
pub fn store_vote (storage: &mut dyn Storage, key: &String, data: VoteStatus) -> StdResult<()> {
    VOTES.save(storage, key, &data)
}
pub fn update_vote <A, E> (storage: &mut dyn Storage, key: &String, action: A ) -> Result<VoteStatus, E> where
A: FnOnce(Option<VoteStatus>) -> Result<VoteStatus, E>,
E: From<StdError>, {
    VOTES.update(storage, key, action)
}