package pingpong

import (
	actor "github.com/wasmcloud/actor-tinygo"     //nolint
	msgpack "github.com/wasmcloud/tinygo-msgpack" //nolint
	//nolint
)

// Description of Pingpong service
type Pingpong interface {
	// Pings an actor, expecting a Pong in return
	Ping(ctx *actor.Context) (string, error)
}

// PingpongHandler is called by an actor during `main` to generate a dispatch handler
// The output of this call should be passed into `actor.RegisterHandlers`
func PingpongHandler(actor_ Pingpong) actor.Handler {
	return actor.NewHandler("Pingpong", &PingpongReceiver{}, actor_)
}

// PingpongReceiver receives messages defined in the Pingpong service interface
// Description of Pingpong service
type PingpongReceiver struct{}

func (r *PingpongReceiver) Dispatch(ctx *actor.Context, svc interface{}, message *actor.Message) (*actor.Message, error) {
	svc_, _ := svc.(Pingpong)
	switch message.Method {

	case "Ping":
		{
			resp, err := svc_.Ping(ctx)
			if err != nil {
				return nil, err
			}

			var sizer msgpack.Sizer
			size_enc := &sizer
			size_enc.WriteString(resp)
			buf := make([]byte, sizer.Len())
			encoder := msgpack.NewEncoder(buf)
			enc := &encoder
			enc.WriteString(resp)
			return &actor.Message{Method: "Pingpong.Ping", Arg: buf}, nil
		}
	default:
		return nil, actor.NewRpcError("MethodNotHandled", "Pingpong."+message.Method)
	}
}

// PingpongSender sends messages to a Pingpong service
// Description of Pingpong service
type PingpongSender struct{ transport actor.Transport }

// NewActorSender constructs a client for actor-to-actor messaging
// using the recipient actor's public key
func NewActorPingpongSender(actor_id string) *PingpongSender {
	transport := actor.ToActor(actor_id)
	return &PingpongSender{transport: transport}
}

// Pings an actor, expecting a Pong in return
func (s *PingpongSender) Ping(ctx *actor.Context) (string, error) {
	buf := make([]byte, 0)
	out_buf, _ := s.transport.Send(ctx, actor.Message{Method: "Pingpong.Ping", Arg: buf})
	d := msgpack.NewDecoder(out_buf)
	resp, err_ := d.ReadString()
	if err_ != nil {
		return "", err_
	}
	return resp, nil
}

// This file is generated automatically using wasmcloud/weld-codegen 0.6.0
