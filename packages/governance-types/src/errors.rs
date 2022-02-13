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

    #[error("VoteIsClosed")]
    VoteIsClosed {},

    #[error("VoterAlreadyParticipate")]
    VoterAlreadyParticipate {},

    #[error("AdminsNoGiven")]
    AdminsNoGiven {},

    #[error("BalanceCannotBeNegative")]
    BalanceCannotBeNegative {},

    #[error("VoteCountCannotBeNegative")]
    VoteCountCannotBeNegative {},

    #[error("SenderIsNotAdmin")]
    SenderIsNotAdmin {},
}
