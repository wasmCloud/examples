# Todo-sql

This actor implements the [Todo backend spec](https://github.com/TodoBackend/todo-backend-js-spec/blob/master/js/specs.js).

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability,
the relational database capability ('wasmcloud:sqldb') 
the number generator capability ('wasmcloud:builtin:numbergen')
and the logging capability (`wasmcloud:logging`). 
As usual, it is worth noting that this actor does _not_ know where its HTTP server comes from,
nor does it know which relational database implementation the host runtime has provided.


## to run,
- set up postgres database with a user that has write access (can create tables, insert, and select)
- edit sql-linkdef.json and update the uri field with the credentials and host/port
```shell
# start the actor
make push && make start
# start http server and link to it
make start-http && make link-http
# start sqldb server and link to it
# (in sql provider folder)
#    make push && make start
make inventory
# get the sqldb provider id from the above command 
# and set it in Makefile as SQLDB_ID
make link-sqldb
```
- start the todo actor (`make push && make start`)
- start the httpserver provider (`make start-http`)
- link to httpserver provider (') 

