package cron

import (
	actor "github.com/wasmcloud/actor-tinygo"     //nolint
	msgpack "github.com/wasmcloud/tinygo-msgpack" //nolint
	//nolint
)

// The Cron service has a single method, timed_invoke, which
// invokes an actor after a specified interval
type Cron interface {
	// Invoked on an actor on the interval specified in the link
	TimedInvoke(ctx *actor.Context, arg uint64) error
}

// CronHandler is called by an actor during `main` to generate a dispatch handler
// The output of this call should be passed into `actor.RegisterHandlers`
func CronHandler(actor_ Cron) actor.Handler {
	return actor.NewHandler("Cron", &CronReceiver{}, actor_)
}

// CronContractId returns the capability contract id for this interface
func CronContractId() string { return "wasmcloud:example:cron" }

// CronReceiver receives messages defined in the Cron service interface
// The Cron service has a single method, timed_invoke, which
// invokes an actor after a specified interval
type CronReceiver struct{}

func (r *CronReceiver) Dispatch(ctx *actor.Context, svc interface{}, message *actor.Message) (*actor.Message, error) {
	svc_, _ := svc.(Cron)
	switch message.Method {

	case "TimedInvoke":
		{

			d := msgpack.NewDecoder(message.Arg)
			value, err_ := d.ReadUint64()
			if err_ != nil {
				return nil, err_
			}

			err := svc_.TimedInvoke(ctx, value)
			if err != nil {
				return nil, err
			}
			buf := make([]byte, 0)
			return &actor.Message{Method: "Cron.TimedInvoke", Arg: buf}, nil
		}
	default:
		return nil, actor.NewRpcError("MethodNotHandled", "Cron."+message.Method)
	}
}

// CronSender sends messages to a Cron service
// The Cron service has a single method, timed_invoke, which
// invokes an actor after a specified interval
type CronSender struct{ transport actor.Transport }

// NewActorSender constructs a client for actor-to-actor messaging
// using the recipient actor's public key
func NewActorCronSender(actor_id string) *CronSender {
	transport := actor.ToActor(actor_id)
	return &CronSender{transport: transport}
}

// Invoked on an actor on the interval specified in the link
func (s *CronSender) TimedInvoke(ctx *actor.Context, arg uint64) error {

	var sizer msgpack.Sizer
	size_enc := &sizer
	size_enc.WriteUint64(arg)
	buf := make([]byte, sizer.Len())

	var encoder = msgpack.NewEncoder(buf)
	enc := &encoder
	enc.WriteUint64(arg)

	s.transport.Send(ctx, actor.Message{Method: "Cron.TimedInvoke", Arg: buf})
	return nil
}

// This file is generated automatically using wasmcloud/weld-codegen 0.6.0
