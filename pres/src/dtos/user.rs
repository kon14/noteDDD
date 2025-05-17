use chrono::{DateTime, Utc};
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use dmn::entities::user::User;

#[derive(Debug, Serialize, ToSchema)]
#[schema(title = "User")]
pub(crate) struct UserDto {
    pub(crate) id: Uuid,
    pub(crate) email: String,
    pub(crate) created_at: DateTime<Utc>,
    pub(crate) updated_at: DateTime<Utc>,
}

impl From<User> for UserDto {
    fn from(user: User) -> Self {
        Self {
            id: user.id(),
            email: user.email().to_string(),
            created_at: user.created_at(),
            updated_at: user.updated_at(),
        }
    }
}
