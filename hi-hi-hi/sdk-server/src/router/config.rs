use crate::handlers::config::*;
use axum::{
    Router,
    routing::{get, post},
};

pub fn config_router() -> Router<()> {
    let mut router = Router::new();

    // ------------------- POST ROUTES -------------------

    router = router.route(
        "/data_abtest_api/config/experiment/list",
        post(on_list_experiment),
    );

    router = router.route(
        "/account/device/api/listNewerDevices",
        post(on_list_devices),
    );

    router = router.route(
        "/{product}/combo/granter/api/compareProtocolVersion",
        post(on_compare_protocol_ver),
    );

    router = router.route("/sdk/dataUpload", post(on_data_upload));

    router = router.route("/device-fp/api/getFp", post(on_device_get_fp));

    // ------------------- GET ROUTES -------------------

    router = router.route(
        "/{product}/mdk/agreement/api/getAgreementInfos",
        get(on_mdk_get_agreement),
    );

    router = router.route(
        "/{product}/combo/granter/api/getConfig",
        get(on_combo_get_config),
    );

    router = router.route("/game_weather/weather/get_weather", get(on_get_weather));

    router = router.route(
        "/{product}/mdk/shield/api/loadConfig",
        get(on_mdk_load_config),
    );

    router = router.route("/combo/box/api/config/sdk/combo", get(on_box_config_combo));

    router = router.route("/device-fp/api/getExtList", get(on_get_ext_list));

    router = router.route("/report", get(on_report));

    router = router.route("/", get(on_report));

    router = router.route("/admin/mi18n/{*remainder}", get(on_admin_mi18n));

    router
}
