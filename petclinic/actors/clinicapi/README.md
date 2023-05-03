# Clinic API Actor

The **Clinic API** actor is an API gateway actor reimagined from the original Spring Cloud microservices Pet Clinic example.

## JSON API ("REST" interface)

The following is the JSON API for this service:

| Resource                         | Method | Description                                           |
| :------------------------------- | :----: | :---------------------------------------------------- |
| `petttypes`                      |  GET   | Retrieves a list of all pet types                     |
| `vets`                           |  GET   | Retrieves the list of all veterinarians in the system |
| `owners`                         |  POST  | Creates a new customer (pet owner)                    |
| `owners`                         |  GET   | Retrieves a list of all customers (pet owners)        |
| `owners/{id}`                    |  PUT   | Updates an owner                                      |
| `owners/{id}`                    |  GET   | Gets a specific customer/owner                        |
| `owners/{oid}/pets`              |  GET   | Gets pets owned by a customer                         |
| `owners/{oid}/pets`              |  POST  | Adds a new pet to an owner                            |
| `owners/{oid}/pets/{pid}`        | DELETE | Deletes a pet from an owner                           |
| `owners/{oid}/pets/{pid}`        |  PUT   | Updates an existing owner's pet                       |
| `owners/{oid}/pets/{pid}`        |  GET   | Gets a specific pet belonging to an owner             |
| `owners/{oid}/pets/{pid}/visits` |  GET   | Retrieves list of visits for a given owner's pet      |
| `owners/{oid}/pets/{pid}/visits` |  POST  | Records a new visit for an owner's pet                |

For the full definition of all data types used in these operations, check out the Smithy files in the [Pet Clinic Interface](../../petclinic-interface) repo.

## Build and Run

- To compile the actor and generate a signed Webassembly module, type `make`.
- To load and start the actor you'll need to have a running OCI-compatible
  registry. Check that `REG_URL` setting in Makefile is correct, and run
  `make push` and `make start` to push the actor to the registry
  and start the actor.
  Alternately, you can load and start the actor from the host's web ui.
  When prompted for the path,
  select `build/clinicapi_s.wasm`.

The actor must be linked with an HttpServer capability
provider with the contract id `wasmcloud:httpserver` and signed with `wasmcloud:builtin:logging`. You can start the
provider via the `wasmcloud.azurecr.io/httpserver:0.17.0` OCI URL.
