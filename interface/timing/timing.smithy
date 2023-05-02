// endpoint-enumerator-interface.smithy

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [{
     namespace: "org.wasmcloud.interface.timing",
     crate: "wasmcloud-interface-timing"
 }]

namespace org.wasmcloud.interface.timing

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32

/// Provides the capability to read the system time.
@wasmbus(
    contractId: "wasmcloud:timing",
    providerReceive: true,
)

service Timing {
    version: "0.1",
    operations: [ Now ]
}

/// Returns the current time as a `wasmbus_rpc::Timestamp` struct.
///
/// The returned timestamp has nanosecond precision, so care should be taken
/// to avoid timing attacks. If the timestamp will be made visible to users,
/// it's recommended to reduce the precision by truncating or removing the
/// `nsec` field.
/// ```ignore
/// let timing = TimingSender::new();
/// let now = timing.now(ctx).await?;
operation Now {
    output: Timestamp,
}


