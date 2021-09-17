# customers Actor

This project implements an actor that returns a greeting.

Upon receiving an http request, the actor returns "Hello World".
The response can be customized by adding an http query parameter 'name'.
For example, if the http server is running on localhost port 8000,
the command

```
curl "localhost:8000/?name=Alice"
```

returns "Hello Alice".

## The implementation

To respond to http requests, the actor must implement the
`httpResponse` method of the
[HttpServer interface](https://github.com/wasmCloud/interfaces/tree/main/httpserver) interface.

The implementation is in the file [src/lib.rs](./src/lib.rs)

## See it in action

- To compile the actor and generate a signed Webassembly module, type `make`.
- To load and start the actor you'll need to have a running OCI-compatible
registry. Check that `REG_URL` setting in Makefile is correct, and run
`make push` and `make start` to push the actor to the registry
and start the actor.
Alternately, you can load and start the actor from the host's web ui.
When prompted for the path, 
select `build/customers_s.wasm`.

The actor must be linked with an HttpServer capability 
provider with the contract id `wasmcloud:httpserver`. You can start the
provider (TODO: need registry url and more specific instructions here)

Your actor can be invoked from a terminal command-line or from a web browser.
The following examples assume the http server is listening on localhost port 8000.

### In a terminal

```
curl localhost:8000

curl "localhost:8000/?name=Alice"
```
(note the quotes in the second example)


### In a browser

visit the url "http://localhost:8000" or "http://localhost:8000/?name=Alice"


