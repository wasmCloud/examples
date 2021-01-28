# Echo Example

This example is designed to illustrate a few simple aspects of coding an actor module with wasmCloud. The first shows how to respond to simple HTTP requests, but this sample also shows how to use `serde` for JSON serialization.

This actor module responds to all incoming HTTP requests by returning a JSON object describing the inbound request.
