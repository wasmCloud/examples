# Key-Value Capability Provider

This is a tutorial capability provider. It supplies an implementation for the `wasmcloud:keyvalue` capability ID and can be used as a testing alternative for the **Redis** capability provider. The usual caveats should apply - this provider's data only lasts as long as the host runtime, so it should only be used for isolated testing and experimentation, and obviously won't provide distributed cached data in production.

This provider was created using the `new-provider-template` cargo generation template as a starter.
