package main

import (
	kv "github.com/wasmcloud/interfaces/keyvalue/tinygo"
	msgpack "github.com/wasmcloud/tinygo-msgpack"
)

func decodeIncrementRequest(req []byte) kv.IncrementRequest {
	d := msgpack.NewDecoder(req)
	msg, _ := kv.MDecodeIncrementRequest(&d)
	return msg
}

func encodeIncrementResponse(value int32) ([]byte, error) {
	var sizer msgpack.Sizer
	size_enc := &sizer
	size_enc.WriteInt32(value)
	buf := make([]byte, sizer.Len())
	encoder := msgpack.NewEncoder(buf)
	encoder.WriteInt32(value)

	err := encoder.CheckError()
	if err != nil {
		p.Logger.Error(err, "Encoder error")
		return nil, err
	}

	return buf, nil
}
