mod change_pass;
mod login;
mod refresh;

use change_pass::*;
use login::*;
use refresh::*;

use utoipa::OpenApi;

use app::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Authentication
        auth_login,
        auth_refresh,
        change_user_pass,
    ),
    tags(
        (name = "Authentication"),
    )
)]
pub struct AuthApiDoc;

pub fn declare_routes(base_path: &str) -> axum::Router<AppState> {
    axum::Router::new()
        .route(
            &format!("{base_path}/login"),
            axum::routing::post(auth_login),
        )
        .route(
            &format!("{base_path}/refresh"),
            axum::routing::post(auth_refresh),
        )
        .route(
            &format!("{base_path}/password"),
            axum::routing::put(change_user_pass),
        )
}
