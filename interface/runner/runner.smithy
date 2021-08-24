// runner.smithy
//
// Interface that contains a single Run method
//

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [
    {
        namespace: "org.wasmcloud.example.runner",
        crate: "wasmcloud_example_runner"
     }
]

namespace org.wasmcloud.example.runner

use org.wasmcloud.model#wasmbus

/// The Runner interface has a single Run method
@wasmbus(
    contractId: "wasmcloud:example:runner",
    actorReceive: true )
service Runner {
  version: "0.1",
  operations: [ Run ]
}

/// The Run operation takes an array of strings and returns an array of strings.
/// The interpretation of the inputs, and the meaning of the outputs,
/// is dependent on the implementation. 
/// Either input or output arrays may be empty.
operation Run { 
    input: StringList,
    output: StringList,
}

list StringList {
    member: String
}

