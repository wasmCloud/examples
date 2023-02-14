package main

import (
	"strings"

	"github.com/wasmcloud/actor-tinygo"
	httpserver "github.com/wasmcloud/interfaces/httpserver/tinygo"
	keyvalue "github.com/wasmcloud/interfaces/keyvalue/tinygo"
	logging "github.com/wasmcloud/interfaces/logging/tinygo"
)

var logger = logging.NewProviderLogging()

func main() {
	me := AsdbApi{}
	actor.RegisterHandlers(httpserver.HttpServerHandler(&me))
}

type AsdbApi struct{}

func (e *AsdbApi) HandleRequest(ctx *actor.Context, req httpserver.HttpRequest) (*httpserver.HttpResponse, error) {
	var resp httpserver.HttpResponse
	var err error

	header := httpserver.HeaderMap{
		"ContentType":                  httpserver.HeaderValues{"application/json"},
		"Access-Control-Allow-Origin":  httpserver.HeaderValues{"http://localhost:3000"},
		"Access-Control-Allow-Methods": httpserver.HeaderValues{"GET"},
		"Access-Control-Allow-Headers": httpserver.HeaderValues{"*"},
	}

	resp.Header = header

	switch {
	case req.Method == "GET" && req.Path == "/contacts":
		resp.Body, err = getData(ctx, "contacts")
		if err != nil {
			return nil, err
		}
		resp.StatusCode = 200
	case req.Method == "GET" && req.Path == "/stations":
		resp.Body, err = getData(ctx, "stations")
		if err != nil {
			return nil, err
		}
		resp.StatusCode = 200
	case req.Method == "GET" && req.Path == "/geojson":
		contacts, err := getList(ctx, "contacts")
		if err != nil {
			return nil, err
		}
		stations, err := getList(ctx, "stations")
		if err != nil {
			return nil, err
		}
		resp.Body, err = makeGeojson(ctx, contacts, stations)
		if err != nil {
			return nil, err
		}
		resp.StatusCode = 200
	default:
		resp.StatusCode = 404
		resp.Body = []byte("{\"error\":\"invalid request\"}")
	}

	return &resp, nil
}

func makeGeojson(ctx *actor.Context, items ...[]string) ([]byte, error) {
	var ret string

	retVals := []string{}
	kvstore := keyvalue.NewProviderKeyValue()
	for _, i := range items {
		for _, ii := range i {
			resp, err := kvstore.Get(ctx, ii)
			if err != nil {
				return nil, err
			}
			if resp.Exists {
				retVals = append(retVals, resp.Value)
			}
		}
	}

	ret += "{\"type\":\"FeatureCollection\",\"features\":["
	ret += strings.Join(retVals, ",")
	ret += "]}"

	return []byte(ret), nil
}

func getData(ctx *actor.Context, inDataType string) ([]byte, error) {
	contacts, err := getList(ctx, inDataType)
	if err != nil {
		return nil, err
	}

	retVals := []string{}
	kvstore := keyvalue.NewProviderKeyValue()
	for _, c := range contacts {
		resp, err := kvstore.Get(ctx, c)
		if err != nil {
			return nil, err
		}
		if resp.Exists {
			retVals = append(retVals, "\""+c+"\":"+resp.Value)
		}
	}

	var ret string
	ret += "{\"" + inDataType + "\":{" + strings.Join(retVals, ",") + "}}"

	return []byte(ret), nil
}

func getList(ctx *actor.Context, listName string) ([]string, error) {
	kvstore := keyvalue.NewProviderKeyValue()
	resp, err := kvstore.SetQuery(ctx, listName)
	if err != nil {
		return nil, err
	}

	activeContacts := []string{}
	for _, s := range *resp {
		exists, err := kvstore.Contains(ctx, s)
		if err != nil {
			return nil, err
		}

		if exists {
			activeContacts = append(activeContacts, s)
		} else {
			del := keyvalue.SetDelRequest{
				SetName: listName,
				Value:   s,
			}
			_, err = kvstore.SetDel(ctx, del)
			if err != nil {
				return nil, err
			}
		}
	}

	return activeContacts, nil
}
