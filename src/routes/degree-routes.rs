use crate::{degree_handlers::select_degree_by_id_handler, shared::AppState};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn degree_routes() -> Router<Arc<AppState>> {
    Router::new().route("/degrees", get(select_degree_by_id_handler))
}
