# Vets Actor

This actor plays the role of the Vets microservice, owning and encapsulating all data pertaining to veterinarians and their specialties.

Like other data service actors, this actor only needs to be signed with the relational database contract ID.

## Build and Run

- To compile the actor and generate a signed Webassembly module, type `make`.
- To load and start the actor you'll need to have a running OCI-compatible
registry. Check that `REG_URL` setting in Makefile is correct, and run
`make push` and `make start` to push the actor to the registry
and start the actor.
Alternately, you can load and start the actor from the host's web ui.
When prompted for the path, 
select `build/vets_s.wasm`.
