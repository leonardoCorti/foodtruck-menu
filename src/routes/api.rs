use crate::models::{AppState, Config, Order};
use axum::{
    Router,
    extract::Path,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{delete, get, post},
};
use tracing::{info, trace};

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
    info!("Creating new order: {:?}", order);
    let mut orders = state.orders.lock().await;
    orders.push_back(order);
    info!("Order created, total orders: {}", orders.len());
    StatusCode::CREATED
}

async fn get_orders(State(state): State<AppState>) -> Json<Vec<Order>> {
    let orders = state.orders.lock().await;
    trace!("Retrieving {} orders", orders.len());
    Json(orders.iter().cloned().collect())
}

async fn clear_order(State(state): State<AppState>) -> StatusCode {
    info!("Clearing next order from queue");
    let mut orders = state.orders.lock().await;
    if !orders.is_empty() {
        let order = orders.pop_front();
        info!("Cleared order: {:?}", order);
    } else {
        info!("No orders to clear");
    }
    StatusCode::NO_CONTENT
}

async fn clear_order_by_id(State(state): State<AppState>, Path(order_id): Path<u64>) -> StatusCode {
    info!("Clearing order with id: {}", order_id);
    let mut orders = state.orders.lock().await;
    if let Some(pos) = orders.iter().position(|o| o.id == order_id) {
        let order = orders.remove(pos);
        info!("Cleared order: {:?}", order);
    } else {
        info!("Order with id {} not found", order_id);
    }
    StatusCode::NO_CONTENT
}

async fn get_config(State(state): State<AppState>) -> Json<Config> {
    info!("Retrieving config");
    let config = state.config.lock().await;
    Json(config.clone())
}

async fn update_config(
    State(state): State<AppState>,
    Json(new_config): Json<Config>,
) -> Json<Config> {
    info!("Updating config: {:?}", new_config);
    let mut config = state.config.lock().await;
    *config = new_config.clone();
    info!("Config updated successfully");
    Json(new_config)
}

async fn save_settings(State(state): State<AppState>) -> Json<Config> {
    info!("Saving settings to file");
    let config = state.config.lock().await;
    crate::models::save_settings(&config);
    info!("Settings saved successfully");
    Json(config.clone())
}

async fn load_settings(State(state): State<AppState>) -> Json<Config> {
    info!("Loading settings from file");
    let config = crate::models::load_settings();
    let mut current = state.config.lock().await;
    *current = config.clone();
    info!("Settings loaded: {:?}", config);
    Json(config)
}
