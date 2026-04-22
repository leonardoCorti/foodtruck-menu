use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Order {
    pub id: u64,
    pub table: u32,
    pub plates: Vec<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub display_name: String,
    pub auto_refresh_ms: u64,
    pub sound_enabled: bool,
    pub order_types: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            display_name: "Tablet Display".to_string(),
            auto_refresh_ms: 5000,
            sound_enabled: true,
            order_types: vec![
                "Panino".to_string(),
                "Pasta".to_string(),
                "Prova".to_string(),
            ],
        }
    }
}

#[derive(Clone)]
pub struct AppState {
    pub orders: Arc<Mutex<VecDeque<Order>>>,
    pub config: Arc<Mutex<Config>>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            orders: Arc::new(Mutex::new(VecDeque::new())),
            config: Arc::new(Mutex::new(Config::default())),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}
