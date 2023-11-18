use super::RouterBuilder;

pub(crate) fn mount() -> RouterBuilder {
    RouterBuilder::new().query("getAppName", |t| {
        t(|_: (), s: String| Ok(format!("rspc Test Project: {}", s)))
    })
}
