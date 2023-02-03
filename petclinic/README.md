# wasmCloud Pet Clinic
The wasmCloud Pet Clinic example application is a WebAssembly and wasmCloud-based reimagining of the classic [Spring Boot microservices Pet
Clinic](https://github.com/spring-petclinic/spring-petclinic-microservices) example.

The purpose of this README is to help you learn the wasmCloud application runtime. We took great care to describe how we designed the application and how you can extend it. We paid special attention to design concepts and vocabulary. Post questions or concerns about any aspect of this application here... We want to hear your feedback.

## Application Architecture
The wasmCloud Pet Clinic application has the following simple architecture: 
* Start all five of the actors
* Push a link definition between the **Clinic API** actor and an **HTTP Server** capability provider via the **wasmcloud:httpserver** contract
* Push link definitions between the **Customers**, **Vets**, and **Visits** actors and the **postgres** capability provider via the **wasmcloud:sqldb** contract. 
* Ensure that both the HTTP Server and relational database capability providers are
started.

The wasmCloud Pet Clinic consists of the following five [actors](./actors):
* [Clinic API](./actors/clinicapi/README.md) - The main JSON (aka "REST") API gateway for
  interacting with the service
* [Customers](./actors/customers/README.md) - Customers/Owners Actor - roughly equivalent to the
  original Customers/Owners microservice.
* [Vets](./actors/vets/README.md) - Veterinarians actor, roughly equivalent to the original Vets
  service.
* [Visits](./actors/visits/README.md) - Visits service, roughly equivalent to the original visits
  service.
* [UI](./actors/ui/README.md) - An actor that contains all assets for the petclinic UI. This actor
  will serve the assets through the Clinic API

The following two capabilities support the wasmCloud Pet Clinic:
* [httpserver](https://github.com/wasmCloud/capability-providers/tree/main/httpserver-rs) - Capability provider implements the wasmcloud:httpserver capability contract, and enables an actor to accept incoming HTTP(s) requests.
* [postgres](https://github.com/wasmCloud/capability-providers/tree/main/sqldb-postgres) - Capability provider allows wasmCloud actors to use a Postgres-compatible database, and implements the "wasmcloud:sqldb" capability contract. 

## Code Review - Where to Start
Review the following files to better understand how the project fits together.
* [wadm.yaml](./wadm.yaml) - See the list of actors and capabilities defined.
* [run.sh](./run.sh) - Constructs and deploys the needed artifacts to run this example. This file is especially interesting for those with devopps interests.
* [customers.smithy](./petclinic-interface/customers.smithy) - Defines the Customers actor using [Smithy IDL specification](https://wasmcloud.com/docs/interfaces/wasmcloud-smithy) to define Models, Data types,Structures, Services, Operations, and Documentation.
* [actors/customers/src/db.rs](./actors/customers/src/db.rs) - defines the database structures and functions (ACTION - needs better description)
* [actors/customers/src/lib.rs](./actors/customers/src/lib.rs) - defines the api function signatures and maps to the database functions/structs (ACTION - needs better description)
* [actors/clinicapi/src/lib.rs](./actors/clinicapi/src/lib.rs) - routes api calls into actor functions (ACTION - needs better description)

## Running the Petclinic

If you'd like a quick out of the box example of the petclinic, you can use the included `run.sh`
script to automatically spin up a running wasmcloud host, database, and all the actors. This does
require `docker` to be installed as all the dependencies are run as docker images. To start
everything, run:

```console
$ ./run.sh all
```

To cleanup everything when you are done:

```console
$ ./run.sh wipe
```

Once it is running, you should be able to access the PetClinic application on `localhost:8080`
