wit_bindgen::generate!({
    path: "../wit",
    world: "wasi-cloud-error",
});

use wasi_cloud_error::Error;

/// This struct implements WASI interfaces
struct WasmcloudWasiCloudError {}

impl crate::wasi_cloud_error::WasiCloudError for WasmcloudWasiCloudError {
    fn drop_error(e: Error) {
        println!("[debug][kv-provider] dropping error [{e}]...");
    }

    fn trace(e: Error) -> String {
        format!("[error] an error ocurred, code [{e}]")
    }
}

export_wasmcloud_wasi_kv_wasi_cloud_error!(WasmcloudWasiCloudError);
