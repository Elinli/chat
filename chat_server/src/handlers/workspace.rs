use axum::{extract::State, response::IntoResponse, Extension, Json};

use crate::{

    AppError, AppState, User,
};

pub(crate) async fn list_chat_users_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let users = state.fetch_chat_users_by_ws_id(user.ws_id as _).await?;
    Ok(Json(users))
}

pub(crate) async fn list_all_users_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let chats = state.fetch_all_users().await?;
    Ok(Json(chats))
}

