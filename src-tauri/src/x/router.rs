use rspc::RouterBuilder;

pub(crate) fn router() -> RouterBuilder {
    RouterBuilder::new()
        .query("getAppName", |t| {
            t(|_: (), s: String| Ok(format!("rspc Test Project: {}", s)))
        })
        .query("getAppName2", |t| {
            t(|_: (), s: String| Ok(format!("rspc Test Project: {}", s)))
        })
}
