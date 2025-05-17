use sqlx::PgExecutor;

use crate::models::user::UserPg;
use common::error::AppError;
use dmn::entities::user::UniqueUserIdentifier;

pub(crate) async fn update_user<'a>(
    db: impl PgExecutor<'a>,
    user_id: &UniqueUserIdentifier,
    user_data: UpdateUserDataPg,
) -> Result<UserPg, AppError> {
    #[allow(non_snake_case)]
    let INTERNAL_ERR_STR = format!("Failed to update user ({user_id})!");
    #[allow(non_snake_case)]
    let NOT_FOUND_ERR_STR = format!("User ({user_id}) doesn't exist!");

    if user_data.email.is_none() && user_data.password_hash.is_none() {
        return Err(AppError::bad_request("No fields to update!"));
    }

    let (id, email) = match user_id {
        UniqueUserIdentifier::Id(id) => (Some(id), None),
        UniqueUserIdentifier::Email(email) => (None, Some(email.to_string())),
    };

    sqlx::query_as!(
        UserPg,
        r#"
        UPDATE users
        SET
            email = COALESCE($3::email, email),
            password_hash = COALESCE($4, password_hash)
        WHERE id = $1 OR email = $2
        RETURNING
            id,
            email,
            created_at,
            updated_at
        "#,
        id,
        email,
        user_data.email as _,
        user_data.password_hash,
    )
    .fetch_one(db)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => AppError::not_found(NOT_FOUND_ERR_STR),
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })
}

#[derive(Debug)]
pub(crate) struct UpdateUserDataPg {
    pub(crate) email: Option<String>,
    pub(crate) password_hash: Option<String>,
}
