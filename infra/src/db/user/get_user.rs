use sqlx::PgExecutor;

use crate::models::user::UserPg;
use common::error::AppError;
use dmn::entities::user::UniqueUserIdentifier;

pub(crate) async fn get_user<'a>(
    db: impl PgExecutor<'a>,
    user_id: &UniqueUserIdentifier,
) -> Result<UserPg, AppError> {
    #[allow(non_snake_case)]
    let INTERNAL_ERR_STR = format!("Failed to retrieve user ({user_id})!");
    #[allow(non_snake_case)]
    let NOT_FOUND_ERR_STR = format!("User ({user_id}) doesn't exist!");

    let (id, email) = match user_id {
        UniqueUserIdentifier::Id(id) => (Some(id), None),
        UniqueUserIdentifier::Email(ref email) => (None, Some(email.to_string())),
    };

    sqlx::query_as!(
        UserPg,
        r#"
        SELECT
            id,
            email,
            created_at,
            updated_at
        FROM users
        WHERE id = $1 OR email = $2
        "#,
        id,
        email,
    )
    .fetch_one(db)
    .await
    .map_err(|err| match err {
        sqlx::Error::RowNotFound => AppError::not_found(NOT_FOUND_ERR_STR),
        _ => AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()),
    })
}
