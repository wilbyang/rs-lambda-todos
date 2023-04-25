use axum::{Extension, Json};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;
use serde::{Deserialize};
use crate::todos::repo::Db;

#[derive(Debug, Deserialize)]
pub struct UpdateTodo {
    text: Option<String>,
    completed: Option<bool>,
}
pub async fn todos_update(
    Extension(db): Extension<Db>,
    Path(id): Path<Uuid>,
    Json(input): Json<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let result = db.update(&id, input.text.unwrap_or("".into()).as_str(), input.completed.unwrap_or(false)).await;
    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::NOT_FOUND),
    }
}