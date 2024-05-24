use axum::http::StatusCode;
use axum::response::IntoResponse;
use axum::Json;
use axum::{extract::State, routing::get, routing::post, Router};
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, Sqlite, SqlitePool};
use std::env;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[derive(Clone)]
struct AppState {
    db: SqlitePool,
}

#[tokio::main]
async fn main() {
    // Start tracing.
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .init();

    // "sqlite://sqlite.db";
    let DATABASE_URL = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");

    // Check if the database exists, if not create it.
    if !Sqlite::database_exists(&DATABASE_URL)
        .await
        .unwrap_or(false)
    {
        println!("Creating database {}", &DATABASE_URL);
        match Sqlite::create_database(&DATABASE_URL).await {
            Ok(_) => println!("Create db success"),
            Err(error) => panic!("error: {}", error),
        }
    } else {
        println!("Database already exists");
    }

    let db = SqlitePool::connect(&DATABASE_URL).await.unwrap();

    let state = AppState { db: db.clone() };

    let app = axum::Router::new()
        .fallback(fallback)
        .route("/status/", get(status))
        .route("/user/", post(create_user).get(list_users))
        .with_state(state);

    // Azure specifies the port in the PORT environment variable.
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
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

#[derive(Deserialize)]
struct CreateUserRequest {
    name: String,
}

async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    let name = &payload.name;
    sqlx::query("INSERT INTO users (name) VALUES (?)")
        .bind(name)
        .execute(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok("User created!")
}

#[derive(Serialize)]
struct User {
    id: i64,
    name: String,
}

async fn list_users(State(state): State<AppState>) -> Result<impl IntoResponse, StatusCode> {
    let users: Vec<User> = sqlx::query_as!(User, "SELECT * FROM users;")
        .fetch_all(&state.db)
        .await
        .unwrap();
    Ok(Json(users))
}
