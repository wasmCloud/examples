# Echo Example

This example is designed to illustrate a few simple aspects of coding an actor module with waSCC. The first shows how to respond to simple HTTP requests, but this sample also shows how to use `serde` for JSON serialization.

This actor module responds to all incoming HTTP requests by returning a JSON object describing the inbound request.

You can build this example with a simple `cargo build`. In order to run it within a host runtime, you will need to sign it with the [wascap](https://github.com/wascc/wascap) tool (which uses the [nkeys](https://github.com/encabulators/nkeys) command-line tool to generate keys). You can also copy sample keys from the [starter template](https://github.com/wascc/new-actor-template).

To see this sample run inside of a waSCC host, take a look at the **examples** directory in the [wascc-host](https://github.com/wascc/wascc-host) repository.

There is also a [tutorial](https://wascc-dev.netlify.com/tutorials/first-actor/) that covers creating this sample and running it in a host runtime.