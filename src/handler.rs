use std::sync::Arc;

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde_json::json;

use crate::{
    model::DegreeModel,
    schema::{CreateDegreeSchema, ParamOptions, SelectDegreeSchema},
    shared::AppState,
};

// GET
pub async fn degrees_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(SelectDegreeSchema, "SELECT * FROM degrees;",)
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all note items",
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
    opts: Option<Query<ParamOptions>>,
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let Query(opts) = opts.unwrap_or_default();

    // let limit = opts.limit.unwrap_or(10);
    // let offset = (opts.page.unwrap_or(1) - 1) * limit;
    let id = opts.id;

    let query_result = sqlx::query_as!(DegreeModel, "SELECT * FROM degrees WHERE id = $1", id,)
        .fetch_all(&data.db)
        .await;

    if query_result.is_err() {
        let error_response = serde_json::json!({
            "status": "fail",
            "message": "Something bad happened while fetching all note items",
        });
        return Err((StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)));
    }

    let notes = query_result.unwrap();

    let json_response = serde_json::json!({
        "status": "success",
        "results": notes.len(),
        "notes": notes
    });
    Ok(Json(json_response))
}
