use std::sync::Arc;

use axum::{
    extract::{Path, Query},
    response::{Html, IntoResponse},
    routing::{get, get_service},
    Router,
};
use dotenv::dotenv;
use ldae_sims::{
    handler::{degrees_handler, select_degree_by_id_handler},
    shared::AppState,
};
use serde::Deserialize;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let pool = match PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
    {
        Ok(pool) => {
            println!("✅Connection to the database is successful!");
            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    let app_state = Arc::new(AppState { db: pool.clone() });

    let routes_all = Router::new()
        .nest("/api", routes_hello().merge(routes_static()))
        .with_state(app_state);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, routes_all).await.unwrap();
}

fn routes_static() -> Router<Arc<AppState>> {
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}

fn routes_hello() -> Router<Arc<AppState>> {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/:name", get(handler_hello2))
        // .route("/degrees", get(degrees_handler))
        .route("/degrees", get(select_degree_by_id_handler))
}

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");

    let name = params.name.as_deref().unwrap_or("World!");
    return Html(format!("Hello <strong>{name}</strong>"));
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello2 - {name:?}", "HANDLER");

    return Html(format!("Hello <strong>{name}</strong>"));
}
