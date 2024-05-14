use axum::{debug_handler, http::StatusCode, response::IntoResponse, Json};

// GET
#[debug_handler]
pub async fn base_handler() -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let json_response = serde_json::json!({
        "status": "Rust API server running.",
    });

    Ok(Json(json_response))
}
