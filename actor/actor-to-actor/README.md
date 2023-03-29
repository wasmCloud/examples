# Actor to Actor calls

This example shows an actor `pinger` that accepts HTTP requests over the [wasmcloud:httpserver](https://crates.io/crates/wasmcloud-interface-httpserver) contract and makes an actor-to-actor call to the `ponger` actor that returns a simple String. The syntax that you see for `PingpongSender::to_actor` can be used for any operation where `actorReceive: true` is set in the interface file, you don't need to have a custom interface for each actor-to-actor call.

## Running this example

You'll need [wash](https://wasmcloud.com/docs/installation) installed to run this.

```
wash up -d
wash ctl start actor wasmcloud.azurecr.io/pinger:0.1.0
wash ctl start actor wasmcloud.azurecr.io/ponger:0.1.0

# Invoke the pinger actor's HTTP handler directly
wash call MBN2KUUZP4Y2F7IRXEPLV232YCZFKGSWYIQ3DNPDIERBE4BHPMUC5BUX HttpServer.HandleRequest '{"method": "GET", "path": "/", "body": "", "queryString":"","header":{}}'
```
