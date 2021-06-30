# TodoMVC

This actor implements the [TodoMVC backend spec](https://github.com/TodoBackend/todo-backend-js-spec/blob/master/js/specs.js).

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability and the key-value store capability (`wasmcloud:keyvalue`). As usual, it is worth noting that this actor does _not_ know where its HTTP server comes from, nor does it know which key-value implementation the host runtime has provided.

## Development TODOs

- [ ] get it compiling under the new name
  - [ ] manifest.yaml needs label name, exe path and actor id changing
- [ ] Implement spec
- [ ] ...
- [ ] Profit
