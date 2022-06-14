use wasmbus_sender::*;

wit_bindgen_rust::export!("../wasmbus-sender.wit");

#[derive(Default, Clone)]
pub struct WasmbusSender;

impl wasmbus_sender::WasmbusSender for WasmbusSender {
    fn send(
        msg: Message,
        contract_name: String,
        link_name: Option<String>,
    ) -> Result<Payload, RpcError> {
        println!(
            "Linkname: {}, contract_name: {}, msg: {:#?}",
            link_name.unwrap_or_else(|| "default".to_string()),
            contract_name,
            msg
        );
        Ok(serde_json::to_vec(&42).unwrap())
    }
}
