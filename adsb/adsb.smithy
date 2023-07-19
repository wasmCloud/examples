metadata package = [ { namespace: "org.adsb", crate: "adsb" } ]

namespace org.adsb

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64
use org.wasmcloud.model#F64

/// The Adsb service has a single method, calculate, which
/// calculates the factorial of its whole number parameter.
@wasmbus(
    contractId: "myorg:adsb",
    actorReceive: true,
    providerReceive: false,
)
service Adsb {
  version: "0.1",
  operations: [ HandleAdsbMsg ]
}

/// Calculates the factorial (n!) of the input parameter
operation HandleAdsbMsg {
  input: AdsbMsg,
}

structure AdsbMsg {
    station: Station,
    icao: U64,
    callSign: String,
    @optional
    altitude: U32,
    @optional
    squawk: String,
    latitude: F64,
    longitude: F64,
}

structure Station {
    id: String,
    name: String,
    latitude: F64,
    longitude: F64,
}
