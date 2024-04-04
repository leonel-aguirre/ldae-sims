use crate::{
    shared::AppState,
    specialization_handlers::{
        create_specialization_handler, delete_specialization_by_code_handler,
        edit_specialization_by_code_handler, select_all_specializations_handler,
        select_specialization_by_code_handler, select_specialization_by_program_handler,
    },
};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn specialization_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/specializations", get(select_all_specializations_handler))
        .route("/specialization", post(create_specialization_handler))
        .route(
            "/specialization/:specialization_code",
            get(select_specialization_by_code_handler)
                .patch(edit_specialization_by_code_handler)
                .delete(delete_specialization_by_code_handler),
        )
        .route(
            "/specializations/byprogram/:program_code",
            get(select_specialization_by_program_handler),
        )
}
