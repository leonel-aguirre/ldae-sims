use crate::{base_handlers::base_handler, shared::AppState};
use axum::{routing::get, Router};
use std::sync::Arc;

pub fn base_routes() -> Router<Arc<AppState>> {
    Router::new().route("/", get(base_handler))
}
