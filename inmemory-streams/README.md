# In-Memory Streams

This is a **waSCC** capability provider for `wascc:eventstreams`, an abstraction around the concept of an append-only event stream service.

This provider exposes a transient, in-memory storage for streams (basically a vector of events) that can be used for acceptance and integration testing for actors.

## WARNING

This is not to be used in production, and is only to be used for testing purposes. This event stream provider makes none of the types of guarantees that you would want from a real, durable event stream.
