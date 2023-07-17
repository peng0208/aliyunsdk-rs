#[allow(unused_imports)]
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fmt::Error;

    use chrono::prelude::{Local, Utc};

    use crate::core::{Client, Request, Response};
    use crate::core::sign::signature;

    const ACCESS_ID: &str = "";
    const ACCESS_SECRET: &str = "";
    const ENDPOINT: &str = "ecs.aliyuncs.com";
    const VERSION: &str = "2014-05-26";
    const ACTION: &str = "DescribeRegions";

    #[async_std::test]
    async fn request() {
        let client = Client::new(ACCESS_ID, ACCESS_SECRET);
        let req = Request::new(ENDPOINT)
            .set_version(VERSION)
            .set_action(ACTION);

        let mut res = client.do_request(req).await.unwrap();

        // let body = res.body_bytes().await;
        // println!("body: {:?}", body.unwrap());

        // let body_string = res.body_string().await;
        // println!("body_string: {:?}", body_string.unwrap());

        let body_json = res.body_map().await;
        println!("body_json: {:?}", body_json
            .unwrap()
            .as_object()
            .unwrap()["Regions"]["Region"]
            .as_array()
            .unwrap()
            .iter()
            .map(|x| {
                let x = x.as_object().unwrap();
                (x["RegionId"].as_str().unwrap(), x["LocalName"].as_str().unwrap())
            })
            .collect::<HashMap<&str, &str>>()
        );
    }

    #[test]
    fn sign() {
        let req = Request::new(ENDPOINT)
            .set_version(VERSION)
            .set_action(ACTION)
            .set_access_key_id(ACCESS_ID)
            .set_signature_nonce(Local::now().timestamp_subsec_nanos().to_string().as_str())
            .set_timestamp(Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string().as_str());

        let sign = signature(
            req.method(),
            ACCESS_SECRET,
            req.common_params(),
            req.action_params(),
        );

        println!("https://{}?{}", req.endpoint(), sign);
    }
}