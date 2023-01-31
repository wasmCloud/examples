use base64::{engine::general_purpose::URL_SAFE_NO_PAD, Engine as _};
use ed25519_compact::{PublicKey, Signature};
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_httpserver::{HttpRequest, HttpResponse, HttpServer, HttpServerReceiver};
use wasmcloud_interface_keyvalue::{KeyValue, KeyValueSender};
use wasmcloud_interface_logging::error;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, HttpServer)]
struct VerifyActor {}

#[async_trait]
impl HttpServer for VerifyActor {
    // Handle http server request
    // To avoid giving away helpful information to an attacker, all errors return not_found (404),
    // Diagnostic information is sent to the host error logs
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        if &req.path != "/verify" || req.body.is_empty() {
            error!("invalid url or mising request content");
            return Ok(HttpResponse::not_found());
        }
        // get key path from 'key' query parameter. if missing, defaults to 'public-key'
        let key_path = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "key")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| "public-key".to_string());
        // get signature from 'sig' parameter, returning error if missing or unexpected format
        let signature = match form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "sig")
            .map(|(_, v)| v.to_string())
            .ok_or("missing signature")
            .and_then(|sig| URL_SAFE_NO_PAD.decode(sig).map_err(|_| "invalid base64"))
            .and_then(|sig| Signature::from_slice(&sig).map_err(|_| "signature format"))
        {
            Ok(sig) => sig,
            Err(e) => {
                error!("invalid signature: {}", e);
                return Ok(HttpResponse::not_found());
            }
        };

        // lookup the public key and verify the signature
        match KeyValueSender::new()
            .get(ctx, &key_path)
            .await
            .map_err(|e| format!("rpc: {}", e))
            .and_then(|get_resp| {
                serde_json::from_str::<KeyPem>(&get_resp.value).map_err(|e| format!("json: {}", e))
            })
            .and_then(|entry| PublicKey::from_pem(&entry.key).map_err(|e| format!("PEM: {}", e)))
            .and_then(|key| {
                key.verify(&req.body, &signature)
                    .map_err(|e| format!("invalid signature: {}", e))
            }) {
            Ok(()) => Ok(HttpResponse::default()),
            Err(e) => {
                error!("verification error: {}", e);
                Ok(HttpResponse::not_found())
            }
        }
    }
}

// pem-encoded secret key is stored in a json struct
#[derive(serde::Deserialize)]
struct KeyPem {
    key: String,
}
