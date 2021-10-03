# Todo-sql

This actor implements the [Todo backend spec](https://github.com/TodoBackend/todo-backend-js-spec/blob/master/js/specs.js).

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability (with TLS enabled),
the relational database capability ('wasmcloud:sqldb') 
the number generator capability ('wasmcloud:builtin:numbergen')
and the logging capability (`wasmcloud:logging`). 
As usual, it is worth noting that this actor does _not_ know where its HTTP server comes from,
nor does it know which relational database implementation the host runtime has provided.

## To generate self-signed TLS certificates

The configuration file has TLS enabled, so you will need to generate self-signed TLS certificates,
if you don't already have certificates to use.
- install mkcert from https://github.com/FiloSottile/mkcert. Read more about it on that page.
- run `mkcert example.com "*.example.com" example.test localhost 127.0.0.1 ::1` 
  - this command generates 'example.com+5.pem' and 'example.com+5-key.pem'
- edit http-linkdef.json and set the absolute paths to these two files for `cert_file` and `priv_key_file`, respectively
  - `cert_file: "/path/to/example.com+5.pem"`
  - `priv_key_file: "/path/to/example.com+5-key.pem"`
 
## to run

- If you want to change from the default https port 9000, edit http-linkdef.json to change the port in the address line and also in the CORS origins
- set up postgres database with a user that has write access (can create tables, insert, and select)
- edit sql-linkdef.json and update the uri field with the credentials and host/port
- make sure nats and the wasmcloud host are running
- run `make clean-start` to push the actor, start the providers, and link neverything
- Open your browser to https://localhost:9000/", and accept the warning from the browser that the server is using a self-signed certificate.

## to run the todo test suite (from www.todobackend.com)

- visit "https://localhost:9000/test/" to view the todo test page and press the green button"