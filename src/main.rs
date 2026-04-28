mod models;
mod routes;

use axum::Router;
use models::AppState;
use routes::{api, pages};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    let config = models::load_settings();
    let state = AppState::with_config(config);

    let app = Router::new()
        .merge(pages::page_routes())
        .nest("/api", api::api_routes())
        .nest_service("/static", ServeDir::new("static"))
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:31151")
        .await
        .unwrap();
    println!("Server running on http://0.0.0.0:31151");
    axum::serve(listener, app).await.unwrap();
}
