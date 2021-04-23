package main

import (
	"strconv"
	"strings"

	core "github.com/wasmcloud/actor-interfaces/actor-core/go"
	httpserver "github.com/wasmcloud/actor-interfaces/http-server/go"
)

func wapc_init() {
}

func main() {
	core.Handlers{HealthRequest: healthRequest}.Register()
	httpserver.Handlers{HandleRequest: handleRequest}.Register()
}

func healthRequest(request core.HealthCheckRequest) (core.HealthCheckResponse, error) {
	return core.HealthCheckResponse{
		Healthy: true,
	}, nil
}

func handleRequest(request httpserver.Request) (httpserver.Response, error) {
	var ret string
	nums := strings.Split(request.QueryString, ",")

	num0, _ := strconv.Atoi(nums[0])
	num1, _ := strconv.Atoi(nums[1])

	switch path := request.Path; path {
	case "/add":
		ret = "add: " + nums[0] + " + " + nums[1] + " = " + strconv.Itoa(num0+num1)
	case "/sub":
		ret = "add: " + nums[0] + " - " + nums[1] + " = " + strconv.Itoa(num0-num1)
	// TODO: add multiplication
	case "/div":
		if num1 == 0 {
			ret = "You can not divide by 0!"
			break
		}
		ret = "add: " + nums[0] + " / " + nums[1] + " = " + strconv.Itoa(num0/num1)
	default:
		ret = "Welcome to the wasmcloud calculator app!"
	}

	return httpserver.Response{
		StatusCode: 200,
		Status:     "OK",
		Body:       []byte(ret),
	}, nil
}
