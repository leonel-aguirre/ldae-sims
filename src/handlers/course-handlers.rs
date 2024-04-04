use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{course_model::CourseModel, course_schemas::CreateCourseSchema, shared::AppState};

// GET
#[debug_handler]
pub async fn select_all_courses_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(CourseModel, "SELECT * FROM courses;",)
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error while fetching all courses.",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let courses = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": courses.len(),
        "data": {"courses": courses}
    });
    Ok(Json(json_response))
}

// GET
#[debug_handler]
pub async fn select_course_by_code_handler(
    Path(course_code): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        CourseModel,
        "SELECT * FROM courses WHERE course_code = $1",
        course_code,
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error while fetching course with code: {course_code}.",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let course = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "data": {"course": course[0]}
    });

    Ok(Json(json_response))
}

// GET
#[debug_handler]
pub async fn select_course_by_program_handler(
    Path(program_code): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        CourseModel,
        "SELECT * FROM courses WHERE program_code = $1",
        program_code,
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error while fetching course by program with code: {program_code}.",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let courses = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "data": {"courses": courses}
    });

    Ok(Json(json_response))
}

// GET
#[debug_handler]
pub async fn select_course_by_specialization_handler(
    Path(specialization_code): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        CourseModel,
        "SELECT * FROM courses WHERE specialization_code = $1",
        specialization_code,
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error while fetching course by specialization with code: {specialization_code}.",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let courses = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "data": {"courses": courses}
    });

    Ok(Json(json_response))
}

// POST
#[debug_handler]
pub async fn create_course_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateCourseSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result;

    let body_specialization_code = body.specialization_code.unwrap_or("".to_string());

    if body_specialization_code == "" {
        query_result = sqlx::query_as!(
        CourseModel,
        "INSERT INTO courses (course_code, display_name, program_code) VALUES ($1, $2, $3) RETURNING *",
        body.course_code.to_string(),
        body.display_name.to_string(),
        body.program_code.to_string(),
    )
    .fetch_one(&data.db)
    .await;
    } else {
        query_result = sqlx::query_as!(
        CourseModel,
        "INSERT INTO courses (course_code, display_name, program_code, specialization_code) VALUES ($1, $2, $3, $4) RETURNING *",
        body.course_code.to_string(),
        body.display_name.to_string(),
        body.program_code.to_string(),
        body_specialization_code
    )
    .fetch_one(&data.db)
    .await;
    }

    match query_result {
        Ok(course) => {
            let course_response = json!({"status": "success","data": json!({
                "course": course
            })});

            return Ok((StatusCode::CREATED, Json(course_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Course already exists.",
                });
                return Err((StatusCode::CONFLICT, Json(error_response)));
            }
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", e)})),
            ));
        }
    }
}

// FIXME: Not working as expected when passing null to specialization_code either from body or from the actual value in the row.
// // PATCH
// #[debug_handler]
// pub async fn edit_course_by_code_handler(
//     Path(course_code): Path<String>,
//     State(data): State<Arc<AppState>>,
//     Json(body): Json<UpdateCourseSchema>,
// ) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
//     let found_row = sqlx::query_as!(
//         CourseModel,
//         "SELECT * FROM courses WHERE course_code = $1",
//         course_code
//     )
//     .fetch_one(&data.db)
//     .await
//     .unwrap_or_default();

//     if found_row.course_code == "" {
//         let error_response = serde_json::json!({
//             "status": "fail",
//             "message": format!("Course with code: {} not found.", course_code)
//         });
//         return Err((StatusCode::NOT_FOUND, Json(error_response)));
//     }

//     let query_result;
//     let found_row_specialization_code = found_row.specialization_code.unwrap_or("".to_string());
//     let body_specialization_code = body
//         .specialization_code
//         .unwrap_or(found_row_specialization_code);

//     if body_specialization_code == "" {
//         query_result = sqlx::query_as!(
//         CourseModel,
//         "UPDATE courses SET display_name = $1, program_code = $2, specialization_code = NULL WHERE course_code = $3 RETURNING *",
//         body.display_name.unwrap_or(found_row.display_name),
//         body.program_code.unwrap_or(found_row.program_code),
//         course_code
//     )
//     .fetch_one(&data.db)
//     .await;
//     } else {
//         query_result = sqlx::query_as!(
//         CourseModel,
//         "UPDATE courses SET display_name = $1, program_code = $2, specialization_code = $3 WHERE course_code = $4 RETURNING *",
//         body.display_name.unwrap_or(found_row.display_name),
//         body.program_code.unwrap_or(found_row.program_code),
//         body_specialization_code,
//         course_code
//     )
//     .fetch_one(&data.db)
//     .await;
//     }

//     match query_result {
//         Ok(course) => {
//             let course_response = serde_json::json!({"status": "success","data": serde_json::json!({
//                 "course": course
//             })});

//             return Ok(Json(course_response));
//         }
//         Err(err) => {
//             return Err((
//                 StatusCode::INTERNAL_SERVER_ERROR,
//                 Json(json!({"status": "error","message": format!("{:?}", err)})),
//             ));
//         }
//     }
// }

// DELETE
#[debug_handler]
pub async fn delete_course_by_code_handler(
    Path(course_code): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM courses WHERE course_code = $1", course_code)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Course with code: {} not found.", course_code)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let json_response = serde_json::json!({
        "status": "success",
        "message": format!("Course with code: {} successfuly deleted.", course_code)
    });

    Ok(Json(json_response))
}
