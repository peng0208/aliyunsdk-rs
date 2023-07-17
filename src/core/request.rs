use std::collections::HashMap;
use http_types::convert::DeserializeOwned;
use serde_json::Value;
use surf;

#[derive(Debug)]
pub struct Request {
    method: String,
    endpoint: String,
    common_map: HashMap<String, String>,
    action_map: HashMap<String, String>,
}

impl Request {
    pub fn new(endpoint: &str) -> Self {
        Self {
            method: "POST".to_string(),
            endpoint: endpoint.to_string(),
            common_map: HashMap::from([
                ("Format".to_string(), "JSON".to_string()),
                ("SignatureMethod".to_string(), "HMAC-SHA1".to_string()),
                ("SignatureVersion".to_string(), "1.0".to_string()),
            ]),
            action_map: HashMap::new(),
        }
    }

    pub(crate) fn method(&self) -> &str {
        &self.method
    }

    pub(crate) fn endpoint(&self) -> &str {
        &self.endpoint
    }

    pub(crate) fn common_params(&self) -> &HashMap<String, String> {
        &self.common_map
    }

    pub(crate) fn action_params(&self) -> &HashMap<String, String> {
        &self.action_map
    }

    #[allow(dead_code)]
    pub(crate) fn set_method(mut self, value: &str) -> Self {
        self.method = value.to_string();
        self
    }

    pub fn set_format(self, value: &str) -> Self {
        self.set_common_param("Format", value)
    }

    pub fn set_action(self, value: &str) -> Self {
        self.set_common_param("Action", value)
    }

    pub fn set_version(self, value: &str) -> Self {
        self.set_common_param("Version", value)
    }

    pub(crate) fn set_access_key_id(self, value: &str) -> Self {
        self.set_common_param("AccessKeyId", value)
    }

    pub(crate) fn set_signature_nonce(self, value: &str) -> Self {
        self.set_common_param("SignatureNonce", value)
    }

    pub(crate) fn set_timestamp(self, value: &str) -> Self {
        self.set_common_param("Timestamp", value)
    }

    fn set_common_param(mut self, name: &str, value: &str) -> Self {
        self.common_map.insert(name.to_string(), value.to_string());
        self
    }

    pub fn set_param(mut self, name: &str, value: &str) -> Self {
        self.action_map.insert(name.to_string(), value.to_string());
        self
    }

    pub fn set_params(mut self, params: &[(&str, &str)]) -> Self {
        for (k, v) in params {
            self.action_map.insert(k.to_string(), v.to_string());
        }
        self
    }
}

#[derive(Debug)]
pub struct Response {
    base: surf::Response,
}

impl Response {
    pub fn new(base: surf::Response) -> Self {
        Self { base }
    }

    pub fn resp(&self) -> &surf::Response { &self.base }

    pub fn status(&self) -> String { self.base.status().to_string() }

    pub async fn body_bytes(&mut self) -> Option<Vec<u8>> {
        match self.base.body_bytes().await {
            Ok(data) => Some(data),
            Err(_) => None
        }
    }

    pub async fn body_string(&mut self) -> Option<String> {
        match self.base.body_string().await {
            Ok(data) => Some(data),
            Err(_) => None
        }
    }

    pub async fn body_map(&mut self) -> Option<Value> {
        match self.base.body_json().await {
            Ok(data) => Some(data),
            Err(_) => {
                None
            }
        }
    }

    pub async fn body_json<T: DeserializeOwned>(&mut self) -> Option<T> {
        match self.base.body_json().await {
            Ok(data) => Some(data),
            Err(_) => {
                None
            }
        }
    }
}