use axum::{extract::Request, routing::IntoMakeService, serve::Serve, Router, ServiceExt};
use tokio::net::TcpListener;
use tower_http::normalize_path::NormalizePath;

use crate::{http::build_router, types::http::ServerAddress};
use app::state::AppState;
use common::error::AppError;

fn get_server_address(api_port: u16) -> String {
    format!("0.0.0.0:{}", api_port)
}

pub async fn build_http_server(
    app_state: AppState,
    api_base_url: &str,
    api_port: u16,
) -> Result<BuildHttpServerResponse, AppError> {
    let router = build_router(app_state.clone(), api_base_url);

    let server_addr = get_server_address(api_port);
    let listener = TcpListener::bind(&server_addr).await.map_err(|err| {
        AppError::internal_with_private(
            format!("Failed to bind TCP listener @ {server_addr}"),
            err.to_string(),
        )
    })?;
    let server = axum::serve(listener, ServiceExt::<Request>::into_make_service(router));

    Ok(BuildHttpServerResponse {
        server,
        server_addr: ServerAddress(server_addr),
    })
}

pub struct BuildHttpServerResponse {
    pub server: Serve<TcpListener, IntoMakeService<NormalizePath<Router>>, NormalizePath<Router>>,
    pub server_addr: ServerAddress,
}
