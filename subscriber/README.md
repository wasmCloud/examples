# Message Broker Subscriber
This actor simply dumps any message it receives to the console via the `ctx.log()` function. Note that nowhwere _inside_ the actor is there any information on the subscription. All subscription configuration is done by configuring the host runtime.
