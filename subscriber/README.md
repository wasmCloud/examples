# Message Broker Subscriber

This actor simply prints any message it receives to the console via the `wasmcloud:logging` capability.

Note that nowhwere _inside_ the actor is there any information on the subscription. All subscription configuration is done by configuring the host runtime via actor-capability link definitions.
