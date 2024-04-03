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
    schema::{CreateDegreeSchema, SelectDegreeSchema},
    shared::AppState,
};

pub async fn degrees_handler(
    State(data): State<Arc<AppState>>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let query_result = sqlx::query_as!(CreateDegreeSchema, "SELECT name FROM degrees;",)
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
