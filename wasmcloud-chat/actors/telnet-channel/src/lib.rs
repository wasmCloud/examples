extern crate wapc_guest as guest;
use guest::prelude::*;
use log::info;
use messaging::BrokerMessage;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_logging as logging;
use wasmcloud_actor_messaging as messaging;
use wasmcloud_actor_telnet as telnet;

#[actor::init]
fn init() {
    messaging::Handlers::register_handle_message(handle_message);
    telnet::Handlers::register_session_started(session_started);
    telnet::Handlers::register_receive_text(receive_text);
    logging::enable_macros();
}

fn handle_message(msg: BrokerMessage) -> HandlerResult<()> {
    info!("RECEIVED MESSAGE: {:?}", msg);
    Ok(())
}

fn session_started(msg: String) -> HandlerResult<bool> {
    info!("telnet session started: {}", msg);
    Ok(true)
}

//TODO: This is what happens when a person types in a message on the front end. Should then go ahead and send that message
fn receive_text(session: String, text: String) -> HandlerResult<bool> {
    info!("text ({}) received on session ({})", text, session);
    Ok(true)
}
