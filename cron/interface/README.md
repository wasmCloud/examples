# Interface for the Cron service, wasmcloud:example:cron

This interface specifies a single `actorReceive` function, `TimedInvoke`. It can be implemented in a variety of ways, but the intent is for the implementation to allow for specifying an interval that the capability provider will call the actor on. You can see an implementation of this using cron expressions in the parent folder [provider](../provider/).
