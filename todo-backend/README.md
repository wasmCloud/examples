# TODO Backend

This actor implements the [Todo backend spec](https://github.com/TodoBackend/todo-backend-js-spec/blob/master/js/specs.js).

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability, the key-value store capability (`wasmcloud:keyvalue`) and the logging capability (`wasmcloud:logging`). As usual, it is worth noting that this actor does _not_ know where its HTTP server comes from, nor does it know which key-value implementation the host runtime has provided.

## How to run

- To start off you will need redis:

```bash
docker run -p 6379:6379 --name todo-backend-store -d redis
```

- Run `make` and `wash claims inspect ./target/wasm32-unknown-unknown/debug/todo_backend_s.wasm --output json | jq .module -r` from which you can extract the `<Actor id>`.

- Run `export TODO_ACTOR=<Actor id (called Module in the above output)>`.

- Run `RUST_LOG=info wasmcloud -m manifest.yaml`. This will trace all `info` type logs from all the capability providers including `wasmcloud:logging`.

## How to run with a 'Hot' reloader

```sh
watchexec --no-ignore --verbose --restart --watch=target/wasm32-unknown-unknown/debug/todomvc_s.wasm -- wasmcloud -m manifest.yaml
```

and

```sh
cargo watch -c --shell 'make'
```

## How to test:

- Add a todo with: `curl localhost:8082/api -d '{"title": "xx"}'`

- List todos with `curl localhost:8082/api`

## Test with acceptance test suite

To run the tests against todo backend spec, use the TodoBackend test suite:

**Note: To be able to use `pnpx` follow [pnpm installation guide](https://pnpm.io/installation)**

```sh
git clone https://github.com/TodoBackend/todo-backend-js-spec
cd todo-backend-js-spec
pnpx -y live-server  --proxy=/api:http://localhost:8082/api --open='/?/api'
```

## Test with client

You can use the [Todo MVC client application](https://github.com/TodoBackend/todo-backend-client)

```sh
git clone https://github.com/TodoBackend/todo-backend-client.git
cd todo-backend-client
npm install && npm run build
pnpx -y live-server  --proxy=/api:http://localhost:8082/api --open='/?/api'
```
