// pingpong.smithy
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.wasmcloud.interface.pingpong.pingpong", crate: "pingpong" } ]

namespace org.wasmcloud.interface.pingpong.pingpong

use org.wasmcloud.model#wasmbus

/// Description of Pingpong service
@wasmbus( actorReceive: true )
service Pingpong {
  version: "0.1",
  operations: [ Ping ]
}

/// Pings an actor, expecting a Pong in return
operation Ping {
  output: String
}

