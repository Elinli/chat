use axum::{extract::State, http::StatusCode, response::IntoResponse, Extension, Json};

use crate::{models::{Chat, CreateChat}, AppError, AppState, User};

// list
pub(crate) async fn list_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let chat = Chat::fetch_all(user.ws_id as _, &state.pool).await?;
    Ok((StatusCode::OK, Json(chat)))
}
// create
pub(crate) async fn create_chat_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Json(input): Json<CreateChat>,
) -> Result<impl IntoResponse, AppError> {
    let chat = Chat::create(input, user.ws_id as _, &state.pool).await?;
    Ok((StatusCode::CREATED, Json(chat)))
}
// update
pub(crate) async fn update_chat_handler() -> impl IntoResponse {
    "update chat".to_string()
}
// delete
pub(crate) async fn delete_chat_handler() -> impl IntoResponse {
    "delete chat".to_string()
}
