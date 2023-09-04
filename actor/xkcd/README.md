# Xkcd comic generator

This example actor demonstrates three capability providers:

- wasmcloud:httpserver for responding to http requests
- wasmcloud:httpclient for fetching http resources from the internet
- wasmcloud:builtin:numbergen for generating a random number.

## Prerequisites

[Install wash](https://wasmcloud.com/docs/installation)

## Running this example

```shell
wash up -d
wash app deploy ./wadm.yaml
```

Once all providers and the actor are started and linked,
you should be able to open a web browser to localhost:8080
to view the comic. Refresh the page to see another one.
