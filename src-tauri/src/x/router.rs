use rspc::RouterBuilder;

use super::client;

pub(crate) fn router() -> RouterBuilder {
    RouterBuilder::new()
        .query("getAppName", |t| {
            t(|_: (), _: ()| async { client::call().await.unwrap() })
        })
        .query("getAppName2", |t| {
            t(|_: (), s: String| Ok(format!("rspc Test Project: {}", s)))
        })
}
