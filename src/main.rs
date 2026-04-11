mod models;
mod routes;

use axum::Router;
use models::AppState;
use routes::{api, pages};

#[tokio::main]
async fn main() {
    let state = AppState::new();

    let app = Router::new()
        .merge(pages::page_routes())
        .nest("/api", api::api_routes())
        .with_state(state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:31151")
        .await
        .unwrap();
    println!("Server running on http://0.0.0.0:31151");
    axum::serve(listener, app).await.unwrap();
}
