# wasmCloud Pet Clinic
The wasmCloud Pet Clinic example application is a WebAssembly and wasmCloud-based reimagining of the classic [Spring Boot microservices Pet Clinic](https://github.com/spring-petclinic/spring-petclinic-microservices) example.

## Application Architecture
The wasmCloud Pet Clinic consists of the following four [actors](./actors):
* [Clinic API](./actors/clinicapi/README.md) - The main JSON (aka "REST") API gateway for interacting with the service
* [Customers](./actors/customers/README.md) - Customers/Owners Actor - roughly equivalent to the original Customers/Owners microservice.
* [Vets](./actors/vets/README.md) - Veterinarians actor, roughly equivalent to the original Vets service.
* [Visits](./actors/visits/README.md) - Visits service, roughly equivalent to the original visits service.

The wasmCloud Pet Clinic application has a simple architecture: simply start all four of the actors, push a link definition between the **Clinic API** actor and an **HTTP Server** capability provider, and then push link definitions between the 3 remaining actors and a relational database. Lastly, ensure that both the HTTP Server and relational database capability providers are started.