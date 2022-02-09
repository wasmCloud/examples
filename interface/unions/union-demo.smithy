// union-demo.smithy
//
// Demonstrates how to use unions in smithy interfaces
//
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [{
    namespace: "org.wasmcloud.example.union_demo",
    crate: "wasmcloud-example-union-demo",
    py_module: "wasmcloud_example_union_demo",
}]

namespace org.wasmcloud.example.union_demo

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#n
use org.wasmcloud.model#U8
use org.wasmcloud.model#U16
use org.wasmcloud.model#U32
use org.wasmcloud.model#U64
use org.wasmcloud.model#F64

/// The Runner interface has a single Run method
@wasmbus(
    contractId: "wasmcloud:example:union_demo",
    actorReceive: true,
    providerReceive: true,
    protocol: "2" )
service UnionDemo {
  version: "0.1",
  operations: [ Get ]
}

operation Get {
    input: String,
    output: Response,
}


/// response contains either a map, for success, or error, for failure
union Response {

    @n(0)
    values: ValueMap

    @n(1)
    error: ErrorResponse
}

/// An error response contains an error message and optional stack trace
structure ErrorResponse {
    @required
    @n(0)
    message: String,

    @n(1)
    stacktrace: String,
}

/// Map a string key to an AnyValue
map ValueMap {
    key: String,
    value: AnyValue,
}

/// Union of various data types
union AnyValue {
    @n(0)
    valU8: U8,

    @n(1)
    valU16: U16,

    @n(2)
    valU32: U32,

    @n(3)
    valU64: U64,

    @n(4)
    valStr: String,

    @n(5)
    valF64: F64,

    @n(6)
    valBin: Blob,
}

