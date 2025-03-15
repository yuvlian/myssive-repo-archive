use crate::files::ASSETS_DIR;
use crate::handlers::*;
use axum::{
    Router,
    routing::{get, post},
};
use paste::paste;
use tower_http::services::ServeDir;

macro_rules! app {
    ($name:ident, [$([$route:expr, $handler:ident, $($method:ident),+]);+ $(;)?]) => {
        pub fn $name() -> Router<()> {
            let mut r = Router::new();

            $(
                paste! {
                    $(
                        r = r.route($route, [<$method>]($handler::$method));
                    )+
                }
            )+

            r = r.nest_service("/assets", ServeDir::new(ASSETS_DIR));

            r
        }
    };
}

app!(ultina_app, [
    ["/", index, get];
    ["/login", login, post, get];
    ["/no-account", no_account, get];
    ["/register", register, post];
]);
