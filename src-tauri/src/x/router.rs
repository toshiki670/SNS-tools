use crate::base_ctx::BaseCtx;
use crate::router::RouterBuilder;

pub fn router() -> RouterBuilder {
    RouterBuilder::new()
        .query("getAppName", |t| {
            t(|_: BaseCtx, _: ()| async { call().await.unwrap() })
        })
        .query("getAppName2", |t| {
            t(|_: BaseCtx, s: String| Ok(format!("rspc Test Project: {}", s)))
        })
}

pub async fn call() -> Result<String, reqwest::Error> {
    let url = "https://httpbin.org/ip";
    let contents = reqwest::get(url).await?.text().await?;
    println!("{:?}", contents);
    Ok(contents)
}