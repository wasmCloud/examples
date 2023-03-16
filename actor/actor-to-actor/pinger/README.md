# pinger Actor

This actor uses the [pingpong](../pingpong/) interface, consisting of a single `ping` function, to call the `ponger` actor by it's [call alias](https://wasmcloud.com/docs/app-dev/a2a/#identifying-actors). This illustrates the process of using an interface with an `actorReceive` operation with an actor-to-actor call.
