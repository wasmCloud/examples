#[macro_use]
extern crate serde_json;
extern crate wapc_guest as guest;
use guest::prelude::*;
use wasmcloud_actor_core as actor;
use wasmcloud_actor_extras as extras;
use wasmcloud_actor_http_server as http;

#[actor::init]
fn init() {
  http::Handlers::register_handle_request(display_extras);
}

fn display_extras(_payload: http::Request) -> HandlerResult<http::Response> {
  let extras = extras::default();

  let result = json!(
  { "random": extras.request_random(0, 100)?,
    "guid" : extras.request_guid()?,
    "sequence": extras.request_sequence()?,
  });
  Ok(http::Response::json(result, 200, "OK"))
}
