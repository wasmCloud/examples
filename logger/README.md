# Logger Example Actor

Example actor that simply logs the HTTP request method to stdout at various log levels. This actor demonstrates direct use of the `write_log` method as well as using log level macros to write logs.

In order to use log level macros, it is important to place the following method call in the initialization section of the actor (e.g. `wapc_init`):
```rust
wasmcloud_actor_logging::enable_macros();
```