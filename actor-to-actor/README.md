# Actor-to-Actor Communications

Calling from one actor to another is relatively easy in wasmcloud. You can either use the public key of the actor as the target of your invocation, or you can use a _call alias_. The call alias allows you to refer to a developer-or-human-friendly alias that doesn't change even if the signing key used for the module does.

To test this example, you can load the `pinger` and `ponger` actors and link the `pinger` actor to an HTTP server capability provider. With that in place, any request you trigger over HTTP will perform an actor-to-actor call from `pinger` to `ponger`, and return the value as serialized JSON over HTTP.
