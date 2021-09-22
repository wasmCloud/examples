namespace org.wasmcloud.examples.petclinic

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U64

use org.wasmcloud.examples.petclinic#Date

@wasmbus( actorReceive: true )
service Customers {
  version: "0.1",
  operations: [CreateOwner, FindOwner, ListOwners, UpdateOwner,
               ListPetTypes, AddPet, RemovePet, UpdatePet, ListPets, FindPet]
}

operation CreateOwner {
  input: Owner,
  output: CreateOwnerReply
}

operation FindOwner {
  input: u64,
  output: FindOwnerReply
}

operation ListOwners {  
  output: OwnersList
}

operation UpdateOwner {
  input: Owner,
  output: UpdateOwnerReply
}

operation ListPetTypes {
    output: PetTypeList
}

operation AddPet {
    input: AddPetRequest,
    output: Boolean
}

operation RemovePet {
    input: U64,
    output: Boolean
}

operation UpdatePet {
    input: Pet,
    output: Boolean
}

operation ListPets {
    input: U64,
    output: PetList
}

operation FindPet {
    input: U64,
    output: FindPetReply
}

structure FindPetReply {
    pet: Pet
}

list PetTypeList {
    member: PetType
}

structure PetType {
    @required
    id: u64,
    @required
    name: String
}

structure Owner {
    @required
    id: u64,
    @required
    firstName: String,    
    lastName: String,    
    address: String,
    city: String,
    telephone: String,
    @required
    email: String
}

list OwnersList {
    member: Owner
}

structure CreateOwnerReply {
    @required
    success: Boolean,
    @required
    id: u64
}

structure FindOwnerReply {    
    owner: Owner
}

structure UpdateOwnerReply {
    @required
    success: Boolean
}

structure AddPetRequest {
    @required
    ownerId: u64,
    @required
    pet: Pet
}

structure Pet {
    @required
    id: U64,
    @required
    name: String,
    @required
    petType: u64,
    @required
    birthdate: Date
}

list PetList {
    member: Pet
}