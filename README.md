# wasmCloud Examples

Example actors, capability providers, and other demonstrations

| Example | Type | Description | OCI Reference (refer to example for latest version) |
|---|---|---|---|
| [echo](https://github.com/wasmcloud/examples/tree/main/echo) | Actor |An actor that returns a JSON payload describing the incoming request | wasmcloud.azurecr.io/echo:0.2.1 |
| [extras](https://github.com/wasmcloud/examples/tree/main/extras) | Actor | A sample illustrating the use of the `wasmcloud:extras` capability for random number, Guid, and sequence number generation. | wasmcloud.azurecr.io/extras:0.2.1 |
| [kvcounter](https://github.com/wasmcloud/examples/tree/main/kvcounter) | Actor | An actor that uses the key-value store to increment a counter and return a value for every HTTP request it receives | wasmcloud.azurecr.io/kvcounter:0.2.0 |
| [kvcounter-as](https://github.com/wasmcloud/examples/tree/main/kvcounter-as) | Actor | The same actor as `kvcounter`, but written in AssemblyScript. This actor is meant to demonstrate the subtle differences between languages.  | wasmcloud.azurecr.io/kvcounter-as:0.1.0 |
| [logger](https://github.com/wasmcloud/examples/tree/main/logger) | Actor | A simple actor that logs every HTTP Request Method it receives to `stdout` | wasmcloud.azurecr.io/logger:0.1.0 |
| [subscriber](https://github.com/wasmcloud/examples/tree/main/subscriber) | Actor | A simple actor that logs every message it receives to `stdout` | wasmcloud.azurecr.io/subscriber:0.2.0 |
| [actor-to-actor](https://github.com/wasmcloud/examples/tree/main/actor-to-actor) | Actors | An example illustrating shared actor interface and actor-to-actor communication | Not Published |
| [inmemory-keyvalue](https://github.com/wasmcloud/examples/tree/main/inmemory-keyvalue) | Provider | A sample in-memory Key-Value Store capability provider, used by the tutorial for creating a new capability provider | wasmcloud.azurecr.io/inmemory-keyvalue:0.4.0 |

Please refer to the GitHub Release versions for the most up-to-date versions of the example actors.
