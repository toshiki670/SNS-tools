use crate::base_ctx::BaseCtx;
use crate::router::RouterBuilder;

use super::client;

pub fn router() -> RouterBuilder {
    RouterBuilder::new()
        .query("getAppName", |t| {
            t(|_: BaseCtx, _: ()| async { client::call().await.unwrap() })
        })
        .query("getAppName2", |t| {
            t(|_: BaseCtx, s: String| Ok(format!("rspc Test Project: {}", s)))
        })
}
