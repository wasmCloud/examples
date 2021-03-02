# Ponger

The ponger actor should be signed with the call alias `wasmcloud/examples/ponger`. This alias allows this actor to be invoked by this name even if its public key were to change (which can happen if you use different signing keys in production than in development).

In response to the `Ping` operation, the ponger will produce a `Pong` struct and send it back to the caller, illustrating actor-to-actor comms.
