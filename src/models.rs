use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::fs;
use std::path::Path;
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

    pub fn with_config(config: Config) -> Self {
        Self {
            orders: Arc::new(Mutex::new(VecDeque::new())),
            config: Arc::new(Mutex::new(config)),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

pub fn load_settings() -> Config {
    let path = Path::new("settings/settings.json");
    if path.exists()
        && let Ok(contents) = fs::read_to_string(path)
        && let Ok(config) = serde_json::from_str(&contents)
    {
        return config;
    }
    Config::default()
}

pub fn save_settings(config: &Config) {
    let path = "settings/settings.json";
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent).expect("Failed to create settings directory");
    }
    let json = serde_json::to_string_pretty(config).expect("Failed to serialize config");
    fs::write(path, json).expect("Failed to write settings file");
}
