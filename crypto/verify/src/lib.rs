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

/// the errors returned to the http client are intentionally non-descriptive, to avoid revealing too much to assist a potential attacker.
/// Each kind of error, such as missing keys, parse errors, etc., has a unique tag defined in this Error enum, and can be seen in the wasmcloud host logs.
#[derive(Debug)]
enum Error {
    /// url needs a 'sig' query parameter
    MissingSignature,
    /// the sig query parameter did not contain valid base64 (using the url-safe character set)
    SignatureBase64,
    /// the sig was valid base64, but ed25519 didn't recognize it as a 'Signature' object.
    SignatureType,
    /// the url was missing '/verify'
    InvalidRequestUrl,
    /// the request body was empty - content is required to verify the signature.
    EmptyBody,
    /// error communicating with provider
    KvRpcError,
    /// signature parsed correctly, and body was non-empty, but the signature did not match the content.
    /// Once the app is debugged and clietns are sending correct urls and the vault contains the correct keys,
    /// this should be the most common error and indicates that the signature and/or content changed since the signature was generated.
    InvalidSignature,
    /// the vault returned invalid or unexpected json at the key path. It is expecting a blob with a single value:  '{"key":"...."}'
    KeyIsInvalidJson,
    /// the value of the key did not parse as a PEM formatted key
    KeyPemFormat,
    /// the url key path didn't match an existing key, or the mount point in the link definition is wrong
    NoSuchKey,
}

#[async_trait]
impl HttpServer for VerifyActor {
    async fn handle_request(&self, ctx: &Context, req: &HttpRequest) -> RpcResult<HttpResponse> {
        match self.verify_key(ctx, req).await {
            Ok(()) => Ok(HttpResponse::default()),
            // we only return two http error codes: 403 for invalid signature, 404 for all other errors
            Err(Error::InvalidSignature) => Ok(HttpResponse {
                status_code: HTTP_STATUS_BAD_SIG,
                ..Default::default()
            }),
            Err(e) => {
                error!("Verification failed with error {:?}", e);
                Ok(HttpResponse::not_found())
            }
        }
    }
}

impl VerifyActor {
    async fn verify_key(&self, ctx: &Context, req: &HttpRequest) -> Result<(), Error> {
        if req.body.is_empty() {
            return Err(Error::EmptyBody);
        }
        if &req.path != "/verify" {
            return Err(Error::InvalidRequestUrl);
        }
        // get key path from 'key' query parameter. if missing, defaults to 'public-key'
        let key_path = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "key")
            .map(|(_, v)| v.to_string())
            .unwrap_or_else(|| "public-key".to_string());
        // get signature from 'sig' parameter. If missing, or not base64, or not the right length,
        // returns an error status and message with correct syntax
        let signature = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "sig")
            .map(|(_, v)| v.to_string())
            .ok_or(Error::MissingSignature)
            .and_then(|sig| {
                URL_SAFE_NO_PAD
                    .decode(sig)
                    .map_err(|_| Error::SignatureBase64)
            })
            .and_then(|sig| Signature::from_slice(&sig).map_err(|_| Error::SignatureType))?;

        KeyValueSender::new()
            .get(ctx, &key_path)
            .await
            .map_err(|e| {
                error!("kv rpc error: {:?}", e);
                Error::KvRpcError
            })
            .and_then(|get_resp| parse_public_key(&get_resp.value))
            .and_then(|pub_key| {
                pub_key
                    .verify(&req.body, &signature)
                    .map_err(|_| Error::InvalidSignature)
            })
    }
}

fn parse_public_key(value: &str) -> Result<PublicKey, Error> {
    if let Value::Object(map) = serde_json::from_str(value).map_err(|_| Error::KeyIsInvalidJson)? {
        if let Some(Value::String(key)) = map.get("key") {
            Ok(PublicKey::from_pem(key).map_err(|_| Error::KeyPemFormat)?)
        } else {
            Err(Error::NoSuchKey)
        }
    } else {
        Err(Error::KeyIsInvalidJson)
    }
}
