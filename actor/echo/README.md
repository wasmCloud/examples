# A simple http echo server

This example demonstrates the actor code generation and
a handler for the wasmcloud:httpserver capability contract.

For each http request, the actor returns a json-formatted
string containing fields from the request.

# Running this example

Run your wasmCloud host using `wash up -d`, then:

```shell
wash ctl start actor wasmcloud.azurecr.io/echo:0.3.7
wash ctl start provider wasmcloud.azurecr.io/httpserver:0.17.0
wash ctl link put MBCFOPM6JW2APJLXJD3Z5O4CN7CPYJ2B4FTKLJUR5YR5MITIU7HD3WD5 VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M wasmcloud:httpserver address=0.0.0.0:8080
```
