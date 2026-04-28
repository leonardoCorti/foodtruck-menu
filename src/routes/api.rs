use crate::models::{AppState, Config, Order};
use axum::{
    Router,
    extract::Path,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
};

pub fn api_routes() -> Router<AppState> {
    Router::new()
        .route("/orders", post(create_order))
        .route("/orders", get(get_orders))
        .route("/orders/next", delete(clear_order))
        .route("/orders/{id}", delete(clear_order_by_id))
        .route("/config", get(get_config))
        .route("/config", post(update_config))
        .route("/settings/save", post(save_settings))
        .route("/settings/load", post(load_settings))
}

async fn create_order(State(state): State<AppState>, Json(order): Json<Order>) -> StatusCode {
    let mut orders = state.orders.lock().await;
    orders.push_back(order);
    StatusCode::CREATED
}

async fn get_orders(State(state): State<AppState>) -> Json<Vec<Order>> {
    let orders = state.orders.lock().await;
    Json(orders.iter().cloned().collect())
}

async fn clear_order(State(state): State<AppState>) -> StatusCode {
    let mut orders = state.orders.lock().await;
    if !orders.is_empty() {
        orders.pop_front();
    }
    StatusCode::NO_CONTENT
}

async fn clear_order_by_id(State(state): State<AppState>, Path(order_id): Path<u64>) -> StatusCode {
    let mut orders = state.orders.lock().await;
    if let Some(pos) = orders.iter().position(|o| o.id == order_id) {
        orders.remove(pos);
    }
    StatusCode::NO_CONTENT
}

async fn get_config(State(state): State<AppState>) -> Json<Config> {
    let config = state.config.lock().await;
    Json(config.clone())
}

async fn update_config(
    State(state): State<AppState>,
    Json(new_config): Json<Config>,
) -> Json<Config> {
    let mut config = state.config.lock().await;
    *config = new_config.clone();
    Json(new_config)
}

async fn save_settings(State(state): State<AppState>) -> Json<Config> {
    let config = state.config.lock().await;
    crate::models::save_settings(&config);
    Json(config.clone())
}

async fn load_settings(State(state): State<AppState>) -> Json<Config> {
    let config = crate::models::load_settings();
    let mut current = state.config.lock().await;
    *current = config.clone();
    Json(config)
}
