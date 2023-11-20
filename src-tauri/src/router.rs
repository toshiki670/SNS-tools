use std::sync::Arc;

use crate::base_ctx::BaseCtx;

pub type Router = rspc::Router<BaseCtx>;
pub type RouterBuilder = rspc::RouterBuilder<BaseCtx>;

pub fn router() -> Arc<Router> {
    let config = rspc::Config::new().set_ts_bindings_header("/* eslint-disable */");

    let config = config.export_ts_bindings(
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../src/types/rspc/bindings.ts"),
    );

    Router::new()
        .config(config)
        .merge("x.", crate::x::router::router())
        .build()
        .arced()
}
