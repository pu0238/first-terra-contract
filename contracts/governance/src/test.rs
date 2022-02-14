#[cfg(test)]
mod test {

    use cosmwasm_std::testing::{mock_dependencies_with_balances, mock_env, mock_info};
    use cosmwasm_std::{coin, coins, from_binary};
    use governance_types::errors::ContractError;
    use governance_types::types::{InstantiateMsg, QueryMsg, ExecuteMsg};
    use crate::state::{Config, VoteStatus, Stats};
    use crate::contract::{execute, instantiate, query};

    #[test]
    fn proper_initialization() {
        let mut deps = mock_dependencies_with_balances(&[("sdsd", &coins(12, "token"))]);
        let info = mock_info("creator", &coins(1000, "earth"));
        let msg = InstantiateMsg {
            admins: vec![info.sender]
        };
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
        let value: Config = from_binary(&res).unwrap();
        let info = mock_info("creator", &coins(1000, "earth"));
        assert_eq!(value.admins[0], info.sender);
    }
    #[test]
    fn create_new_vote() {
        let mut deps = mock_dependencies_with_balances(&[("sdsd", &coins(12, "token"))]);
        let msg = InstantiateMsg {
            admins: vec![]
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let _res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();

        let info = mock_info("creator", &coins(1000, "earth"));
        let new_vote = ExecuteMsg::CreateNewVote {
            title: "some title".to_string(),
            min_votes_count: 1,
            required_votes_percentage: 1,
            whitelist_on: false,
            whitelist: vec![],
            required_coins_on: false,
            required_coin: coin(1, "test"),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, new_vote).unwrap();

        let get_vote = QueryMsg::GetVote {
            title: "sdsd".to_string()
        };
        let res = query(deps.as_ref(), mock_env(), get_vote).unwrap();
        let value: Option<VoteStatus> = from_binary(&res).unwrap();
        assert_eq!(value, None);

        let get_vote = QueryMsg::GetVote {
            title: "some title".to_string()
        };
        let res = query(deps.as_ref(), mock_env(), get_vote).unwrap();
        let value: VoteStatus = from_binary(&res).unwrap();
        assert_eq!(value.whitelist_on, false);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::Config {}).unwrap();
        let value: Config = from_binary(&res).unwrap();
        assert_eq!(value.votes_titles, vec!["some title".to_string()]);

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetStats {}).unwrap();
        let value: Stats = from_binary(&res).unwrap();
        assert_eq!(value.in_progress, 1);
    }
    #[test]
    fn proper_vote() {
        let mut deps = mock_dependencies_with_balances(&[("sdsd", &coins(12, "token"))]);
        let msg = InstantiateMsg {
            admins: vec![]
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        let new_vote = ExecuteMsg::CreateNewVote {
            title: "some title".to_string(),
            min_votes_count: 1,
            required_votes_percentage: 1,
            whitelist_on: false,
            whitelist: vec![],
            required_coins_on: false,
            required_coin: coin(1, "test"),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, new_vote).unwrap();

        let info = mock_info("creator", &coins(1000, "earth"));
        let vote = ExecuteMsg::Vote {
            vote: "For".to_string(),
            title: "some title".to_string()
        };
        let _res = execute(deps.as_mut(), mock_env(), info, vote).unwrap();
        let get_vote = QueryMsg::GetVote {
            title: "some title".to_string()
        };
        let res = query(deps.as_ref(), mock_env(), get_vote).unwrap();
        let value: VoteStatus = from_binary(&res).unwrap();
        assert_eq!(value.votes_for, 1);

        let _res = query(deps.as_ref(), mock_env(), QueryMsg::GetStats {}).unwrap();
        let value: Stats = from_binary(&_res).unwrap();
        assert_eq!(value.in_progress, 1);
    }
    #[test]
    fn proper_pause_and_unpause() {
        let mut deps = mock_dependencies_with_balances(&[("sdsd", &coins(12, "token"))]);
        let msg = InstantiateMsg {
            admins: vec![]
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        let new_vote = ExecuteMsg::CreateNewVote {
            title: "some title".to_string(),
            min_votes_count: 1,
            required_votes_percentage: 1,
            whitelist_on: false,
            whitelist: vec![],
            required_coins_on: false,
            required_coin: coin(1, "test"),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, new_vote).unwrap();

        let info = mock_info("creator", &coins(1000, "earth"));
        let vote = ExecuteMsg::Pause {
            title: "some title".to_string()
        };
        let _res = execute(deps.as_mut(), mock_env(), info, vote).unwrap();
        //===========
        let get_vote = QueryMsg::GetVote {
            title: "some title".to_string()
        };
        let res = query(deps.as_ref(), mock_env(), get_vote).unwrap();
        let value: VoteStatus = from_binary(&res).unwrap();
        assert_eq!(value.paused, true);

        let _res = query(deps.as_ref(), mock_env(), QueryMsg::GetStats {}).unwrap();
        let value: Stats = from_binary(&_res).unwrap();
        assert_eq!(value.in_progress, 0);
        assert_eq!(value.paused, 1);
    }
    #[test]
    fn privent_vote_on_pause() {
        let mut deps = mock_dependencies_with_balances(&[("sdsd", &coins(12, "token"))]);
        let msg = InstantiateMsg {
            admins: vec![]
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        let new_vote = ExecuteMsg::CreateNewVote {
            title: "some title".to_string(),
            min_votes_count: 1,
            required_votes_percentage: 1,
            whitelist_on: false,
            whitelist: vec![],
            required_coins_on: false,
            required_coin: coin(1, "test"),
        };
        let _res = execute(deps.as_mut(), mock_env(), info, new_vote).unwrap();

        let info = mock_info("creator", &coins(1000, "earth"));
        let vote = ExecuteMsg::Pause {
            title: "some title".to_string()
        };
        let _res = execute(deps.as_mut(), mock_env(), info, vote).unwrap();
        let info = mock_info("creator", &coins(1000, "earth"));
        let vote = ExecuteMsg::Vote {
            vote: "For".to_string(),
            title: "some title".to_string()
        };
        let res = execute(deps.as_mut(), mock_env(), info, vote);
        match res {
            Err(ContractError::VoteIsPaused {}) => {}
            _ => panic!("Must return vote is paused error"),
        }
    }
    #[test]
    fn proper_whitelist() {
        let mut deps = mock_dependencies_with_balances(&[("sdsd", &coins(12, "token"))]);
        let msg = InstantiateMsg {
            admins: vec![]
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let whitelisted = mock_info("whitelisted", &coins(1000, "earth"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        let new_vote = ExecuteMsg::CreateNewVote {
            title: "some title".to_string(),
            min_votes_count: 1,
            required_votes_percentage: 1,
            whitelist_on: true,
            whitelist: vec![whitelisted.sender],
            required_coins_on: false,
            required_coin: coin(1, "test"),
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), new_vote).unwrap();

        let vote = ExecuteMsg::Vote {
            vote: "For".to_string(),
            title: "some title".to_string()
        };
        let _res = execute(deps.as_mut(), mock_env(), info, vote).unwrap();

        let info = mock_info("as", &coins(1000, "earth"));
        let vote = ExecuteMsg::Vote {
            vote: "For".to_string(),
            title: "some title".to_string()
        };
        let res = execute(deps.as_mut(), mock_env(), info, vote);

        match res {
            Err(ContractError::SenderIsNotWhitelisted {}) => {}
            _ => panic!("Must return sender is not whitelisted error"),
        }  
    }
    #[test]
    fn proper_get_vote_titles() {
        let mut deps = mock_dependencies_with_balances(&[("sdsd", &coins(12, "token"))]);
        let msg = InstantiateMsg {
            admins: vec![]
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let whitelisted = mock_info("whitelisted", &coins(1000, "earth"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        let new_vote = ExecuteMsg::CreateNewVote {
            title: "some title".to_string(),
            min_votes_count: 1,
            required_votes_percentage: 1,
            whitelist_on: true,
            whitelist: vec![whitelisted.sender],
            required_coins_on: false,
            required_coin: coin(1, "test"),
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), new_vote).unwrap();

        let res = query(deps.as_ref(), mock_env(), QueryMsg::GetVotesTitles {}).unwrap();
        let value: Vec<String> = from_binary(&res).unwrap();
        assert_eq!(value, vec!["some title".to_string()]);
    }
    #[test]
   
    fn proper_require_coins() {
        let mut deps = mock_dependencies_with_balances(&[("sdsd", &coins(12, "token"))]);
        let msg = InstantiateMsg {
            admins: vec![]
        };
        let info = mock_info("creator", &coins(1000, "earth"));
        let _res = instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();
        let new_vote = ExecuteMsg::CreateNewVote {
            title: "some title".to_string(),
            min_votes_count: 1,
            required_votes_percentage: 1,
            whitelist_on: false,
            whitelist: vec![],
            required_coins_on: true,
            required_coin: coin(1, "test"),
        };
        let _res = execute(deps.as_mut(), mock_env(), info.clone(), new_vote).unwrap();

        let vote = ExecuteMsg::Vote {
            vote: "For".to_string(),
            title: "some title".to_string()
        };
        let info = mock_info("user1", &coins(1000, "earth"));
        let res = execute(deps.as_mut(), mock_env(), info, vote);
        match res {
            Err(ContractError::SenderDoNotHaveEnoughAmount {}) => {}
            _ => panic!("Must return sender is not whitelisted error"),
        }
        let vote = ExecuteMsg::Vote {
            vote: "For".to_string(),
            title: "some title".to_string()
        };
        let info = mock_info("user1", &coins(0, "test"));
        let res = execute(deps.as_mut(), mock_env(), info, vote);
        match res {
            Err(ContractError::SenderDoNotHaveEnoughAmount {}) => {}
            _ => panic!("Must return sender is not whitelisted error"),
        }
        let vote = ExecuteMsg::Vote {
            vote: "For".to_string(),
            title: "some title".to_string()
        };
        let info = mock_info("user1", &coins(1, "test"));
        let res = execute(deps.as_mut(), mock_env(), info, vote).unwrap();
    }
}