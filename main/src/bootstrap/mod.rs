use dotenv::dotenv;
use std::sync::Arc;

use app::state::AppState;
use common::error::AppError;
use infra::{
    auth::{
        authenticator::JwtAuthenticator, pass_hasher::BcryptPasswordHasher,
        pass_service::DefaultPasswordService, token_adapter::JwtTokenAdapter,
    },
    get_pg_pool,
    repos::{auth_token::PgAuthTokenRepository, note::PgNoteRepository, user::PgUserRepository},
    tx::PgUnitOfWork,
};
use pres::utils::BuildHttpServerResponse;

pub(crate) fn setup_env() {
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("error"));

    // Assert Envs
    let _ = crate::utils::get_api_base_url();
    let _ = crate::utils::get_auth_jwt_secret();
}

pub(crate) async fn build_app_state() -> Result<AppState, AppError> {
    // Postgres
    let db_url = crate::utils::get_database_url();
    let pg_pool = get_pg_pool(&db_url).await?;
    let db = Arc::new(pg_pool);

    // Repositories
    let unit_of_work = Arc::new(PgUnitOfWork::new(db.clone()));
    let note_repo = Arc::new(PgNoteRepository::new(db.clone()));
    let user_repo = Arc::new(PgUserRepository::new(db.clone()));
    let auth_token_repo = Arc::new(PgAuthTokenRepository::new(db.clone()));

    // Authentication
    let auth_jwt_secret = crate::utils::get_auth_jwt_secret();
    let auth_access_token_duration_secs = crate::utils::get_auth_access_token_duration_secs();
    let auth_refresh_token_duration_secs = crate::utils::get_auth_access_token_duration_secs();
    let token_adapter = Arc::new(JwtTokenAdapter::new(auth_jwt_secret));
    let authenticator = Arc::new(JwtAuthenticator::new(
        auth_access_token_duration_secs,
        auth_refresh_token_duration_secs,
        token_adapter,
        unit_of_work,
        auth_token_repo,
        user_repo.clone(),
    ));
    let pass_hasher = Arc::new(BcryptPasswordHasher {});
    let pass_service = Arc::new(DefaultPasswordService::new(
        user_repo.clone(),
        pass_hasher.clone(),
    ));

    // App State
    let app_state = AppState::new(authenticator, pass_service, note_repo, user_repo);
    Ok(app_state)
}

pub(crate) async fn build_http_server(
    app_state: AppState,
) -> Result<BuildHttpServerResponse, AppError> {
    let api_base_url = crate::utils::get_api_base_url();
    let api_port = crate::utils::get_api_port();
    pres::utils::build_http_server(app_state, &api_base_url, api_port).await
}
