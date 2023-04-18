// apigw.smithy

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "jammin.interfaces.apigw", crate: "jammin_interfaces_apigw" } ]

namespace jammin.interfaces.apigw

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64


/// The Route interface describes a service
/// that can deliver messages
@wasmbus(
    contractId: "jammin:interfaces:apigw",
    actorReceive: true )
service Apigw {
  version: "0.1",
  operations: [ Route ]
}

/// The RouteSubscriber interface describes
/// an actor interface that receives messages
/// sent by the Apigw Router Actor
@wasmbus(
    contractId: "jammin:interfaces:apigw",
    actorReceive: true )
service RoutedSubscriber {
  version: "0.1",
  operations: [ Route ]
}

structure RoutedRequest {
  // Use Path as Nats Subject
  @n(0)
  @required
  path: String
  
  @n(1)
  @required
  method: String

  @n(2)
  @required
  @sensitive
  body: Blob

  /// A timeout, in milliseconds
  @required
  @n(3)
  timeoutMs: U32,
}

structure RoutedResponse {
  // Use Path as Nats Subject
  @n(0)
  @required
  path: String
  
  @n(1)
  @required
  success: Boolean

  /// If success is false, this may contain an error
  @n(2)
  error: String

  @n(3)
  @required
  @sensitive
  body: Blob
}

/// Route Request - Expect Response
operation Route {
    input: RoutedRequest
    output: RoutedResponse
}