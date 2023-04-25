use axum::{Extension};
use axum::extract::Path;
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;
use crate::todos::repo::Db;

pub async fn todos_delete(Extension(db): Extension<Db>, Path(id): Path<Uuid>) -> impl IntoResponse {
    let result = db.delete(&id).await;
    match result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::NOT_FOUND,
    }
}