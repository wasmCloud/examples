use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_compact::{Noise, SecretKey};
use serde_json::Value;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender};
use wasmcloud_interface_logging::error;
use wasmcloud_interface_numbergen::{NumberGen, NumberGenSender};

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct SignActor {}

/// Implementation of HttpServer trait methods
#[async_trait]
impl HttpServer for SignActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        let key_path = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "key")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| "private-key".to_string());

        if &req.path == "/sign" && !req.body.is_empty() {
            match KeyValueSender::new().get(ctx, &key_path).await {
                Ok(get_resp) => match parse_secret_key(&get_resp.value) {
                    Some(sk) => {
                        let noise = make_noise(ctx).await;
                        let mut resp = HttpResponse::default();
                        let raw_sig = sk.sign(&req.body, Some(noise.to_owned())).to_vec();
                        resp.body = URL_SAFE_NO_PAD.encode(raw_sig).into_bytes();
                        resp.header.insert(
                            "Content-Type".into(),
                            vec!["application/octet-stream".into()],
                        );
                        Ok(resp)
                    }
                    None => {
                        error!("Invalid key at path '{}'", &key_path);
                        Ok(HttpResponse::not_found())
                    }
                },
                Err(e) => {
                    error!("Key lookup error key='{}': {}", key_path, e);
                    Ok(HttpResponse::not_found())
                }
            }
        } else {
            Ok(HttpResponse::bad_request("invalid request"))
        }
    }
}

fn parse_secret_key(value: &str) -> Option<SecretKey> {
    if let Ok(Value::Object(map)) = serde_json::from_str(value) {
        if let Some(Value::String(key)) = map.get("key") {
            return SecretKey::from_pem(key).ok();
        }
    }
    None
}

/// Generate a random 16-Byte nonce
async fn make_noise(ctx: &Context) -> Noise {
    let mut buf = [0u8; 16];
    let rand = NumberGenSender::new();
    for n in 0..3 {
        let val32 = rand.random_32(ctx).await.unwrap();
        buf[n * 4..n * 4 + 4].copy_from_slice(&val32.to_ne_bytes());
    }
    Noise::new(buf)
}
