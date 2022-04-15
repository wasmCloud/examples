// Owners - [/owners, /owners/ownerId]
export function getOwners() {
  return this.getRequest('/owners');
}

export function getOwner(ownerId) {
  return this.getRequest(`/owners/${ownerId}`);
}

export function createOwner(owner) {
  return this.modifyRequest(`/owners`, 'POST', owner);
}

export function updateOwner(ownerId, owner) {
  return this.modifyRequest(`/owners/${ownerId}`, 'PUT', owner);
}

// Pets - [/owners/ownerId/pets, /owners/ownerId/pets/petId]
export function getPets(ownerId) {
  return this.getRequest(`/owners/${ownerId}/pets`);
}

export function getPet(ownerId, petId) {
  return this.getRequest(`/owners/${ownerId}/pets/${petId}`);
}

export function createPet(ownerId, pet) {
  return this.modifyRequest(`/owners/${ownerId}/pets`, 'POST', pet);
}

export function updatePet(ownerId, petId, pet) {
  return this.modifyRequest(`/owners/${ownerId}/pets/${petId}`, 'PUT', pet);
}

export function deletePet(ownerId, petId) {
  return this.modifyRequest(`/owners/${ownerId}/pets/${petId}`, 'DELETE');
}

// Pet Visits = [/owners/ownerId/pets/petId/visits]
export function getPetVisits(ownerId, petId) {
  return this.getRequest(`/owners/${ownerId}/pets/${petId}/visits`);
}

export function createPetVisit(ownerId, petId, visit) {
  return this.modifyRequest(`/owners/${ownerId}/pets/${petId}/visits`, 'POST', visit);
}

// Vets - [/vets]
export function getVets() {
  return this.getRequest('/vets');
}

// PetTypes - [pettypes]
export function getPetTypes() {
  return this.getRequest('/pettypes');
}