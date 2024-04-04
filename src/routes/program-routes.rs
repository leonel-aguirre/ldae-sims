use crate::{
    program_handlers::{
        create_program_handler, delete_program_by_code_handler, edit_program_by_code_handler,
        select_all_programs_handler, select_program_by_code_handler,
    },
    shared::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn program_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/programs", get(select_all_programs_handler))
        .route("/program", post(create_program_handler))
        .route(
            "/program/:id",
            get(select_program_by_code_handler)
                .patch(edit_program_by_code_handler)
                .delete(delete_program_by_code_handler),
        )
}
