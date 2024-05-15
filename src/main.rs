use std::sync::Arc;

use axum::Router;
use dotenv::dotenv;
use ldae_sims::{
    base_routes::base_routes, course_routes::course_routes, program_routes::program_routes,
    shared::AppState, specialization_routes::specialization_routes,
};
use sqlx::postgres::PgPoolOptions;

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
            println!("✅ Connection to the database is successful!");

            pool
        }
        Err(err) => {
            println!("🔥 Failed to connect to the database: {:?}", err);
            std::process::exit(1);
        }
    };

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("🔥 could not run SQLx migrations");

    let app_state = Arc::new(AppState { db: pool.clone() });

    let all_routes = program_routes()
        .merge(specialization_routes())
        .merge(course_routes());

    let router = Router::new()
        .merge(base_routes())
        .nest("/api", all_routes)
        .with_state(app_state);

    println!("🚀 Server started successfully");
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, router).await.unwrap();
}
