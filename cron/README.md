# Cron (Timed Interval Invocations)

This example shows how to use the [cron interface](./interface/) to invoke an actor on a timed interval. This can be useful for many types of applications, from a reminder every morning at 8am to eat your breakfast to kicking off a database backup hourly.

This example consists of a cron [interface](./interface/), [provider](./provider/), and [actor](./actor/). The interface includes a single function, `timed_invoke`, that actors need to implement a handler for. The implementation of this interface is the provider, which lets you configure the link with a set of parameters documented in [the provider README](./provider/README.md).

## Run this Example

Ensure you have [wash](https://wasmcloud.com/docs/installation) installed.

```shell
# Run wasmCloud in the background
wash up -d
# Start the actor and provider
wash ctl start actor wasmcloud.azurecr.io/cron_logger:0.1.0
wash ctl start provider wasmcloud.azurecr.io/cron:0.1.0
# Link the actor and provider, specifying the cron expression
wash ctl link put <> <> wasmcloud:example:cron expression="0 * * * * * *"
```

## Rust Example Code

```rust
use wasmbus_rpc::actor::prelude::*;
use wasmcloud_interface_cron::*;
use wasmcloud_interface_logging::info;

#[derive(Debug, Default, Actor, HealthResponder)]
#[services(Actor, Cron)] // If you're using your own actor, add Cron to the services list
struct CronActor {}

#[async_trait]
impl Cron for CronActor { // Impl the Cron trait for your actor, with the timed_invoke function
    async fn timed_invoke(&self, _ctx: &Context, req: &u64) -> RpcResult<()> {
        info!("Timed invoke at {req}");
        Ok(())
    }
}
```

## Tinygo Example Code

```go
# TBD PLS HELP ME JORDAN
```
