# Stock Capability

This is an example of a _portable capability provider_, a WebAssembly module compiled to the `wasm32-wasi` target that assumes the presence of a WASI-compliant host and also utilizes the waSCC actor SDK. Another way of thinking about _portable capability provider_s is that they are _privileged actors_.

This actor responds to the `acme:stock!StockRequest` message by replying with a JSON payload indicating the in-stock quantity. The request is a JSON payload with a single field called `sku`, and the response is a JSON payload containing the `sku`, and `quantity`, and a `ships_within` friendly string.

This example illustrates a number of important concepts, including:
* Building a portable capability provider
* Handling non-protobuf payloads 