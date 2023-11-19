use reqwest;

pub async fn call() -> Result<String, reqwest::Error> {
    let url = "https://httpbin.org/ip";
    let contents = reqwest::get(url).await?.text().await?;
    println!("{:?}", contents);
    Ok(contents)
}
