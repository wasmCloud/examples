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

include ../../build/makefiles/actor.mk

HTTPSERVER_REF = wasmcloud.azurecr.io/httpserver:0.14.5
HTTPSERVER_ID  = VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M
SQLDB_ID       = VB2EUDLLFPNZXERK7IMM4XZEX73A27HJL66BO6YBNW6SXMQPY6NEUFNU

start-http:
	wash ctl start provider $(HTTPSERVER_REF) --link-name default \
		--host-id $(shell $(WASH) ctl get hosts -o json | jq -r ".hosts[0].id")

link-http:
	wash ctl link put $(shell make --silent actor_id) \
		$(HTTPSERVER_ID) PORT=9000

link-sqldb:
	wash ctl link put $(shell make --silent actor_id) \
		$(SQLDB_ID) $(shell base64 -w0 sql-linkdef.json)
