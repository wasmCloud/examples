# Customers Actor
The Customers actor plays the role of the customers/owners microservice in this example, reacting to service operations defined by a shared interface and accessing a relational database during the fulfillment of requests.

This service owns and encapsulates data corresponding to human customers (pet owners) as well as a list of known pet types.

## Build and Run

- To compile the actor and generate a signed Webassembly module, type `make`.
- To load and start the actor you'll need to have a running OCI-compatible
registry. Check that `REG_URL` setting in Makefile is correct, and run
`make push` and `make start` to push the actor to the registry
and start the actor.
Alternately, you can load and start the actor from the host's web ui.
When prompted for the path, 
select `build/customers_s.wasm`.

