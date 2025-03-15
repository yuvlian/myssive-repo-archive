use crate::handlers::*;
use axum::{
    Router,
    routing::{get, post},
};

pub fn endfield_router() -> Router<()> {
    let mut r = Router::new();

    // GET routes
    r = r.route("/api/game/get_latest", get(api_game_get_latest::handle));
    r = r.route(
        "/api/gameBulletin/version",
        get(api_game_bulletin_version::handle),
    );
    r = r.route("/api/meta", get(api_meta::handle));
    r = r.route(
        "/api/remote_config/get_remote_config/1003/prod-cbt/default/default/network_config",
        get(api_remote_config_network::handle),
    );
    r = r.route(
        "/api/remote_config/get_remote_config/1003/prod-cbt/default/default/server_config_EUAndUS",
        get(api_remote_config_server::handle),
    );
    r = r.route(
        "/api/remote_config/get_remote_config/1003/prod-cbt/default/Windows/game_config",
        get(api_remote_config_game::handle),
    );
    r = r.route("/app/v1/config", get(app_config::handle));
    r = r.route(
        "/game/user/v1/query_zone_whitelist",
        get(game_user_zone_whitelist::handle),
    );
    r = r.route("/user/info/v1/basic", get(user_info_basic::handle));
    r = r.route(
        "/user/oauth2/v1/check_whitelist",
        get(user_oauth_check_whitelist::handle),
    );

    // POST routes
    r = r.route(
        "/u8/pay/getAllProductList",
        post(u8_pay_product_list::handle),
    );
    r = r.route(
        "/u8/user/auth/v2/token_by_channel_token",
        post(u8_user_auth_token::handle),
    );
    r = r.route(
        "/user/auth/v1/token_by_email_password",
        post(user_auth_token_email::handle),
    );
    r = r.route("/user/oauth2/v2/grant", post(user_oauth_grant::handle));

    r
}
