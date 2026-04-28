use crate::models::AppState;
use axum::{Router, extract::State, response::Html, routing::get};
use tera::{Context, Tera};

pub fn page_routes() -> Router<AppState> {
    Router::new()
        .route("/", get(frontdesk))
        .route("/frontdesk", get(frontdesk))
        .route("/kitchen", get(kitchen))
        .route("/administrator", get(administrator))
}

fn create_tera() -> Tera {
    Tera::new("templates/**/*.html").unwrap()
}

pub fn render_template(template_name: &str, context: Context) -> String {
    static TERA: std::sync::OnceLock<Tera> = std::sync::OnceLock::new();
    let tera = TERA.get_or_init(create_tera);
    tera.render(template_name, &context).unwrap()
}

async fn frontdesk(State(state): State<AppState>) -> Html<String> {
    let config = state.config.lock().await.clone();
    let mut ctx = Context::new();
    ctx.insert("order_types", &config.order_types);
    ctx.insert(
        "order_types_json",
        &serde_json::to_string(&config.order_types).unwrap(),
    );
    Html(render_template("frontdesk.html", ctx))
}

async fn kitchen(_state: State<AppState>) -> Html<String> {
    let ctx = Context::new();
    Html(render_template("kitchen.html", ctx))
}

async fn administrator(State(state): State<AppState>) -> Html<String> {
    let config = state.config.lock().await.clone();
    let mut ctx = Context::new();
    ctx.insert("config", &config);
    ctx.insert(
        "order_types_json",
        &serde_json::to_string(&config.order_types).unwrap(),
    );
    Html(render_template("admin.html", ctx))
}


