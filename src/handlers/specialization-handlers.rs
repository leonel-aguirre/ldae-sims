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
    shared::AppState,
    specialization_model::SpecializationModel,
    specialization_schemas::{CreateSpecializationSchema, UpdateSpecializationSchema},
};

// GET
#[debug_handler]
pub async fn select_all_specializations_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(SpecializationModel, "SELECT * FROM specializations;",)
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error while fetching all specializations.",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let specializations = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": specializations.len(),
        "data": {"specializations": specializations}
    });
    Ok(Json(json_response))
}

// GET
#[debug_handler]
pub async fn select_specialization_by_code_handler(
    Path(specialization_code): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        SpecializationModel,
        "SELECT * FROM specializations WHERE specialization_code = $1",
        specialization_code,
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error while fetching specialization with code: {specialization_code}.",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let specialization = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "data": {"specialization": specialization[0]}
    });

    Ok(Json(json_response))
}

// GET
#[debug_handler]
pub async fn select_specialization_by_program_handler(
    Path(program_code): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        SpecializationModel,
        "SELECT * FROM specializations WHERE program_code = $1",
        program_code,
    )
    .fetch_all(&data.db)
    .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error while fetching specialization by program with code: {program_code}.",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let specializations = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "data": {"specializations": specializations}
    });

    Ok(Json(json_response))
}

// POST
#[debug_handler]
pub async fn create_specialization_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateSpecializationSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        SpecializationModel,
        "INSERT INTO specializations (specialization_code, display_name, program_code) VALUES ($1, $2, $3) RETURNING *",
        body.specialization_code.to_string(),
        body.display_name.to_string(),
        body.program_code.to_string(),
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(specialization) => {
            let specialization_response = json!({"status": "success","data": json!({
                "specialization": specialization
            })});

            return Ok((StatusCode::CREATED, Json(specialization_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Specialization already exists.",
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
pub async fn edit_specialization_by_code_handler(
    Path(specialization_code): Path<String>,
    State(data): State<Arc<AppState>>,
    Json(body): Json<UpdateSpecializationSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let found_row = sqlx::query_as!(
        SpecializationModel,
        "SELECT * FROM specializations WHERE specialization_code = $1",
        specialization_code
    )
    .fetch_one(&data.db)
    .await
    .unwrap_or_default();

    if found_row.specialization_code == "" {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Specialization with code: {} not found.", specialization_code)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let query_result = sqlx::query_as!(
        SpecializationModel,
        "UPDATE specializations SET display_name = $1, program_code = $2 WHERE specialization_code = $3 RETURNING *",
        body.display_name.unwrap_or(found_row.display_name),
        body.program_code.unwrap_or(found_row.program_code),
        specialization_code
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(specialization) => {
            let specialization_response = serde_json::json!({"status": "success","data": serde_json::json!({
                "specialization": specialization
            })});

            return Ok(Json(specialization_response));
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
pub async fn delete_specialization_by_code_handler(
    Path(specialization_code): Path<String>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let rows_affected = sqlx::query!(
        "DELETE FROM specializations WHERE specialization_code = $1",
        specialization_code
    )
    .execute(&data.db)
    .await
    .unwrap()
    .rows_affected();

    if rows_affected == 0 {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": format!("Specialization with code: {} not found.", specialization_code)
        });
        return Err((StatusCode::NOT_FOUND, Json(error_response)));
    }

    let json_response = serde_json::json!({
        "status": "success",
        "message": format!("Specialization with code: {} successfuly deleted.", specialization_code)
    });

    Ok(Json(json_response))
}
