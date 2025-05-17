mod bootstrap;
mod utils;

use pres::utils::BuildHttpServerResponse;

#[tokio::main]
async fn main() {
    bootstrap::setup_env();

    let app_state = bootstrap::build_app_state().await.unwrap();

    let BuildHttpServerResponse {
        server,
        server_addr,
    } = bootstrap::build_http_server(app_state).await.unwrap();

    println!("Server listening on: http://{server_addr}");
    server.await.unwrap();
}
