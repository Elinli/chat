use axum::{extract::State, response::IntoResponse, Extension, Json};

use crate::{
    models::{ChatUser, Workspace},
    AppError, AppState, User,
};

pub(crate) async fn list_chat_users_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let users = Workspace::find_all_chat_users(user.ws_id as _, &state.pool).await?;
    Ok(Json(users))
}

pub(crate) async fn list_all_chat_users_handler(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let chats = ChatUser::fetch_all(&state.pool).await?;
    Ok(Json(chats))
}
