use axum::{
    extract::{Multipart, Path, State},
    http::{self, HeaderMap},
    response::IntoResponse,
    Extension, Json,
};
use tokio::fs;
use tracing::warn;

use crate::{models::ChatFile, AppError, AppState, User};

// send
pub(crate) async fn send_message_handler() -> impl IntoResponse {
    "send message".to_string()
}

// list
pub(crate) async fn list_message_handler() -> impl IntoResponse {
    "list message".to_string()
}

//  upload file
pub(crate) async fn upload_file_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    mut multipart: Multipart,
) -> Result<impl IntoResponse, AppError> {
    let ws_id = user.ws_id as u64;
    let base_dir = state.config.server.base_dir.join(ws_id.to_string());

    let mut files = vec![];
    while let Some(field) = multipart.next_field().await.unwrap() {
        let filename = field.file_name().map(|s| s.to_string());
        let (Some(filename), Ok(file_data)) = (filename, field.bytes().await) else {
            warn!("Failed to read multipart field");
            continue;
        };
        let file = ChatFile::new(&filename, &file_data);
        let path = file.path(&base_dir);
        if path.exists() {
            warn!("File {} already exists: {:?}", filename, path);
        } else {
            fs::create_dir_all(path.parent().expect("file path parent should exists")).await?;
            fs::write(path, file_data).await?;
        }
        files.push(file.url(ws_id));
    }
    Ok(Json(files))
}

// download file
pub(crate) async fn download_file_handler(
    Extension(user): Extension<User>,
    State(state): State<AppState>,
    Path((ws_id, path)): Path<(i64, String)>,
) -> Result<impl IntoResponse, AppError> {
    if user.ws_id != ws_id {
        return Err(AppError::NotFound(
            "Not found or you don't have permission".into(),
        ));
    }

    let base_dir = state.config.server.base_dir.join(ws_id.to_string());

    let path = base_dir.join(path);
    if !path.exists() {
        return Err(AppError::NotFound("File doesn't exist".into()));
    }

    let mime = mime_guess::from_path(&path).first_or_octet_stream();
    let body = fs::read(path).await?;
    let mut headers = HeaderMap::new();
    headers.insert(http::header::CONTENT_TYPE, mime.as_ref().parse().unwrap());
    Ok((headers, body))
}
