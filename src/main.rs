use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::routing::get;
use axum::Json;
use serde_json::json;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    // Start tracing.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // TODO: Create connection pool here, to pass into app.

    let app = axum::Router::new()
        .fallback(fallback)
        .route("/status/", get(status));

    // Azure specifies the port in the PORT environment variable.
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .expect("PORT must be a valid u16");

    // Run our application as a hyper server on http://localhost:3000.
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    tracing::debug!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}

// axum handler for any request that fails to match the router routes.
// This implementation returns HTTP status code Not Found (404).
pub async fn fallback(uri: axum::http::Uri) -> impl axum::response::IntoResponse {
    (
        axum::http::StatusCode::NOT_FOUND,
        format!("No route {}", uri),
    )
}

async fn status() -> Result<impl IntoResponse, StatusCode> {
    Ok("Ok!")
}
