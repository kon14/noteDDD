use chrono::{DateTime, Utc};
use uuid::Uuid;

use common::error::AppError;
use dmn::entities::user::{User, UserData};

#[derive(Debug)]
pub(crate) struct UserPg {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl From<User> for UserPg {
    fn from(dmn_user: User) -> Self {
        Self {
            id: dmn_user.id(),
            email: dmn_user.email().to_string(),
            created_at: dmn_user.created_at(),
            updated_at: dmn_user.updated_at(),
        }
    }
}

impl TryFrom<UserPg> for UserData {
    type Error = AppError;

    fn try_from(pg_user: UserPg) -> Result<Self, Self::Error> {
        let email = pg_user.email.try_into()?;
        Ok(UserData {
            id: pg_user.id,
            email,
            created_at: pg_user.created_at,
            updated_at: pg_user.updated_at,
        })
    }
}
