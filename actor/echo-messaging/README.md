# An actor that echoes messages received

This actor subscribes to a topic
using the wasmcloud:messaging provider and replies
to every message, echoing it back.

Ensure that the nats capability provider has been started.
To run the actor, use `wash build` to compile it and then start the actor from `build/echo_messaging_s.wasm`

Then using the nats-cli command, type

`nats req demo.echo hello`

You should get back "reply: hello"
