use axum::{Extension, Json};
use axum::http::StatusCode;
use axum::response::IntoResponse;
use uuid::Uuid;
use serde::{Deserialize};
use crate::todos::repo::Db;
use crate::todos::domain::Todo;

#[derive(Debug, Deserialize)]
pub struct CreateTodo {
    text: String,
}
pub async fn todos_create(
    Extension(db): Extension<Db>,
    Json(input): Json<CreateTodo>,
) -> impl IntoResponse {
    let todo = Todo {
        id: Uuid::new_v4(),
        text: input.text,
        completed: false,
    };

    let ret = db.create(&todo).await;
    match ret {
        Ok(_) => StatusCode::CREATED,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }


}
