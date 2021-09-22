namespace org.wasmcloud.examples.petclinic

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U64

@wasmbus( actorReceive: true )
service Vets {
  version: "0.1",
  operations: [ListVets]
}

operation ListVets {
    output: VetList
}

list VetList {
    member: Vet
}

structure Vet {
    @required
    id: u64,
    @required 
    firstName: String,
    @required
    lastName: String,
    @required
    specialties: SpecialtyList
}

list SpecialtyList {
    member: String
}