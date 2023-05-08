# wasmcloud-interface-timing

[![crates.io](https://img.shields.io/crates/v/wasmcloud-interface-timing.svg)](https://crates.io/crates/wasmcloud-interface-timing)
[![Documentation](https://docs.rs/wasmcloud-interface-timing/badge.svg)](https://docs.rs/wasmcloud-interface-timing)

Interface definition for the "wasmcloud:timing" capability contract. This 
contract allows actors to retrieve the current system time on the wasmcloud host. 

The `Timestamp` struct has nanosecond precision, so if it will be exposed to 
users at any point, care should be taken to avoid timing attacks by truncating 
the `nsec` field or to setting it to `0`.

The default implementation of this capability contract truncates the `Timestamp` 
to millisecond precision, but it may be necessary to reduce the precision even 
further. 
## Example
```rust
use wasmcloud_interface_timing::TimingSender;
use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::Timestamp;

async fn get_current_time(ctx: &Context) -> RpcResult<Timestamp> {
    let timing = TimingSender::new();
    timing.now(ctx).await
}
```