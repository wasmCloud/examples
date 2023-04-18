//! sleepy capability provider
//!
//!
use async_trait::async_trait;
use std::time::SystemTime;
use tokio::time::{sleep, Duration};
use wasmbus_rpc::common::Context;
use wasmbus_rpc::error::{RpcError, RpcResult};
use wasmbus_rpc::provider::prelude::*;
use wasmbus_rpc::Timestamp;
use wasmcloud_interface_timing::{Timing, TimingReceiver};

// main (via provider_main) initializes the threaded tokio executor,
// listens to lattice rpcs, handles actor links,
// and returns only when it receives a shutdown message
//
fn main() -> Result<(), Box<dyn std::error::Error>> {
    provider_main(
        TimingProvider::default(),
        Some("sleepy Provider".to_string()),
    )?;

    eprintln!("sleepy provider exiting");
    Ok(())
}

/// Timing capability provider implementation
/// contractId: "jclmnop:sleepy"
#[derive(Default, Clone, Provider)]
#[services(Timing)]
struct TimingProvider {}

/// use default implementations of provider message handlers
impl ProviderDispatch for TimingProvider {}
impl ProviderHandler for TimingProvider {}

#[async_trait]
impl Timing for TimingProvider {
    async fn sleep(&self, _ctx: &Context, duration_ms: &u32) -> RpcResult<()> {
        let duration = Duration::from_millis(*duration_ms as u64);
        sleep(duration).await;
        Ok(())
    }

    async fn sleep_until(
        &self,
        _ctx: &Context,
        timestamp: &Timestamp,
    ) -> RpcResult<()> {
        let now_duration = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .map_err(|e| RpcError::from(format!("System time error: {e}")))?;
        let end_duration = Duration::new(timestamp.sec as u64, timestamp.nsec);
        let sleep_duration = end_duration - now_duration;
        sleep(sleep_duration).await;
        Ok(())
    }

    async fn now(&self, _ctx: &Context) -> RpcResult<Timestamp> {
        Ok(Timestamp::now())
    }
}

