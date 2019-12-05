# K/V Counter
This actor accepts `GET` requests and for each URL it gets, it will increment a call counter for that URL and return the result in a JSON payload as follows:

```json
{
    "count": 12
}
```

This actor makes use of the HTTP server (`wascc:http_server`) capability and the key-value store capability (`wascc:keyvalue`). As usual, it is worth noting that this actor does _not_ know where its HTTP server comes from, nor does it know which key-value implementation the host runtime has provided.
