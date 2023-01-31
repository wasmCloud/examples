use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_compact::{Noise, SecretKey};
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
        // 'key' query parameter is the lookup path, defaults to 'private-key'
        let key_path = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "key")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| "private-key".to_string());

        if &req.path == "/sign" && !req.body.is_empty() {
            let noise = make_noise(ctx).await.to_owned();
            match KeyValueSender::new()
                .get(ctx, &key_path)
                .await
                .map_err(|e| format!("rpc error: {}", e))
                .and_then(|get_resp| {
                    serde_json::from_str::<KeyPem>(&get_resp.value)
                        .map_err(|e| format!("invalid json: {}", e))
                })
                .and_then(|entry| {
                    SecretKey::from_pem(&entry.key).map_err(|e| format!("invalid PEM: {}", e))
                })
                .map(|sk| sk.sign(&req.body, Some(noise)))
            {
                Ok(signature) => Ok(HttpResponse {
                    body: URL_SAFE_NO_PAD.encode(signature).into(),
                    ..Default::default()
                }),
                Err(e) => {
                    error!("Invalid or missing key at path '{}': {}", &key_path, e);
                    Ok(HttpResponse::not_found())
                }
            }
        } else {
            Ok(HttpResponse::bad_request("invalid request"))
        }
    }
}

// pem-encoded secret key is stored in a json struct
#[derive(serde::Deserialize)]
struct KeyPem {
    key: String,
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
