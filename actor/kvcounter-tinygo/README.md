# kvcounter-tinygo

This actor accepts http GET requests, and 
increments a counter under the key `tinygo:count`.
The result is returned in a plaintext payload as follows:

```plain
Count: 12
```

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability 
and the key-value store capability (`wasmcloud:keyvalue`). 

As usual, it is worth noting that this actor does _not_ know 
where its HTTP server comes from, nor does it know which 
key-value implementation the host runtime has provided.

## Building
To build this actor and sign the WebAssembly file, run `make`.

Make sure the wasmcloud host is running (and a registry and a nats
server) as described in [Getting
Started](https://wasmcloud.dev/overview/installation/)

Start an http server provider and link it to your actor.
This only needs to be done once, even if you test multiple iterations of
your actor.
```
make provider
make link
```

To run your actor, issue the following commands
```
# push the signed wasm to your OCI registry
make push
# start the actor
make start
```

To test it,
```
curl -v localhost:8085
```
It should print the response "Count: 1" on your console.

