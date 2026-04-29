use crate::{
    ConvertError,
    user::{EmailAddress, UserId},
};

#[derive(thiserror::Error, Debug)]
pub enum AddNewUserError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to query schema revisions: {0}")]
    Database(sqlx::Error),
    #[error("Entry with such id: {id} or email: {email} already exists")]
    AlreadyExists { id: UserId, email: EmailAddress },
    #[error("It must affect only one row, affects {0}")]
    InvalidAffectedRowsAmount(u64),
}

#[derive(thiserror::Error, Debug)]
pub enum UserFromRowError {
    #[error("Cannot decode users id column: {0}")]
    CannotDecodeId(sqlx::Error),
    #[error("Cannot decode users email column: {0}")]
    CannotDecodeEmail(sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum GetUserIdError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to query user by email: {0}")]
    Database(sqlx::Error),
    #[error("User with email: {0} not found")]
    NotFound(EmailAddress),
    #[error("Cannot decode users id column: {0}")]
    CannotDecodeId(sqlx::Error),
}

#[derive(thiserror::Error, Debug)]
pub enum GetUserError {
    #[error(transparent)]
    CantConvert(#[from] ConvertError),
    #[error("Failed to query schema revisions: {0}")]
    Database(sqlx::Error),
    #[error("User entry not found by the provided id {0}")]
    NotFound(UserId),
}
