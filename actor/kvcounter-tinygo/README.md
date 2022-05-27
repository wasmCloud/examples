# kvcounter-tinygo

This actor takes an incoming HTTP request and returns a simple HTTP response. 
This is very similar to the echo example in the [tinygo actor sdk](https://github.com/wasmcloud/actor-tinygo).
Feel free to experiment with the code to see how you can change the HTTP response easily.

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
curl -v localhost:8085/abc
```
It should print the response "hello" on your console.

