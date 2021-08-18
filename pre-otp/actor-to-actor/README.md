# Actor-to-Actor Communications

Calling from one actor to another is relatively easy in wasmcloud. You can either use the public key of the actor as the target of your invocation, or you can use a _call alias_. The call alias allows you to refer to a developer-or-human-friendly alias that doesn't change even if the signing key used for the module does. This sample illustrates using a call alias for actor-to-actor calls.

## Running

To run this, just type `make run` and it will compile and sign your actors then start the actors and
capability provider pre-linked and all you'll have to do is curl `localhost:8080` to watch it in action.

If you want to do it manually, you can build `pinger` and `ponger` independently, sign them, and then
start a wasmcloud host with the included manifest.