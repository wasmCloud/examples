use wasmcloud_interface_httpclient::HttpResponse;
const FALLBACK_IMAGE_URL: &str = "https://i.imgur.com/WQxgQUb.jpg";

/// Helper function to parse thecatapi / dog.ceo API responses for the
/// embedded image_url
pub(crate) fn parse_api_response(api_response: HttpResponse) -> String {
    use serde_json::Value;
    match serde_json::from_slice::<Value>(&api_response.body) {
        Ok(v) => {
            if let Value::String(image_url) = &v["message"] {
                image_url.to_string()
            } else if let Some(Value::String(image_url)) = v[0].get("url") {
                image_url.to_string()
            } else {
                FALLBACK_IMAGE_URL.to_string()
            }
        }
        _ => FALLBACK_IMAGE_URL.to_string(),
    }
}
