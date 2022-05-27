package main

import (
	"strconv"

	"github.com/wasmcloud/actor-tinygo"
	httpserver "github.com/wasmcloud/interfaces/httpserver/tinygo"
	keyvalue "github.com/wasmcloud/interfaces/keyvalue/tinygo"
)

func main() {
	actor.RegisterHandlers(httpserver.HttpServerHandler(&KvcounterTinygo{}))
}

type KvcounterTinygo struct{}

func (e *KvcounterTinygo) HandleRequest(ctx *actor.Context, req httpserver.HttpRequest) (*httpserver.HttpResponse, error) {
	kv := keyvalue.NewProviderKeyValue()

	prev, err := kv.Get(ctx, "tinygo:count")
	if err != nil {
		return Failure("Couldn't query keyvalue store"), nil
	}

	count, err := strconv.Atoi(prev.Value)
	if err != nil {
		count = 0
	}

	newValue := strconv.Itoa(count + 1)
	err = kv.Set(ctx, keyvalue.SetRequest{
		Key:     "tinygo:count",
		Value:   newValue,
		Expires: 0,
	})
	if err != nil {
		return Success("Couldn't set new value in keyvalue store"), err
	}
	return Success("Count: " + newValue), nil
}

// Helper function to construct a successful HTTP Response
func Success(msg string) *httpserver.HttpResponse {
	return &httpserver.HttpResponse{
		StatusCode: 200,
		Header:     make(httpserver.HeaderMap, 0),
		Body:       []byte(msg),
	}
}

// Helper function to construct a failed HTTP Response
func Failure(msg string) *httpserver.HttpResponse {
	return &httpserver.HttpResponse{
		StatusCode: 500,
		Header:     make(httpserver.HeaderMap, 0),
		Body:       []byte(msg),
	}
}
