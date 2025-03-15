use crate::models::OnListExperimentRsp;
use axum::Json;

pub async fn on_list_experiment() -> Json<OnListExperimentRsp> {
    Json(OnListExperimentRsp {
        success: true,
        ..Default::default()
    })
}
