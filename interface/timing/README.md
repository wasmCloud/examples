# wasmcloud-interface-timing

[![crates.io](https://img.shields.io/crates/v/wasmcloud-interface-timing.svg)](https://crates.io/crates/wasmcloud-interface-timing)
[![Documentation](https://docs.rs/wasmcloud-interface-timing/badge.svg)](https://docs.rs/wasmcloud-interface-timing)

Interface definition for the "wasmcloud:timing" capability contract. This 
contract allows actors to sleep for a specified duration, and also to 
retrieve the current system time on the wasmcloud host. 

Useful for implementing rate limits or other time-based functionality, although 
it is important to take RPC timeouts into account when using this interface.

## Example
```rust
use wasmcloud_interface_timing::TimingSender;
use wasmbus_rpc::actor::prelude::*;
use wasmbus_rpc::Timestamp;

async fn sleep_for_5_seconds(ctx: &Context) -> RpcResult<()> {
    let timing = TimingSender::new();
    timing.sleep(ctx, &5).await?;
}

async fn sleep_until_5_seconds(ctx: &Context) -> RpcResult<()> {
    let timing = TimingSender::new();
    let now = sleepy::now();
    let five_seconds = Timestamp::new(now.sec + 5, now.nsec);
    timing.sleep_until(ctx, &five_seconds).await?;
}

async fn get_current_time(ctx: &Context) -> RpcResult<Timestamp> {
    let timing = TimingSender::new();
    sleepy.now(ctx).await
}
```