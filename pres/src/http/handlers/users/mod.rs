mod delete_self_user;
mod get_self_user;
mod register_user;
mod update_self_user;

use delete_self_user::*;
use get_self_user::*;
use register_user::*;
use update_self_user::*;

use utoipa::OpenApi;

use app::state::AppState;

#[derive(OpenApi)]
#[openapi(
    paths(
        // Users
        register_user,
        get_self_user,
        update_self_user,
        delete_self_user,
    ),
    tags(
        (name = "Users"),
    )
)]
pub struct UsersApiDoc;

pub fn declare_routes(base_path: &str) -> axum::Router<AppState> {
    axum::Router::new()
        .route(&format!("{base_path}"), axum::routing::post(register_user))
        .route(
            &format!("{base_path}/self"),
            axum::routing::get(get_self_user),
        )
        .route(
            &format!("{base_path}/self"),
            axum::routing::put(update_self_user),
        )
        .route(
            &format!("{base_path}/self"),
            axum::routing::delete(delete_self_user),
        )
}
