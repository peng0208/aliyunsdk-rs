use chrono::prelude::{Local, Utc};

use super::request::{Request, Response};
use super::sign::signature;

#[derive(Debug)]
pub struct Client {
    scheme: String,
    access_key_id: String,
    access_key_secret: String,
}

impl Client {
    pub fn new(id: &str, secret: &str) -> Self {
        Self {
            scheme: "https".to_string(),
            access_key_id: id.to_string(),
            access_key_secret: secret.to_string(),
        }
    }

    #[allow(dead_code)]
    fn set_scheme(&mut self, value: &str) -> &Self {
        self.scheme = value.to_string();
        self
    }

    pub async fn do_request(&self, request: Request) -> Result<Response, http_types::Error> {
        let response = surf::Client::new()
            .send(self.build_request(request))
            .await?;

        Ok(Response::new(response))
    }

    fn build_request(&self, request: Request) -> surf::Request {
        let request = request
            .set_access_key_id(&self.access_key_id)
            .set_signature_nonce(Local::now().timestamp_subsec_nanos().to_string().as_str())
            .set_timestamp(Utc::now().format("%Y-%m-%dT%H:%M:%SZ").to_string().as_str());

        let method = request.method();

        let url = format!(
            "{}://{}?{}",
            self.scheme,
            request.endpoint(),
            signature(
                method,
                &self.access_key_secret,
                request.common_params(),
                request.action_params(),
            )
        );

        match method {
            "GET" => surf::get(url).build(),
            _ => surf::post(url).build()
        }
    }
}

