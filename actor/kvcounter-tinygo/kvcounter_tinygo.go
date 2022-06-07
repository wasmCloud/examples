package main

import (
	"strconv"
	"strings"

	actor "github.com/wasmcloud/actor-tinygo"
	httpserver "github.com/wasmcloud/interfaces/httpserver/tinygo"
	keyvalue "github.com/wasmcloud/interfaces/keyvalue/tinygo"
)

func main() {
	actor.RegisterHandlers(httpserver.HttpServerHandler(&KvcounterTinygo{}))
}

type KvcounterTinygo struct{}

func (e *KvcounterTinygo) HandleRequest(
	ctx *actor.Context,
	req httpserver.HttpRequest) (*httpserver.HttpResponse, error) {

	key := strings.Replace(req.Path, "/", "_", -1)

	kv := keyvalue.NewProviderKeyValue()

	count, err := kv.Increment(ctx, keyvalue.IncrementRequest{
		Key: key, Value: 1,
	})
	if err != nil {
		return InternalServerError(err), nil
	}

	res := "{\"counter\": " + strconv.Itoa(int(count)) + "}"

	r := httpserver.HttpResponse{
		StatusCode: 200,
		Header:     make(httpserver.HeaderMap, 0),
		Body:       []byte(res),
	}
	return &r, nil
}

func InternalServerError(err error) *httpserver.HttpResponse {
	return &httpserver.HttpResponse{
		StatusCode: 500,
		Header:     make(httpserver.HeaderMap, 0),
		Body:       []byte(err.Error()),
	}
}
