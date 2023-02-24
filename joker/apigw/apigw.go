package main

import (
	actor "github.com/wasmcloud/actor-tinygo"
	httpclient "github.com/wasmcloud/interfaces/httpclient/tinygo"
	httpserver "github.com/wasmcloud/interfaces/httpserver/tinygo"
)

const parserId string = "MBT7BNACBGZR3UFLXVPXR2LYRXPJICC26W72OMTEAOI2PQRNXQEFBULB"
const jokeUrl string = "https://v2.jokeapi.dev/joke/Programming?blacklistFlags=nsfw,religious,political,racist,sexist,explicit&type=twopart"

func main() {
	me := Apigw{}
	actor.RegisterHandlers(httpserver.HttpServerHandler(&me))
}

type Apigw struct{}

func (e *Apigw) HandleRequest(ctx *actor.Context, req httpserver.HttpRequest) (*httpserver.HttpResponse, error) {
	r := httpserver.HttpResponse{
		Header: httpserver.HeaderMap{
			"Content-Type": httpserver.HeaderValues{"application/json"},
		},
	}

	switch req.Path {
	case "/":
		client := httpclient.NewProviderHttpClient()
		resp, err := client.Request(ctx, httpclient.HttpRequest{Method: "GET", Url: jokeUrl, Headers: make(httpclient.HeaderMap, 0), Body: []byte("a")})
		if err != nil {
			r.Body = []byte("{\"error_1\":\"" + err.Error() + "\"}")
			r.StatusCode = 500
			break
		}

		joke, err := goGetParsed(ctx, resp.Body)
		if err != nil {
			r.Body = []byte("{\"error_2\":\"" + err.Error() + "\"}")
			r.StatusCode = 500
			break
		}

		if !joke.Flags.Nsfw && !joke.Flags.Religious && !joke.Flags.Political && !joke.Flags.Racist && !joke.Flags.Sexist && !joke.Flags.Explicit {
			r.Body = []byte("{\"joke\":\"" + joke.Setup + "\",\"answer\":\"" + joke.Delivery + "\"}")
			r.StatusCode = 200
		} else {
			r.Body = []byte("{\"msg\":\"we detected a inappropriate joke, please refresh to get a different one\"}")
			r.StatusCode = 200
		}

	default:
		r.Body = []byte("{\"error\":\"invalid endpoint\"}")
		r.StatusCode = 404
	}
	return &r, nil
}

func goGetParsed(ctx *actor.Context, data []byte) (*JokeMsg, error) {
	sender := NewActorJokerSender(parserId)
	return sender.JokeMsgHandler(ctx, data)
}
