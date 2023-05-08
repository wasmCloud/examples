module github.com/wasmcloud/actor-tinygo/example

go 1.17

require (
	github.com/wasmcloud/actor-tinygo v0.1.4
	github.com/wasmcloud/examples/cron/interface/tinygo v0.0.0-00010101000000-000000000000
	github.com/wasmcloud/interfaces/logging/tinygo v0.0.0-20230426203856-205450d2ad7b
)

require (
	github.com/wasmcloud/tinygo-cbor v0.1.0 // indirect
	github.com/wasmcloud/tinygo-msgpack v0.1.4 // indirect
)

replace github.com/wasmcloud/examples/cron/interface/tinygo => ../interface/tinygo/
