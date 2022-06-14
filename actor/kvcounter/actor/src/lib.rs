use httpserver::*;

wit_bindgen_rust::import!("../keyvalue.wit");
wit_bindgen_rust::export!("../httpserver.wit");

#[derive(Default, Clone)]
pub struct Httpserver;

impl httpserver::Httpserver for Httpserver {
    fn handle_request(req: HttpRequest) -> Result<HttpResponse, RpcError> {
        // make friendlier key
        let key = format!("counter:{}", req.path.replace('/', ":"));

        // bonus: use specified amount from query, or 1
        let amount: i32 = form_urlencoded::parse(req.query_string.as_bytes())
            .find(|(n, _)| n == "amount")
            .map(|(_, v)| v.parse::<i32>())
            .unwrap_or(Ok(1))
            .unwrap_or(1);

        // increment the value in kv and send response in json
        let (body, status_code) = match increment_counter(key, amount) {
            Ok(v) => (serde_json::json!({ "counter": v }).to_string(), 200),
            // if we caught an error, return it to client
            Err(e) => (
                serde_json::json!({ "error": format!("{:?}", e) }).to_string(),
                500,
            ),
        };
        let resp = HttpResponse {
            body: body.as_bytes().to_vec(),
            status_code,
            header: Vec::new(),
        };
        Ok(resp)
    }
}

fn increment_counter(key: String, value: i32) -> Result<i32, RpcError> {
    keyvalue::increment(&key, value).map_err(map_wit_err)
}

fn map_wit_err(e: keyvalue::RpcError) -> RpcError {
    // TODO: Actually map the error
    RpcError::Other(format!("{:?}", e))
}
