# Examples
Example actors, capability providers, and other demonstrations

* [echo](https://github.com/wascc/examples/tree/master/echo) - An actor that returns a JSON payload describing the incoming request
* [subscriber](https://github.com/wascc/examples/tree/master/subscriber) - A simple actor that logs every message it receives to `stdout`
* [kvcounter](https://github.com/wascc/examples/tree/master/kvcounter) - An actor that uses the key-value store to increment a counter and return a value for every HTTP request it receives
* [keyvalue-provider](https://github.com/wascc/examples/tree/master/keyvalue-provider) - A sample in-memory Key-Value Store capability provider, used by the tutorial for creating a new capability provider
* [inmemory-streams](https://github.com/wascc/examples/tree/master/inmemory-streams) - An example illustrating an in-memory `wascc:eventstreams` provider
* [extras](https://github.com/wascc/examples/tree/master/extras) - A sample illustrating the use of the `wascc:extras` capability for random number, Guid, and sequence number generation.

To run these, go to the root directory of the [wascc-host](https://github.com/wascc/wascc-host) project and issue the following command:

```shell
$ cargo run --example (example name)
```

You'll see the examples in the [examples](https://github.com/wascc/wascc-host/tree/master/examples) directory.
