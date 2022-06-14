use serde::{Deserialize, Serialize};
use wasmbus_receiver::*;

wit_bindgen_rust::import!("../httpserver.wit");
wit_bindgen_rust::export!("../wasmbus-receiver.wit");

const HANDLE_REQUEST_METHOD: &str = "HttpServer.HandleRequest";

// TODO: We need to improve codegen to allow for hooking in custom derive traits on generated wit
// types so we don't have to do this

// TODO: I was getting deserialization errors for the body ("invalid type: sequence, expected a
// borrowed byte array at line 1 column 83") when I tried to do this completely borrowed (which is
// the most efficient thing to do here). When we do this for real, we'll need to figure out how to
// make that work

// #[derive(Debug, Default, Deserialize)]
// pub struct HttpRequestInternal<'a> {
//     /// HTTP method. One of: GET,POST,PUT,DELETE,HEAD,OPTIONS,CONNECT,PATCH,TRACE
//     #[serde(default)]
//     pub method: &'a str,
//     /// full request path
//     #[serde(default)]
//     pub path: &'a str,
//     /// query string. May be an empty string if there were no query parameters.
//     #[serde(default)]
//     pub query_string: &'a str,
//     /// map of request headers (string key, string value)
//     pub header: Vec<(&'a str, &'a str)>,
//     /// Request body as a byte array. May be empty.
//     #[serde(default)]
//     pub body: &'a [u8],
// }

// impl<'a> HttpRequestInternal<'a> {
//     fn as_req(&'a self) -> httpserver::HttpRequest<'a> {
//         httpserver::HttpRequest {
//             method: self.method,
//             path: self.path,
//             query_string: self.query_string,
//             header: &self.header,
//             body: self.body,
//         }
//     }
// }

#[derive(Debug, Default, Deserialize)]
pub struct HttpRequestInternal {
    /// HTTP method. One of: GET,POST,PUT,DELETE,HEAD,OPTIONS,CONNECT,PATCH,TRACE
    #[serde(default)]
    pub method: String,
    /// full request path
    #[serde(default)]
    pub path: String,
    /// query string. May be an empty string if there were no query parameters.
    #[serde(default)]
    pub query_string: String,
    /// map of request headers (string key, string value)
    pub header: Vec<(String, String)>,
    /// Request body as a byte array. May be empty.
    #[serde(default)]
    pub body: Vec<u8>,
}

#[derive(Default, Debug, Serialize)]
pub struct HttpResponseInternal {
    /// statusCode is a three-digit number, usually in the range 100-599,
    /// A value of 200 indicates success.
    #[serde(default)]
    pub status_code: u16,
    /// Map of headers (string keys, list of values)
    pub header: Vec<(String, String)>,
    /// Body of response as a byte array. May be an empty array.
    #[serde(default)]
    pub body: Vec<u8>,
}

impl From<httpserver::HttpResponse> for HttpResponseInternal {
    fn from(resp: httpserver::HttpResponse) -> Self {
        HttpResponseInternal {
            status_code: resp.status_code,
            header: resp.header,
            body: resp.body,
        }
    }
}

#[derive(Default, Clone)]
pub struct WasmbusReceiver;

impl wasmbus_receiver::WasmbusReceiver for WasmbusReceiver {
    fn receive(msg: Message) -> Result<Payload, RpcError> {
        if msg.method != HANDLE_REQUEST_METHOD {
            return Err(RpcError::MethodNotHandled(format!(
                "Method {} is not supported by the httpserver contract",
                msg.method
            )));
        }
        let req: HttpRequestInternal = serde_json::from_slice(&msg.arg)
            .map_err(|e| RpcError::Deser(format!("httpserver: {}", e)))?;
        let header: Vec<(&str, &str)> = req
            .header
            .iter()
            .map(|(k, v)| (k.as_str(), v.as_str()))
            .collect();
        let resp: HttpResponseInternal = httpserver::handle_request(httpserver::HttpRequest {
            method: &req.method,
            path: &req.path,
            query_string: &req.query_string,
            header: &header,
            body: &req.body,
        })
        .map_err(httpserver_to_wasmbus_error)?
        .into();
        serde_json::to_vec(&resp).map_err(|e| RpcError::Ser(e.to_string()))
    }
}

fn httpserver_to_wasmbus_error(e: httpserver::RpcError) -> RpcError {
    match e {
        httpserver::RpcError::ActorHandler(m) => RpcError::ActorHandler(m),
        httpserver::RpcError::DeadlineExceeded(m) => RpcError::DeadlineExceeded(m),
        httpserver::RpcError::Deser(m) => RpcError::Deser(m),
        httpserver::RpcError::HostError(m) => RpcError::HostError(m),
        httpserver::RpcError::InvalidParameter(m) => RpcError::InvalidParameter(m),
        httpserver::RpcError::MethodNotHandled(m) => RpcError::MethodNotHandled(m),
        httpserver::RpcError::Nats(m) => RpcError::Nats(m),
        httpserver::RpcError::NotImplemented => RpcError::NotImplemented,
        httpserver::RpcError::NotInitialized(m) => RpcError::NotInitialized(m),
        httpserver::RpcError::Other(m) => RpcError::Other(m),
        httpserver::RpcError::ProviderInit(m) => RpcError::ProviderInit(m),
        httpserver::RpcError::Rpc(m) => RpcError::Rpc(m),
        httpserver::RpcError::Ser(m) => RpcError::Ser(m),
        httpserver::RpcError::Timeout(m) => RpcError::Timeout(m),
    }
}
