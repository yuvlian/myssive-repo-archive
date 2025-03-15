use crate::models::OnAdminMi18nRsp;
use axum::Json;

pub async fn on_admin_mi18n() -> Json<OnAdminMi18nRsp> {
    Json(OnAdminMi18nRsp { version: 107 })
}
