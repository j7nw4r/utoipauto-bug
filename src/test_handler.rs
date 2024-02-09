use axum::http::StatusCode;
use axum::response::IntoResponse;


pub async fn test_handler() -> impl IntoResponse {
    StatusCode::OK.into_response()
}