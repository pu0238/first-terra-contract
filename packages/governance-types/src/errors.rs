use cosmwasm_std::StdError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("Unauthorized")]
    Unauthorized {},

    #[error("WrongVotesPercentage")]
    WrongVotesPercentage {},

    #[error("VoteNotValid")]
    VoteNotValid {},

    #[error("FailedToUpdateData")]
    FailedToUpdateData {},

    #[error("VoteIsPaused")]
    VoteIsPaused {},

    #[error("VoterAlreadyParticipate")]
    VoterAlreadyParticipate {},

    #[error("AdminsNoGiven")]
    AdminsNoGiven {},

    #[error("BalanceCannotBeNegative")]
    BalanceCannotBeNegative {},

    #[error("SenderIsNotAdmin")]
    SenderIsNotAdmin {},

    #[error("VoteAlreadyExist")]
    VoteAlreadyExist {},

    #[error("CannotFindVote")]
    CannotFindVote {},

    #[error("SenderIsNotWhitelisted")]
    SenderIsNotWhitelisted {},

    #[error("SenderDoNotHaveEnoughAmount")]
    SenderDoNotHaveEnoughAmount {},
}
