package main

import (
	"errors"
	"fmt"
	"sync"

	provider "github.com/wasmCloud/provider-sdk-go"
	core "github.com/wasmcloud/interfaces/core/tinygo"
)

type data map[string]interface{}

var (
	p        *provider.WasmcloudProvider
	database map[string]data
	lock     sync.RWMutex

	ErrNotImplemented   error = errors.New("operation not implemented")
	ErrInvalidOperation error = errors.New("operation not valid")
)

func main() {
	var err error
	database = make(map[string]data)
	lock = sync.RWMutex{}

	p, err = provider.New(
		"wasmcloud:keyvalue",
		provider.WithProviderActionFunc(handleKVAction),
		provider.WithNewLinkFunc(handleNewLink),
		provider.WithDelLinkFunc(handleDelLink),
		provider.WithHealthCheckMsg(healthCheckMsg),
	)
	if err != nil {
		panic(err)
	}

	err = p.Start()
	if err != nil {
		panic(err)
	}
}

func healthCheckMsg() string {
	return fmt.Sprintf("Num databases initialized: %d", len(database))
}

func handleDelLink(linkdef core.LinkDefinition) error {
	delete(database, linkdef.ActorId)
	return nil
}

func handleNewLink(linkdef core.LinkDefinition) error {
	newTable := make(map[string]interface{})
	database[linkdef.ActorId] = newTable
	return nil
}

func handleKVAction(a provider.ProviderAction) (*provider.ProviderResponse, error) {
	resp := new(provider.ProviderResponse)
	var err error

	switch a.Operation {
	case "KeyValue.Increment":
		msg := decodeIncrementRequest(a.Msg)

		// Get the actor specific database
		db := database[a.FromActor]

		if db[msg.Key] == nil {
			lock.Lock()
			db[msg.Key] = msg.Value
			lock.Unlock()
		} else {
			value, ok := db[msg.Key].(int32)
			if !ok {
				return nil, errors.New("key value not of int type")
			}

			lock.Lock()
			db[msg.Key] = value + msg.Value
			lock.Unlock()
		}

		resp.Msg, err = encodeIncrementResponse(db[msg.Key].(int32))
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
