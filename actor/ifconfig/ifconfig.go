package main

import (
	"github.com/wasmcloud/actor-tinygo"
	httpclient "github.com/wasmcloud/interfaces/httpclient/tinygo"
	httpserver "github.com/wasmcloud/interfaces/httpserver/tinygo"
	logging "github.com/wasmcloud/interfaces/logging/tinygo"
)

func main() {
	me := Ifconfig{
		logger: logging.NewProviderLogging(),
	}

	actor.RegisterHandlers(httpserver.HttpServerHandler(&me))
}

type Ifconfig struct {
	logger *logging.LoggingSender
}

func (e *Ifconfig) HandleRequest(ctx *actor.Context, req httpserver.HttpRequest) (*httpserver.HttpResponse, error) {
	ip, err := GetIpAddress(ctx, e.logger)
	if err != nil {
		return nil, err
	}

	page := "<!DOCTYPE html><html><head><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1\"><title>Your IP Address</title></head><body>Your IP Address is: " + string(ip) + "</body></html>"

	r := httpserver.HttpResponse{
		StatusCode: 200,
		Header:     make(httpserver.HeaderMap, 0),
		Body:       []byte(page),
	}
	return &r, nil
}

func GetIpAddress(ctx *actor.Context, logger *logging.LoggingSender) ([]byte, error) {
	client := httpclient.NewProviderHttpClient()

	err := logger.WriteLog(ctx, logging.LogEntry{Level: "info", Text: "Making request to website"})
	if err != nil {
		return nil, err
	}
	resp, err := client.Request(ctx, httpclient.HttpRequest{
		Method: "GET",
		Url:    "https://ifconfig.io/ip",
		// Body can not be blank due to a bug
		Body: []byte("a"),
	})
	if err != nil {
		return nil, err
	}

	err = logger.WriteLog(ctx, logging.LogEntry{Level: "info", Text: "Request complete, returning body"})
	if err != nil {
		return nil, err
	}

	err = logger.WriteLog(ctx, logging.LogEntry{Level: "info", Text: string(resp.Body)})
	if err != nil {
		return nil, err
	}

	return resp.Body, nil
}
