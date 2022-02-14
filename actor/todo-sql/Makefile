# examples/actor/todo-sql

PROJECT  = todo_sql
VERSION  = $(shell cargo metadata --no-deps --format-version 1 | jq -r '.packages[] .version' | head -1)
REVISION = 0
# list of all contract claims for actor signing (space-separated)
CLAIMS   = wasmcloud:httpserver wasmcloud:sqldb \
			wasmcloud:builtin:logging wasmcloud:builtin:numbergen
# registry url for our actor
REG_URL  = localhost:5000/v2/$(PROJECT):$(REVISION)
# command to upload to registry (without last wasm parameter)
PUSH_REG_CMD = wash reg push --insecure $(REG_URL)
ACTOR_NAME = "Todo Sql"

include ../../build/makefiles/actor.mk

HTTPSERVER_REF = wasmcloud.azurecr.io/httpserver:0.14.5
SQLDB_REF      = wasmcloud.azurecr.io/sqldb-postgres:0.1.0
HTTPSERVER_ID  = VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M
SQLDB_POSTGRES_ID = VDJQVOMF5FI3S5P4KJLCK2N25525U4IQIPH6NLHWZVTRZXTU6AOX4OPN

start-providers:
	wash ctl start provider $(HTTPSERVER_REF) --link-name default \
		--host-id $(shell $(WASH) ctl get hosts -o json | jq -r ".hosts[0].id")
	wash ctl start provider $(SQLDB_REF) --link-name default \
		--host-id $(shell $(WASH) ctl get hosts -o json | jq -r ".hosts[0].id")

link-providers:
	wash ctl link put $(shell make --silent actor_id) \
		$(HTTPSERVER_ID) wasmcloud:httpserver config_b64=$(shell base64 -w0 http-linkdef.json)
	wash ctl link put $(shell make --silent actor_id) \
		$(SQLDB_POSTGRES_ID) wasmcloud:sqldb config_b64=$(shell base64 -w0 sql-linkdef.json)

clean-start:
	# if pushing to local server, make sure the host is started with
	# WASMCLOUD_OCI_ALLOWED_INSECURE=localhost:5000
	-killall wasmcloud_httpserver_default
	-killall wasmcloud_sqldb_default
	# check syntax of json files
	jq < http-linkdef.json >/dev/null
	jq < sql-linkdef.json >/dev/null
	# ensure actor is built and signed
	$(MAKE)
	# push actor to local registry
	$(MAKE) push
	sleep 1
	# start actor and providers, and link
	$(MAKE) start
	$(MAKE) start-providers
	$(MAKE) link-providers
