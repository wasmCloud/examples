package main

import (
	"errors"
	"sync"
	"time"

	provider "github.com/wasmCloud/provider-sdk-go"
)

var p *provider.WasmcloudProvider
var aID string
var data map[string]int32

var ErrNotImplemented error = errors.New("operation not implemented")
var ErrInvalidOperation error = errors.New("operation not valid")

func main() {
	var err error
	var wg sync.WaitGroup
	data = make(map[string]int32)

	p, err = provider.New("wasmcloud:keyvalue",
		provider.WithProviderActionFunc(handleKVAction),
	)
	if err != nil {
		panic(err)
	}
	err = p.Start()
	if err != nil {
		panic(err)
	}

	wg.Add(1)
	go func() {
		// TODO: move this to a cancel context
		time.Sleep(5 * time.Hour)
		wg.Done()
	}()
	wg.Wait()
}

func handleKVAction(a provider.ProviderAction) (*provider.ProviderResponse, error) {
	p.Logger.Info("Operation: " + a.Operation)
	resp := new(provider.ProviderResponse)
	var err error

	switch a.Operation {
	case "KeyValue.Increment":
		msg := decodeIncrementRequest(a.Msg)
		data[msg.Key]++
		resp.Msg, err = encodeIncrementResponse(data[msg.Key])
		if err != nil {
			return nil, err
		}
		return resp, nil
	case "KeyValue.Contains":
		return nil, ErrNotImplemented
	case "KeyValue.Del":
		return nil, ErrNotImplemented
	case "KeyValue.Get":
		return nil, ErrNotImplemented
	case "KeyValue.ListAdd":
		return nil, ErrNotImplemented
	case "KeyValue.ListClear":
		return nil, ErrNotImplemented
	case "KeyValue.ListDel":
		return nil, ErrNotImplemented
	case "KeyValue.ListRange":
		return nil, ErrNotImplemented
	case "KeyValue.Set":
		return nil, ErrNotImplemented
	case "KeyValue.SetAdd":
		return nil, ErrNotImplemented
	case "KeyValue.SetDel":
		return nil, ErrNotImplemented
	case "KeyValue.SetIntersection":
		return nil, ErrNotImplemented
	case "KeyValue.SetQuery":
		return nil, ErrNotImplemented
	case "KeyValue.SetUnion":
		return nil, ErrNotImplemented
	case "KeyValue.SetClear":
		return nil, ErrNotImplemented
	default:
		return nil, ErrInvalidOperation
	}
}
