use std::sync::Arc;

use axum::{
    debug_handler,
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    program_model::ProgramModel,
    program_schemas::{CreateProgramSchema, UpdateProgramSchema},
    shared::AppState,
};

// GET
#[debug_handler]
pub async fn select_all_programs_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(ProgramModel, "SELECT * FROM programs;",)
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error while fetching all programs.",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let programs = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": programs.len(),
        "data": {"programs": programs}
    });
    Ok(Json(json_response))
}

// GET
#[debug_handler]
pub async fn select_program_by_code_handler(
    Path(program_code): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        ProgramModel,
        "SELECT * FROM programs WHERE program_code = $1",
        program_code,
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error while fetching program with code: {program_code}.",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let program = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "data": {"program": program[0]}
    });

    Ok(Json(json_response))
}

// POST
#[debug_handler]
pub async fn create_program_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateProgramSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        ProgramModel,
        "INSERT INTO programs (program_code, display_name, program_type, duration_type, duration) VALUES ($1, $2, $3, $4, $5) RETURNING *",
        body.program_code.to_string(),
        body.display_name.to_string(),
        body.program_type.to_string(),
        body.duration_type.to_string(),
        body.duration,
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(program) => {
            let program_response = json!({"status": "success","data": json!({
                "program": program
            })});

            return Ok((StatusCode::CREATED, Json(program_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Program already exists.",
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

// PATCH
#[debug_handler]
pub async fn edit_program_by_code_handler(
    Path(program_code): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateProgramSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let found_row = sqlx::query_as!(
        ProgramModel,
        "SELECT * FROM programs WHERE program_code = $1",
        program_code
    )
    .fetch_one(&data.db)
    .await
    .unwrap_or_default();

    if found_row.program_code == "" {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Program with code: {} not found.", program_code)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let query_result = sqlx::query_as!(
        ProgramModel,
        "UPDATE programs SET display_name = $1, program_type = $2, duration_type = $3, duration = $4 WHERE program_code = $5 RETURNING *",
        body.display_name.unwrap_or(found_row.display_name),
        body.program_type.unwrap_or(found_row.program_type),
        body.duration_type.unwrap_or(found_row.duration_type),
        body.duration.unwrap_or(found_row.duration),
        program_code
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(program) => {
            let program_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "program": program
            })});

            return Ok(Json(program_response));
        }
        Err(err) => {
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({"status": "error","message": format!("{:?}", err)})),
            ));
        }
    }
}

// DELETE
#[debug_handler]
pub async fn delete_program_by_code_handler(
    Path(program_code): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!("DELETE FROM programs WHERE program_code = $1", program_code)
        .execute(&data.db)
        .await
        .unwrap()
        .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Program with code: {} not found.", program_code)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let json_response = serde_json::json!({
        "status": "success",
        "message": format!("Program with code: {} successfuly deleted.", program_code)
    });

    Ok(Json(json_response))
}
