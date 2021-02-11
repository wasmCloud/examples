// The sample subscriber actor illustrates handling
// a message from an externally-configured subscription
// and then prints the contents of that message to
// stdout using the `wasmcloud:logging` capability.

extern crate wapc_guest as guest;
use actorcore::HealthCheckResponse;
use guest::prelude::*;
use log::info;
use messaging::BrokerMessage;
use wasmcloud_actor_core as actorcore;
use wasmcloud_actor_logging as logging;
use wasmcloud_actor_messaging as messaging;

#[no_mangle]
pub fn wapc_init() {
    actorcore::Handlers::register_health_request(health);
    messaging::Handlers::register_handle_message(handle_message);
    logging::enable_macros();
}

fn handle_message(msg: BrokerMessage) -> HandlerResult<()> {
    info!("Received message broker message: {:?}", msg);
    Ok(())
}

fn health(_req: actorcore::HealthCheckRequest) -> HandlerResult<HealthCheckResponse> {
    Ok(HealthCheckResponse::healthy())
}
