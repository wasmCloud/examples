// endpoint-enumerator-interface.smithy

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [{
     namespace: "org.wasmcloud.interface.timing",
     crate: "wasmcloud-interface-timing"
 }]

namespace org.wasmcloud.interface.timing

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32

/// Allows actors to `sleep` for a specified duration, or until a desired time.
@wasmbus(
    contractId: "wasmcloud:timing",
    providerReceive: true,
)

service Timing {
    version: "0.1",
    operations: [ Sleep, SleepUntil, Now ]
}

/// Sleep for a specified number of milliseconds
/// ```ignore
///   let sleepy = SleepySender::new();
///   // sleep for 5 seconds
///   sleepy.sleep(ctx, &5000).await?;
operation Sleep {
    input: U32,
}

/// Sleep until a specified time, provided as a `wasmbus_rpc::Timestamp` struct.
/// If the specified time is in the past, the operation will return immediately.
/// ```ignore
///  let sleepy = SleepySender::new();
///  let now = sleepy.now(ctx).await?;
///  let five_seconds = Timestamp::new(now.sec + 5, now.nsec);
///  // sleep until 5 seconds from now
///  sleepy.sleep_until(ctx, &five_seconds).await
operation SleepUntil {
    input: Timestamp,
}

/// Returns the current time as a `wasmbus_rpc::Timestamp` struct.
/// ```ignore
/// let sleepy = SleepySender::new();
/// let now = sleepy.now(ctx).await?;
operation Now {
    output: Timestamp,
}


