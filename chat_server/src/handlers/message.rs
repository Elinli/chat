use axum::response::IntoResponse;

// send
pub(crate) async fn send_message_handler() -> impl IntoResponse {
    "send message".to_string()
}

// list
pub(crate) async fn list_message_handler() -> impl IntoResponse {
    "list message".to_string()
}