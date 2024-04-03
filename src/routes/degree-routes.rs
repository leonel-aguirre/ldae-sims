use crate::{
    degree_handlers::{select_all_degrees_handler, select_degree_by_id_handler},
    shared::AppState,
};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn degree_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/degrees", get(select_all_degrees_handler))
        .route("/degree/:id", get(select_degree_by_id_handler))
}
