use crate::handlers::auth::*;
use axum::{Router, routing::post};

pub fn auth_router() -> Router<()> {
    let mut router = Router::new();

    router = router.route("/account/risky/api/check", post(on_risky_api_check));

    router = router.route(
        "/{product}/combo/granter/login/v2/login",
        post(on_combo_granter_login),
    );

    router = router.route("/{product}/mdk/shield/api/login", post(on_mdk_shield_api));

    router = router.route("/{product}/mdk/shield/api/verify", post(on_mdk_shield_api));

    router
}
