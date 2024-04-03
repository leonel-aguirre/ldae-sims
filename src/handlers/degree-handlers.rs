use std::sync::Arc;

use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    degree_model::DegreeModel,
    degree_schemas::{CreateDegreeSchema, ParamOptions},
    shared::AppState,
};

// GET
pub async fn select_all_degrees_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(DegreeModel, "SELECT * FROM degrees;",)
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error while fetching all degrees.",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let degrees = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": degrees.len(),
        "degrees": degrees
    });
    Ok(Json(json_response))
}

// GET
pub async fn select_degree_by_id_handler(
    options: Path<ParamOptions>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Path(options) = options;
    let id = options.id;

    let query_result = sqlx::query_as!(DegreeModel, "SELECT * FROM degrees WHERE id = $1", id,)
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Error while fetching degree with id: {id}.",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let degree = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "degree": degree[0]
    });

    Ok(Json(json_response))
}

// POST
pub async fn create_degree_handler(
    State(data): State<Arc<AppState>>,
    Json(body): Json<CreateDegreeSchema>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(
        DegreeModel,
        "INSERT INTO degrees (name) VALUES ($1) RETURNING *",
        body.name.to_string(),
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(degree) => {
            let degree_response = json!({"status": "success","data": json!({
                "degree": degree
            })});

            return Ok((StatusCode::CREATED, Json(degree_response)));
        }
        Err(e) => {
            if e.to_string()
                .contains("duplicate key value violates unique constraint")
            {
                let error_response = serde_json::json!({
                    "status": "fail",
                    "message": "Degree already exists.",
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
