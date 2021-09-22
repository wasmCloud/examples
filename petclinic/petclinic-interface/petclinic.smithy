// petclinic.smithy
//
// Definitions for the -internal- service protocol between participating actors
// in the wasmCloud Pet Clinic sample

// Tell the code generator how to reference symbols defined in this namespace
metadata package = [ 
  { 
    namespace: "org.wasmcloud.examples.petclinic", 
    crate: "petclinic_interface" 
  }
]

namespace org.wasmcloud.examples.petclinic

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U16
use org.wasmcloud.model#U8

structure Date {
    @required
    year: u16,
    @required
    month: u8,
    @required
    day: u8
}

structure Time {
  @required
  hour: u8,

  @required
  minute: u8
}

/// Description of Petclinic service
@wasmbus( actorReceive: true )
service Petclinic {
  version: "0.1",
  operations: [ Convert ]
}

/// Converts the input string to a result
operation Convert {
  input: String,
  output: String
}

