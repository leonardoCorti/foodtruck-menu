use axum::{Router, response::Html, routing::get};

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:31151")
        .await
        .unwrap();

    println!("Server running on http://127.0.0.1:31151");

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> Html<&'static str> {
    Html(
        r#"
        <!DOCTYPE html>
        <html>
        <head>
            <title>Test Page</title>
        </head>
        <body>
            <h1>Hello world</h1>
            <p>This is a test HTML page served at /</p>
        </body>
        </html>
        "#,
    )
}
