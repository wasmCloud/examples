# A "Hello World" actor

This example is a simple wasmcloud actor that returns a greeting.

Upon receiving an http request, the actor returns "Hello World".
The response can be customized by adding an http query parameter 'name'.
For example, if the http server is running on localhost port 8000,
the command

```curl "localhost:8000/?name=Alice"```

returns "Hello Alice".

## The implementation

To respond to http requests, the actor must implement the
`httpResponse` method of the
[HttpServer interface](https://github.com/wasmCloud/interfaces/tree/main/httpserver) interface.

The implementation is in the file [src/lib.rs](./src/lib.rs)

## See it in action

- To compile the actor and generate a signed Webassembly module, type `make`.
- Start the actor: You can load and start the actor using the host's Web UI. When prompted, select the path to the compiled actor (`build/hello_s.wasm`).
 
The actor must be linked with an HttpServer capability 
provider with the contract id `wasmcloud:httpserver`. You can start the
provider (TODO: need registry url and more specific instructions here)

Your actor can be invoked from a terminal command-line or from a web browser. The following examples assume the http server is listening on localhost port 8000.

### In a terminal


```
curl localhost:8000

curl "localhost:8000/?name=Alice"
```
(note the quotes in the second example)


### In a browser

visit the url "http://localhost:8000" or "http://localhost:8000/?name=Alice"


