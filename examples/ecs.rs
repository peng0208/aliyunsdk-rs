use aliyunsdk_rs::{Client, Request};

#[tokio::main]
async fn main() {
    // example: DescribeRegions, get ecs regions
    // set client access key
    let client = Client::new("access_key_id", "access_key_secret");

    // build action request params
    let req = Request::new("ecs.aliyuncs.com")
        .set_version("2014-05-26")
        .set_action("DescribeRegions");

    // receive response
    let res = client.do_request(req).await.unwrap();

    println!("body_map: {:?}", res.body_map().await);
}
