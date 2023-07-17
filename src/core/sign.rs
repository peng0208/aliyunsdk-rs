use std::collections::HashMap;

use base64::Engine;
use crypto::hmac::Hmac;
use crypto::mac::Mac;
use crypto::sha1::Sha1;
use url::form_urlencoded::byte_serialize;

fn url_encode(text: &str) -> String {
    let encoded_text: String = byte_serialize(text.as_bytes()).collect();
    encoded_text
        .replace('+', "%20")
        .replace(',', "%2A")
        .replace("%7E", "~")
}

fn base64_encode(secret: &str, body: &str) -> String {
    base64::engine::general_purpose::STANDARD.encode(hmac_sign(secret, body))
}

fn hmac_sign(secret: &str, body: &str) -> Vec<u8> {
    let mut mac = Hmac::new(Sha1::new(), (secret.to_string() + "&").as_bytes());
    mac.input(body.as_bytes());
    mac.result().code().to_vec()
}

pub(crate) fn signature(method: &str,
                        secret: &str,
                        common_params: &HashMap<String, String>,
                        action_params: &HashMap<String, String>) -> String {
    let common_pairs = common_params
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect::<Vec<(&str, &str)>>();

    let action_pairs = action_params
        .iter()
        .map(|(k, v)| (k.as_str(), v.as_str()))
        .collect::<Vec<(&str, &str)>>();

    let mut pairs = common_pairs
        .iter()
        .chain(&action_pairs)
        .collect::<Vec<_>>();
    pairs.sort_by_key(|item| item.0);

    let encoded_pairs = pairs
        .iter()
        .map(|(k, v)| format!("{}={}", url_encode(k), url_encode(v)))
        .collect::<Vec<_>>();

    let encoded_query = encoded_pairs.join("&");

    let string_to_sign = format!(
        "{}&{}&{}",
        method,
        url_encode("/"),
        url_encode(&encoded_query)
    );

    let sign_base64 = base64_encode(secret, &string_to_sign);
    format!("Signature={}&{}", url_encode(&sign_base64), encoded_query)
}