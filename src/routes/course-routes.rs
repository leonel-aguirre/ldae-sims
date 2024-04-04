use crate::{
    course_handlers::{
        create_course_handler, delete_course_by_code_handler, select_all_courses_handler,
        select_course_by_code_handler, select_course_by_program_handler,
        select_course_by_specialization_handler,
    },
    shared::AppState,
};
use axum::{
    routing::{get, post},
    Router,
};
use std::sync::Arc;

pub fn course_routes() -> Router<Arc<AppState>> {
    Router::new()
        .route("/courses", get(select_all_courses_handler))
        .route("/course", post(create_course_handler))
        .route(
            "/course/:course_code",
            get(select_course_by_code_handler).delete(delete_course_by_code_handler),
        )
        .route(
            "/courses/byprogram/:program_code",
            get(select_course_by_program_handler),
        )
        .route(
            "/courses/byspecialization/:specialization_code",
            get(select_course_by_specialization_handler),
        )
}
