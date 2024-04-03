use crate::{
    degree_handlers::{
        create_degree_handler, edit_degree_by_id_handler, select_all_degrees_handler,
        select_degree_by_id_handler,
    },
    shared::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn degree_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/degrees", get(select_all_degrees_handler))
        .route("/degree", post(create_degree_handler))
        .route(
            "/degree/:id",
            get(select_degree_by_id_handler).patch(edit_degree_by_id_handler),
        )
}
