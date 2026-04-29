pub mod errors;

use sqlx::Row;

use crate::{
    ConvertTo, CoreDb,
    user::errors::{AddNewUserError, GetUserError, GetUserIdError, UserFromRowError},
};

pub type UserId = uuid::Uuid;
pub type EmailAddress = String;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct User {
    pub id: UserId,
    pub email: EmailAddress,
}

/// Trying to read row in the format `(users.id, users.email)`
impl TryFrom<sqlx::sqlite::SqliteRow> for User {
    type Error = UserFromRowError;

    fn try_from(row: sqlx::sqlite::SqliteRow) -> Result<Self, Self::Error> {
        let id = row.try_get(0).map_err(UserFromRowError::CannotDecodeId)?;
        let email = row
            .try_get(1)
            .map_err(UserFromRowError::CannotDecodeEmail)?;
        Ok(User { id, email })
    }
}

impl CoreDb {
    pub async fn add_new_user(
        &self,
        id: impl ConvertTo<UserId>,
        email: impl ConvertTo<EmailAddress>,
    ) -> Result<(), AddNewUserError> {
        let id = id.convert()?;
        let email = email.convert()?;
        let res = sqlx::query("INSERT INTO users (id, email) VALUES ($1, $2)")
            .bind(id)
            .bind(&email)
            .execute(&self.pool)
            .await
            .map_err(|e| {
                if let Some(db_err) = e.as_database_error() {
                    if db_err.kind() == sqlx::error::ErrorKind::UniqueViolation {
                        AddNewUserError::AlreadyExists { id, email }
                    } else {
                        AddNewUserError::Database(e)
                    }
                } else {
                    AddNewUserError::Database(e)
                }
            })?;
        if res.rows_affected() != 1 {
            return Err(AddNewUserError::InvalidAffectedRowsAmount(
                res.rows_affected(),
            ));
        }
        Ok(())
    }

    pub async fn get_user_id(
        &self,
        email: impl ConvertTo<EmailAddress>,
    ) -> Result<UserId, GetUserIdError> {
        let email = email.convert()?;
        sqlx::query_scalar("SELECT id FROM users WHERE email = $1")
            .bind(&email)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if let sqlx::Error::RowNotFound = e {
                    GetUserIdError::NotFound(email)
                } else {
                    GetUserIdError::Database(e)
                }
            })
    }

    pub async fn get_user(
        &self,
        id: impl ConvertTo<UserId>,
    ) -> Result<User, GetUserError> {
        let id = id.convert()?;
        let row = sqlx::query("SELECT id, email FROM users WHERE users.id = $1")
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(|e| {
                if let sqlx::Error::RowNotFound = e {
                    GetUserError::NotFound(id)
                } else {
                    GetUserError::Database(e)
                }
            })?;
        Ok(row.convert()?)
    }
}
