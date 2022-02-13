#[cfg(test)]
mod test {

    use cosmwasm_std::testing::{mock_dependencies_with_balances, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    use governance_types::types::{InstantiateMsg, QueryMsg, ExecuteMsg};
    use crate::state::{Config};
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
            required_balance: 1,
            min_votes_count: 1,
            required_votes_percentage: 1,
            whitelist_on: false,
            whitelist: vec![],
        };
        let res = execute(deps.as_mut(), mock_env(), info, new_vote).unwrap();

    }
}