# This actor demonstrates the random number generation capability
# in wasmcloud:builtin:numbergen

Build and start the actor with `make`, `make push`, `make start`.

To make the actor easy to test,
the actor implements the "Runner" interface, which can be
invoked with the `wash` cli. 

When the Run method is invoked with a list of strings,
the actor returns a random item from the list.

Some sample invocations are included in the Makefile. Try
`make pick-animal` or `make pick-a-card`

