package main

import (
	"github.com/wasmcloud/actor-tinygo"

	"github.com/wasmcloud/examples/cron/interface/tinygo"
	"github.com/wasmcloud/interfaces/logging/tinygo"
)

func main() {
	me := CronActor{
		Logger: logging.NewProviderLogging(),
	}
	actor.RegisterHandlers(cron.CronHandler(&me))
}

type CronActor struct {
	Logger *logging.LoggingSender
}

func (e *CronActor) TimedInvoke(ctx *actor.Context, req uint64) error {
	_ = e.Logger.WriteLog(ctx, logging.LogEntry{Level: "info", Text: "Timed Invoke!!"})
	return nil
}
