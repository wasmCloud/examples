# Message Pub Actor

This actor receives HTTP requests and publishes the request body on a topic without waiting for a reply. This can be used to transform messages sent over HTTP to a message broker pub (if no reply is needed) and also serves as an example for how you can send messages using the messaging capability.

## Required Capability Claims

1. `wasmcloud:httpserver` to receive HTTP requests
1. `wasmcloud:messaging` to send a message on a message broker

## Running this actor

Start a wasmCloud host as detailed in the [Installation Guide](https://wasmcloud.dev/overview/installation/), build your actor using `make`, then start your actor using the `From File` option in the wasmCloud dashboard. The wasmCloud [HTTP Server](https://github.com/wasmCloud/capability-providers/tree/main/httpserver-rs) and [NATS messaging](https://github.com/wasmCloud/capability-providers/tree/main/nats) providers are first-party resources that fulfill the above contracts, but you're free to use your own as well. Simply start those providers and link them to your actor, giving the HTTP server link value something like `address=0.0.0.0:8080`, then you can use the [NATS CLI](https://github.com/nats-io/natscli) to listen for messages while making requests to `localhost:8080`.

Once you've installed **wash** and ran wasmCloud after following the [installation guide](https://wasmcloud.dev/overview/installation/), you can run this example actor and the wasmCloud providers with the following commands:

```
wash ctl start actor wasmcloud.azurecr.io/message-pub:0.1.3
# If you use a locally build actor, replace the actor ID below with your own
wash ctl link put MC3QONHYH3FY4KYFCOSVJWIDJG4WA2PVD6FHKR7FFT457GVUTZJYR2TJ VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M wasmcloud:httpserver address=0.0.0.0:8080
wash ctl link put MC3QONHYH3FY4KYFCOSVJWIDJG4WA2PVD6FHKR7FFT457GVUTZJYR2TJ VADNMSIML2XGO2X4TPIONTIC55R2UUQGPPDZPAVSC2QD7E76CR77SPW7 wasmcloud:messaging
wash ctl start provider wasmcloud.azurecr.io/httpserver:0.17.0 --skip-wait
wash ctl start provider wasmcloud.azurecr.io/nats_messaging:0.17.0 --skip-wait
```

And you can subscribe for messages with:

```
nats sub "wasmcloud.http.>"
```
