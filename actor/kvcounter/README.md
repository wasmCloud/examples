# Key value counter

This actor accepts http GET requests, and 
increments a counter whose name is based on the url path.
Each unique url is associated a unique counter.
The result is returned in a JSON payload as follows:

```json
{
    "counter": 12
}
```

This actor makes use of the HTTP server (`wasmcloud:httpserver`) capability 
and the key-value store capability (`wasmcloud:keyvalue`). 

As usual, it is worth noting that this actor does _not_ know 
where its HTTP server comes from, nor does it know which 
key-value implementation the host runtime has provided.
