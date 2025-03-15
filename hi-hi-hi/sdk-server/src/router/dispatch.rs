use crate::handlers::dispatch::*;
use axum::{Router, routing::get};

pub fn dispatch_router() -> Router<()> {
    let mut router = Router::new();

    router = router.route("/query_dispatch", get(on_query_dispatch));

    router = router.route("/query_gateway", get(on_query_gateway));

    router
}
