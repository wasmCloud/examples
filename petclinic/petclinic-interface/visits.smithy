namespace org.wasmcloud.examples.petclinic

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U64

use org.wasmcloud.examples.petclinic#Date
use org.wasmcloud.examples.petclinic#Time

@wasmbus( actorReceive: true )
service Visits {
  version: "0.1",
  operations: [ListVisits, RecordVisit]
}

/// Retrieve a list of visits for a given owner and an optional
/// list of pet IDs
operation ListVisits {
    input: ListVisitsRequest,
    output: VisitList
}

/// Records a new visit
operation RecordVisit {
    input: RecordVisitRequest,
    output: Boolean
}

/// Request to list visits
structure ListVisitsRequest {
    @required 
    ownerId: U64,
    petIds: PetIdList
}

list PetIdList {
    member: U64
}

/// Request to record a visit
structure RecordVisitRequest {
    @required
    ownerId: U64,
    @required
    visit: Visit
}

list VisitList {
    member: Visit
}

/// The core metadata for a veterinarian visit
structure Visit {    
    /// The date the visit occurred
    @required
    date: Date,
    /// The time the visit occurred
    @required
    time: Time,
    /// Description of the visit
    @required
    description: String,
    /// The ID of the pet involved in the visit
    @required
    petId: U64,
    /// ID of the veterinarian who saw the given pet on this visit
    @required
    vetId: U64,
    /// The ID of the owner for this visit
    @required     
    ownerId: U64
}