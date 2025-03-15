use crate::models::OnDataUploadRsp;
use axum::Json;

pub async fn on_data_upload() -> Json<OnDataUploadRsp> {
    Json(OnDataUploadRsp::default())
}
