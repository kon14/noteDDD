use sqlx::PgExecutor;

use common::error::AppError;
use dmn::entities::user::UniqueUserIdentifier;

pub(crate) async fn delete_user<'a>(
    db: impl PgExecutor<'a>,
    user_id: &UniqueUserIdentifier,
) -> Result<(), AppError> {
    #[allow(non_snake_case)]
    let INTERNAL_ERR_STR = format!("Failed to delete user ({user_id})!");
    #[allow(non_snake_case)]
    let NOT_FOUND_ERR_STR = format!("Failed to delete user ({user_id})!");

    let (id, email) = match user_id {
        UniqueUserIdentifier::Id(id) => (Some(id), None),
        UniqueUserIdentifier::Email(ref email) => (None, Some(email.to_string())),
    };

    let result = sqlx::query!(
        r#"
        DELETE FROM users
        WHERE id = $1 OR email = $2
        "#,
        id,
        email,
    )
    .execute(db)
    .await
    .map_err(|err| AppError::internal_with_private(INTERNAL_ERR_STR, err.to_string()))?;
    if result.rows_affected() == 0 {
        Err(AppError::not_found(NOT_FOUND_ERR_STR))
    } else {
        Ok(())
    }
}
