package main

import (
	core "github.com/wasmcloud/actor-interfaces/actor-core/go"
	blob "github.com/wasmcloud/actor-interfaces/blobstore/go"
	httpserver "github.com/wasmcloud/actor-interfaces/http-server/go"
	"strings"
)

func main() {
	core.Handlers{HealthRequest: healthRequest}.Register()
	httpserver.Handlers{HandleRequest: handleRequest}.Register()
}

func handleRequest(request httpserver.Request) (httpserver.Response, error) {
	switch method := request.Method; method {
	case "GET":
		return downloadImage()
	case "POST":
		return uploadImage(request.Path, request.Body)
	default:
		return httpserver.Response{
			StatusCode: 400,
			Status:     "Bad Request",
			Body:       nil,
		}, nil
	}
}

func uploadImage(path string, imageBytes []byte) (httpserver.Response, error) {
	blobStore := blob.NewHost("default")
	container := blob.Container{
		ID: "wasmcloud-bucket",
	}
	image := blob.FileChunk{
		SequenceNo: 0,
		Container:  container,
		ID:         strings.ReplaceAll(path, "/", ""),
		TotalBytes: uint64(len(imageBytes)),
		ChunkSize:  uint64(len(imageBytes)),
		Context:    nil,
		ChunkBytes: imageBytes,
	}
	blobStore.StartUpload(image)
	blobStore.UploadChunk(image)
	return httpserver.Response{
		StatusCode: 200,
		Status:     "OK",
		Body:       []byte("Uploaded Successfully"),
	}, nil
}

func downloadImage() (httpserver.Response, error) {
	return httpserver.Response{
		StatusCode: 200,
		Status:     "OK",
		Body:       []byte("download not implemented"),
	}, nil
}

func healthRequest(request core.HealthCheckRequest) (core.HealthCheckResponse, error) {
	return core.HealthCheckResponse{
		Healthy: true,
	}, nil
}
