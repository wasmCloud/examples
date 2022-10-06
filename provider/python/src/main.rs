//! main process for python capability-provider
//!
use lazy_static::lazy_static;
use log::info;
use pyprov::Service;
use std::convert::Infallible;
use tokio::sync::RwLock;
use wasmbus_rpc::provider::prelude::*;

// The Service is a singleton, because of python GIL
lazy_static! {
    static ref SINGLE_SERVICE: RwLock<Service> = RwLock::new(Service::default());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host_data = load_host_data().map_err(|e| {
        eprintln!("error loading host data: {}", &e.to_string());
        Box::new(e)
    })?;

    let runtime = tokio::runtime::Runtime::new().unwrap();
    runtime.block_on(async move {
        let mut s = SINGLE_SERVICE.write().await;
        let service = match Service::try_init(None).await {
            Ok(service) => service,
            Err(e) => {
                eprintln!("ERROR starting service: {}", e.to_string());
                return;
            }
        };
        *s = service;
        drop(s);

        if let Err(e) = provider_run(
            PythonProvider::default(),
            host_data,
            Some("Python Provider".to_string()),
        )
        .await
        {
            eprintln!("ERROR provider exited with {}", e.to_string());
        }
    });

    // in the unlikely case there are any stuck threads,
    // close them so the process has a clean exit
    runtime.shutdown_timeout(core::time::Duration::from_secs(5));
    eprintln!("INFO  python provider exiting");
    Ok(())
}

#[derive(Clone, Default)]
struct PythonProvider {}

// use default implementations of provider message handlers
impl ProviderDispatch for PythonProvider {}

// Because the service is a global singleton, and configured by environment variables,
// there are no linkdef properties saved per actor link,
// and we can inherit the default ProviderHandler methods for link handling.
#[async_trait]
impl ProviderHandler for PythonProvider {
    async fn shutdown(&self) -> Result<(), Infallible> {
        Service::shutdown().await;
        Ok(())
    }
}

#[async_trait]
impl MessageDispatch for PythonProvider {
    async fn dispatch(&self, _ctx: &Context, message: Message<'_>) -> Result<Vec<u8>, RpcError> {
        use wasmbus_rpc::common::MessageFormat;

        let method = message.method;

        let buf = match wasmbus_rpc::common::message_format(&message.arg) {
            (MessageFormat::Cbor, offset) => &message.arg[offset..],
            (MessageFormat::Empty, offset) => &message.arg[offset..],
            (fmt, offset) => {
                info!(
                    "received {} with arg format: {} size {}",
                    method,
                    &fmt,
                    message.arg.len() - offset
                );
                return Err(RpcError::Rpc("invalid message format".to_string()));
            }
        };

        // call into python and return result
        let service = SINGLE_SERVICE.read().await;
        let result = service.check_invoke(method, buf).await?;
        Ok(result)
    }
}
