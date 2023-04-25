use axum::{Extension, Json};
use axum::extract::Query;
use axum::http::StatusCode;
use axum::response::{IntoResponse};
use serde::Deserialize;

use crate::todos::repo::Db;

#[derive(Debug, Deserialize, Default)]
pub struct Pagination {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

pub async fn todos_index(
    Extension(db): Extension<Db>,
    pagination: Option<Query<Pagination>>,
) -> Result<impl IntoResponse, StatusCode> {
    let todos = db.list().await.map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    Ok(Json(todos))
}