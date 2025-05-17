use axum::{extract::State, Json};
use serde::Deserialize;
use utoipa::ToSchema;

use crate::{dtos::UserDto, types::error::PresentationError};
use app::{state::AppState, usecases::user::RegisterUserInput};

#[derive(Deserialize, ToSchema)]
pub(crate) struct CreateUserHttpRequestBody {
    pub(crate) email: String,
    pub(crate) password: String,
}

/// Registers a new User.
#[utoipa::path(
    tag = "Users",
    post,
    path = "/",
    responses(
        (status = 200, description = "Success", body = UserDto),
        (status = 500, description = "Failure"),
    ),
)]
pub(crate) async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserHttpRequestBody>,
) -> Result<Json<UserDto>, PresentationError> {
    let AppState {
        register_user_use_case,
        ..
    } = state;

    let input = payload.into();
    let user = register_user_use_case.execute(input).await?;

    let user_dto = user.into();
    Ok(Json(user_dto))
}

impl From<CreateUserHttpRequestBody> for RegisterUserInput {
    fn from(payload: CreateUserHttpRequestBody) -> Self {
        Self {
            email: payload.email,
            password: payload.password,
        }
    }
}
