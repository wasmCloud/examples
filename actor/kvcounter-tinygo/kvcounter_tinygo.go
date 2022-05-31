package main

import (
	"strconv"

	actor "github.com/wasmcloud/actor-tinygo"
	httpserver "github.com/wasmcloud/interfaces/httpserver/tinygo"
	keyvalue "github.com/wasmcloud/interfaces/keyvalue/tinygo"
)

func main() {
	actor.RegisterHandlers(httpserver.HttpServerHandler(&KvcounterTinygo{}))
}

type KvcounterTinygo struct{}

func (e *KvcounterTinygo) HandleRequest(ctx *actor.Context, req httpserver.HttpRequest) (*httpserver.HttpResponse, error) {
	kv := keyvalue.NewProviderKeyValue()

	newValue, err := kv.Increment(ctx, keyvalue.IncrementRequest{
		Key: "tinygo:count", Value: 1,
	})

	if err != nil {
		return Success("Couldn't set increment value in keyvalue store"), err
	}
	return Success("Count: " + strconv.FormatInt(int64(newValue), 10)), nil
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
