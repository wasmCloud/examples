# TODO

This actor implements the [Todo backend spec](https://github.com/TodoBackend/todo-backend-js-spec/blob/master/js/specs.js).

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability, the key-value store capability (`wasmcloud:keyvalue`) and the logging capability (`wasmcloud:logging`). As usual, it is worth noting that this actor does _not_ know where its HTTP server comes from, nor does it know which key-value implementation the host runtime has provided.
