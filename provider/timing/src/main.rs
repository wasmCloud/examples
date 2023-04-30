//! sleepy capability provider
//!
//!
use async_trait::async_trait;
use wasmbus_rpc::common::Context;
use wasmbus_rpc::error::RpcResult;
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
    async fn now(&self, _ctx: &Context) -> RpcResult<Timestamp> {
        Ok(truncate_nanos(Timestamp::now()))
    }
}

/// Decrease resolution of timestamp from nanosecond to millisecond to reduce
/// the risk of timing attacks
fn truncate_nanos(timestamp: Timestamp) -> Timestamp {
    Timestamp {
        sec: timestamp.sec,
        nsec: timestamp.nsec - timestamp.nsec % 1_000_000
    }
}

#[cfg(test)]
mod tests {
    use super::truncate_nanos;
    use super::Timestamp;

    #[test]
    fn test_truncate_nanos() {
        let timestamp = truncate_nanos(Timestamp::new(1682884424, 123456789).unwrap());
        assert_eq!(timestamp.sec, 1682884424);
        assert_eq!(timestamp.nsec, 123000000);
    }
}