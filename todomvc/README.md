# TodoMVC

This actor implements the [TodoMVC backend spec](https://github.com/TodoBackend/todo-backend-js-spec/blob/master/js/specs.js).

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability and the key-value store capability (`wasmcloud:keyvalue`). As usual, it is worth noting that this actor does _not_ know where its HTTP server comes from, nor does it know which key-value implementation the host runtime has provided.

To run this, you will need redis:

```bash
docker run -p 6379:6379 --name todomvc-store -d redis
```

Then `make` and `wash claims inspect target/wasm32-unknown-unknown/debug/todomvc_s.wasm`

Then `export TODO_ACTOR=<Actor id (called Module in the above output)>`.

Then `wasmcloud -m manifest.yaml`.

## To test:

Add a todo with: `curl localhost:8082 -d '{"title": "xx"}'`

List todos with `curl localhost:8082`

For a real test, use the TodoBackend test suite:

```
git clone https://github.com/TodoBackend/todo-backend-js-spec
cd todo-backend-js-spec
pnpx live-server --proxy=/api:http://localhost:8082 --open='/?/api'
```

## Hacky "hot" reloader until I can think of a better one:

```
watchexec --no-ignore --verbose --restart --watch=target/wasm32-unknown-unknown/debug/todomvc_s.wasm -- wasmcloud -m manifest.yaml
```

and

```
cargo watch -c --shell 'make'
```

## Development TODOs

- [ ] Implement spec
- [ ] ...
- [ ] Profit
