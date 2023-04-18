[![](https://ghcr-badge.egpl.dev/jclmnop/wasmcloud-provider-timing/latest_tag?trim=major&label=ghcr.io%2Fjclmnop%2Fwasmcloud-provider-timing%3A0.1.0)](ghcr.io/jclmnop/wasmcloud-provider-timing:0.1.0)

# Timing Capability Provider

This third-party capability provider implements the
["wasmcloud:timing"](https://crates.io/crates/wasmcloud-interface-timing/0.1.0) 
capability contract and allows actors to sleep for a specified duration.

Useful for implementing rate limits or other time-based functionality.

Build with 'make'. Test with 'make test'.