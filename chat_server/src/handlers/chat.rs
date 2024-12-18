use axum::response::IntoResponse;
use tracing::info;

// list
pub(crate) async fn list_chat_handler() -> impl IntoResponse {
    info!("get here and process is successful");
    "list chat".to_string()
}
// create
pub(crate) async fn create_chat_handler() -> impl IntoResponse {
    "create chat".to_string()
}
// update
pub(crate) async fn update_chat_handler() -> impl IntoResponse {
    "update chat".to_string()
}
// delete
pub(crate) async fn delete_chat_handler() -> impl IntoResponse {
    "delete chat".to_string()
}