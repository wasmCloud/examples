# Xkcd comic generator

This example actor demonstrates three capability providers:
- wasmcloud:httpserver  for responding to http requests
- wasmcloud:httpclient  for fetching http resources from the internet
- wasmcloud:builtin:numbergen for generating a random number.

When the actor runs, open a web browser to the http port
(selected when linking to the httpserver)

Each time you refresh the page, you should get a random xkcd comic!

To use this actor:
_These instructions will be simpler when the providers
are published to a public registry_

```shell
# start wasmcloud host, nats, and a registry
# (use the docker-compose.yml file from the sdk)
# docker-compose up -d

# compile and generate signed wasm file for this actor
make
# push to local registry and start the actor
make push 
make start

# install and start httpserver provider
cd path/to/capability-providers/httpserver-rs
make 
make push
make start
make inspect
# get the provider id (starting with 'V') and paste it into the Makefile for HTTPSERVER_PROVIDER_ID

# install and start httpclient provider
cd path/to/capability-providers/httpclient
make 
make push
make start
make inspect
# get the provider id (starting with 'V') and paste it into the Makefile for HTTPCLIENT_PROVIDER_ID

# you should now be able to link them
make link

```

Once all providers and the actor are started and linked,
you should be able to open a web browser to localhost:8080
to view the comic. Refresh the page to see another one.
