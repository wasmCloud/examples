# wasmCloud Examples

Example actors, capability providers, and other demonstrations

## Actors

The following actors run as WebAssembly on wasmCloud hosts.

| Example                                                                                                  | Description                                                                                                         | OCI Reference <br/> (refer to example for latest version) |
|----------------------------------------------------------------------------------------------------------|---------------------------------------------------------------------------------------------------------------------|-----------------------------------------------------------|
| [animal-image-downloader](https://github.com/wasmcloud/examples/tree/main/actor/animal-image-downloader) | An actor that receives messages and makes HTTP requests, downloading a picture of an animal to a blobstore          | `wasmcloud.azurecr.io/animal-image-downloader`            |
| [echo](https://github.com/wasmcloud/examples/tree/main/actor/echo)                                       | An actor that listens on an HTTP port and returns a JSON payload describing the incoming request                    | `wasmcloud.azurecr.io/echo`                               |
| [echo-messaging](https://github.com/wasmcloud/examples/tree/main/actor/echo-messaging)                   | An actor that listens on a message broker topic and replies                                                         | `wasmcloud.azurecr.io/echo-messaging`                     |
| [hello](https://github.com/wasmcloud/examples/tree/main/actor/hello)                                     | Canonical "hello world" actor that listens on an HTTP port and replies with a greeting                              | `wasmcloud.azurecr.io/hello`                              |
| [kvcounter](https://github.com/wasmcloud/examples/tree/main/actor/kvcounter)                             | An actor that uses the key-value store to increment a counter and return a value for every HTTP request it receives | `wasmcloud.azurecr.io/kvcounter`                          |
| [logging](https://github.com/wasmcloud/examples/tree/main/actor/logging)                                 | An actor that demonstrates the builtin logging capability provider                                                  | `wasmcloud.azurecr.io/logging`                            |
| [message-pub](https://github.com/wasmcloud/examples/tree/main/actor/message-pub)                         | An actor that demonstrates receiving HTTP requests and publishing the request body as a message                     | `wasmcloud.azurecr.io/message-pub`                        |
| [random](https://github.com/wasmcloud/examples/tree/main/actor/random)                                   | An actor that demonstrates the builtin random number generation capability provider                                 | `wasmcloud.azurecr.io/random`                             |
| [todo-sql](https://github.com/wasmcloud/examples/tree/main/actor/todo-sql)                               | An todo-application using sql database, https server (with TLS), logging, and numbergen                             | (unpublished)                                             |
| [todo](https://github.com/wasmcloud/examples/tree/main/actor/todo)                                       | An todo-application using keyvalue store, http server, and logging                                                  | (unpublished)                                             |
| [xkcd](https://github.com/wasmcloud/examples/tree/main/actor/xkcd)                                       | A application that generates xkcd comics                                                                            | `wasmcloud.azurecr.io/xkcd`                               |
| [ifconfig](https://github.com/wasmcloud/examples/tree/main/actor/ifconfig)                               | A tinygo actor that will return the external IP address of your http client                                         | (unpublished)                                             |


## Interfaces

The following example interfaces are defined by [Smithy](https://awslabs.github.io/smithy/) models.

| Example | Description | Capability contract | Rust crate |
| --- | --- | --- | --- |
| [payments](https://github.com/wasmcloud/examples/tree/main/interface/payments) | A simple interface for a payments capability provider (used in the [Creating an Interface](https://wasmcloud.dev/app-dev/create-provider/new-interface/) tutorial) |  `wasmcloud:example:payments` | `wasmcloud-examples-payments` |
| [runner](https://github.com/wasmcloud/examples/tree/main/interface/runner) | A simple interface with a single 'Run' method |  `wasmcloud:example:runner` | `wasmcloud-examples-runner` |


## Capability providers

Providers of capabilities for wasmCloud actors

| Example | Description | Capability contract | OCI Reference |
| --- | --- | --- | --- |
| [factorial](https://github.com/wasmcloud/examples/tree/main/provider/factorial) | A capability provider that computes factorial of a number |  `wasmcloud:example:payments` | `wasmcloud.azurecr.io/factorial` |
| [fakepay](https://github.com/wasmcloud/examples/tree/main/provider/fakepay) | A simple payment provider, used in the [Creating a capability provider](https://wasmcloud.dev/app-dev/create-provider/) tutorial |  `wasmcloud:example:fakepay` | `wasmcloud.azurecr.io/fakepay` |


## Applications

| Example | Description | 
| --- | --- | 
| [petclinic](https://github.com/wasmcloud/examples/tree/main/petclinic) |  A WebAssembly and wasmCloud-based reimagining of the classic [Spring Boot microservices Pet Clinic](https://github.com/spring-petclinic/spring-petclinic-microservices). The wasmCloud Pet Clinic consists of multiple actors, and uses a relational database capability provider and an http server capability provider. |
| [adsb](https://github.com/wasmcloud/examples/tree/main/adsb) |  A FlightAware clone that allows users to take an [RTL-SDR](https://www.rtl-sdr.com/) and plot airplane data on a map. Users can also cluster many RTL-SDRs from all over the country and visualize them together utilizing the power of the wasmCloud lattice.  |

