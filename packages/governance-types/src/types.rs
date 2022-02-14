use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use cosmwasm_std::{Addr, Coin};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {
    pub admins: Vec<Addr>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    CreateNewVote { 
        title: String,
        min_votes_count: i32,
        required_votes_percentage: i32,
        whitelist_on: bool,
        whitelist: Vec<Addr>,
        required_coins_on: bool,
        required_coin: Coin,
    },
    Vote { 
        vote: String, 
        title: String 
    },
    Pause { title: String },
    Unpause { title: String },
    ToogleWhitelist { title: String },
    ToogleRequiredCoin { title: String }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Config {},
    GetStats {},
    GetVotesTitles {},
    GetVote { title: String }
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct ConfigResponse {
    pub owner: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct MigrateMsg {}