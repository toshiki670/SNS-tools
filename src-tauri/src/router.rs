use std::sync::Arc;

use rspc::Router;

pub(crate) fn router() -> Arc<Router> {
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
