use sqlx::PgExecutor;

use crate::models::user::UserPg;
use common::error::AppError;

pub(crate) async fn create_user<'a>(
    db: impl PgExecutor<'a>,
    user_data: CreateUserDataPg,
) -> Result<UserPg, AppError> {
    const INTERNAL_ERR_STR: &str = "Failed to create user!";

    sqlx::query_as!(
        UserPg,
        r#"
        INSERT INTO users (
            email,
            password_hash
        )
        VALUES ($1::email, $2)
        RETURNING
            id,
            email,
            created_at,
            updated_at
        "#,
        user_data.email as _,
        user_data.password_hash,
    )
    .fetch_one(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))
}

#[derive(Debug)]
pub(crate) struct CreateUserDataPg {
    pub(crate) email: String,
    pub(crate) password_hash: String,
}
