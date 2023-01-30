use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_compact::{PublicKey, Signature};
use serde_json::Value;
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender};
use wasmcloud_interface_logging::error;

// response code for invalid signature
const HTTP_STATUS_BAD_SIG: u16 = 403;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct VerifyActor {}

#[async_trait]
impl HttpServer for VerifyActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        // get key path from 'key' query parameter. if missing, defaults to 'public_key'
        let key_path = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "key")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| "public-key".to_string());
        // get signature from 'sig' parameter. If missing, or not base64, or not the right length,
        // returns an error status and message with correct syntax
        let signature = match form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "sig")
            .map(|(_, v)| v.to_string())
            .and_then(|sig| URL_SAFE_NO_PAD.decode(sig).ok())
            .and_then(|sig| Signature::from_slice(&sig).ok())
        {
            Some(sig) => sig,
            _ => {
                return Ok(HttpResponse::bad_request(
                    "missing or invalid signature (expecting url '/validate?sig=signature_in_base64[&key=key_path]')",
                ));
            }
        };

        let resp = if &req.path == "/verify" && !req.body.is_empty() {
            match KeyValueSender::new()
                .get(ctx, &key_path)
                .await
                .map(|get_resp| parse_public_key(&get_resp.value))
            {
                Ok(Some(pub_key)) => {
                    if pub_key.verify(&req.body, &signature).is_ok() {
                        HttpResponse::default()
                    } else {
                        HttpResponse {
                            status_code: HTTP_STATUS_BAD_SIG,
                            ..Default::default()
                        }
                    }
                }
                _ => {
                    error!("Invalid key at path '{}'", &key_path);
                    HttpResponse::not_found()
                }
            }
        } else {
            HttpResponse::bad_request("invalid request")
        };
        Ok(resp)
    }
}

fn parse_public_key(value: &str) -> Option<PublicKey> {
    if let Ok(Value::Object(map)) = serde_json::from_str(value) {
        if let Some(Value::String(key)) = map.get("key") {
            return PublicKey::from_pem(key).ok();
        }
    }
    None
}
