pub(crate) mod handlers;

use axum::routing::Router;
use std::collections::BTreeMap;
use tower::Layer;
use tower_http::normalize_path::{NormalizePath, NormalizePathLayer};
use utoipa::{
    openapi::security::{Http, HttpAuthScheme, SecurityScheme},
    OpenApi,
};
use utoipa_swagger_ui::SwaggerUi;

use app::state::AppState;
use handlers::{auth::AuthApiDoc, notes::NotesApiDoc, users::UsersApiDoc};

#[derive(OpenApi)]
#[openapi(
    info(
        title = "NoteDDD",
        description = "A note-taking API written in Rust using domain driven design.",
    ),
    nest(
        (path = "/auth", api = AuthApiDoc),
        (path = "/users", api = UsersApiDoc),
        (path = "/notes", api = NotesApiDoc),
    ),
)]
struct ApiDoc;

impl ApiDoc {
    pub fn new(api_base_url: &str) -> utoipa::openapi::OpenApi {
        let mut doc = Self::openapi();
        doc.servers = Some(vec![utoipa::openapi::Server::new(api_base_url)]);

        let mut security_schemes = BTreeMap::new();
        security_schemes.insert(
            "bearerAuth".to_string(),
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
        let mut components = doc.components.unwrap_or_default();
        components.security_schemes = security_schemes;
        doc.components = Some(components);

        doc
    }
}

pub(crate) fn build_router(app_state: AppState, api_base_url: &str) -> NormalizePath<Router> {
    let router = Router::new()
        .merge(setup_swagger_ui(api_base_url))
        .merge(handlers::auth::declare_routes("/auth"))
        .merge(handlers::notes::declare_routes("/notes"))
        .merge(handlers::users::declare_routes("/users"))
        .with_state(app_state);

    // Fix trailing slash endpoints
    NormalizePathLayer::trim_trailing_slash().layer(router)
}

fn setup_swagger_ui(api_base_url: &str) -> SwaggerUi {
    // NormalizeLayer breaks Swagger UI
    const SWAGGER_UI_PATH: &str = "/swagger"; // /swagger/index.html
    const SWAGGER_API_DOC_PATH: &str = "/swagger.json";

    let config = utoipa_swagger_ui::Config::new([SWAGGER_API_DOC_PATH])
        .try_it_out_enabled(true)
        .persist_authorization(true);

    SwaggerUi::new(SWAGGER_UI_PATH)
        .config(config)
        .url(SWAGGER_API_DOC_PATH, ApiDoc::new(api_base_url))
}
