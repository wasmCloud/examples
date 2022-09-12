# nats-pub Actor

This actor receives HTTP requests and publishes the request body on a topic without waiting for a reply. This can be used to transform messages sent over HTTP to a message broker pub (if no reply is needed) and also serves as an example for how you can send messages using the messaging capability.

## Required Capability Claims
1. `wasmcloud:httpserver` to receive HTTP requests
1. `wasmcloud:messaging` to send a message on a message broker

## Running this actor
Start a wasmCloud host as detailed in the [Installation Guide](https://wasmcloud.dev/overview/installation/), build your actor using `make`, then start your actor using the `From File` option in the wasmCloud dashboard. The wasmCloud [HTTP Server](https://github.com/wasmCloud/capability-providers/tree/main/httpserver-rs) and [NATS messaging](https://github.com/wasmCloud/capability-providers/tree/main/nats) providers are first-party resources that fulfill the above contracts, but you're free to use your own as well. Simply start those providers and link them to your actor, giving the HTTP server link value something like `PORT=8080`, then you can use the [NATS CLI](https://github.com/nats-io/natscli) to listen for messages while making requests to `localhost:8080`.