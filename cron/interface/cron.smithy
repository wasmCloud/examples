// cron.smithy
// A simple service that invokes actors on a specified interval

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ { namespace: "org.wasmcloud.example.cron", crate: "cron" } ]

namespace org.wasmcloud.example.cron

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64

/// The Cron service has a single method, timed_invoke, which
/// invokes an actor after a specified interval
@wasmbus(
    contractId: "wasmcloud:example:cron",
    actorReceive: true )
service Cron {
  version: "0.1",
  operations: [ TimedInvoke ]
}

/// Invoked on an actor on the interval specified in the link
operation TimedInvoke {
  // Time since the epoch that the actor is being invoked at 
  input: U64,
}

