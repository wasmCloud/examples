const FALLBACK_IMAGE_URL: &str = "https://i.imgur.com/WQxgQUb.jpg";

pub(crate) struct AnimalPic {
    pub(crate) image_url: String,
}

impl AnimalPic {
    fn new(image_url: String) -> Self {
        AnimalPic { image_url }
    }
}

impl From<Vec<u8>> for AnimalPic {
    fn from(source_bytes: Vec<u8>) -> Self {
        use serde_json::Value;
        match serde_json::from_slice::<Value>(&source_bytes) {
            Ok(v) => {
                if let Value::String(image_url) = &v["message"] {
                    AnimalPic::new(image_url.to_string())
                } else if let Some(Value::String(image_url)) = v[0].get("url") {
                    AnimalPic::new(image_url.to_string())
                } else {
                    AnimalPic::new(FALLBACK_IMAGE_URL.to_string())
                }
            }
            _ => AnimalPic::new(FALLBACK_IMAGE_URL.to_string()),
        }
    }
}
