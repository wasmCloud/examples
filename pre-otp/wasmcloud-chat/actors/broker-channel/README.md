# Message Broker Channel

[![](https://mermaid.ink/img/eyJjb2RlIjoic2VxdWVuY2VEaWFncmFtXG4gICAgcGFydGljaXBhbnQgRkIgYXMgRnJvbnRlbmQgQnJva2VyXG4gICAgcGFydGljaXBhbnQgQkMgYXMgQnJva2VyIENoYW5uZWxcbiAgICBwYXJ0aWNpcGFudCBNIGFzIE1lc3NhZ2VzXG4gICAgcGFydGljaXBhbnQgQkIgYXMgQmFja2VuZCBCcm9rZXJcblxuICAgIEZCLT4-K0JDOiBDaGF0IFJlcXVlc3RcbiAgICBCQy0-PitNOiBDaGFubmVsIE1lc3NhZ2VcbiAgICBNLS0-PkJCOiBQdWJsaXNoIEV2ZW50XG4gICAgTS0-Pi1CQzogQUNLXG4gICAgQkMtPj4tRkI6IEFDS1xuICAgIFxuICAgIEJCLS0-PitCQzogQ2hhdCBFdmVudFxuICAgIEJDLS0-Pi1GQjogTWVzc2FnZVxuICAgICAgICAgICAgIiwibWVybWFpZCI6e30sInVwZGF0ZUVkaXRvciI6ZmFsc2V9)](https://mermaid-js.github.io/mermaid-live-editor/#/edit/eyJjb2RlIjoic2VxdWVuY2VEaWFncmFtXG4gICAgcGFydGljaXBhbnQgRkIgYXMgRnJvbnRlbmQgQnJva2VyXG4gICAgcGFydGljaXBhbnQgQkMgYXMgQnJva2VyIENoYW5uZWxcbiAgICBwYXJ0aWNpcGFudCBNIGFzIE1lc3NhZ2VzXG4gICAgcGFydGljaXBhbnQgQkIgYXMgQmFja2VuZCBCcm9rZXJcblxuICAgIEZCLT4-K0JDOiBDaGF0IFJlcXVlc3RcbiAgICBCQy0-PitNOiBDaGFubmVsIE1lc3NhZ2VcbiAgICBNLS0-PkJCOiBQdWJsaXNoIEV2ZW50XG4gICAgTS0-Pi1CQzogQUNLXG4gICAgQkMtPj4tRkI6IEFDS1xuICAgIFxuICAgIEJCLS0-PitCQzogQ2hhdCBFdmVudFxuICAgIEJDLS0-Pi1GQjogTWVzc2FnZVxuICAgICAgICAgICAgIiwibWVybWFpZCI6e30sInVwZGF0ZUVkaXRvciI6ZmFsc2V9)

The message broker channel actor is one of the channel actors that provide a _proxy_ or _gateway_ to the back-end. In the case of
the broker channel, the front-end is a `wasmcloud:messaging` capability provider and the back-end is the `messages` actor.

This actor subscribes to message requests from the broker "front-end" and delivers them to the messages actor. It also subscribes to messages published on the back-end event subjects (`wcc.events.[user|room].*`), where the messages on the appropriate subjects will be forwarded to the appropriate front-end subject.

## Subject Topology

The following is a description of the relevant subjects used by this actor

* `wcc.frontend.requests` - Subscribed to by the actor via the `frontend` named capability provider. Messages received here are then used to invoke the `messages` capability provider, with an appropriate acknowledgement returned.
* `wcc.backend.events.[user|room].*` - Subscribed to by the actor via the `backend` named capability provider. Messages received here are then forwarded to the front-end by publication to the frontend events subject.
* `wcc.frontend.events.[user|room].*` - Subscribed to by all manner of front-end consumers. These subjects are where actual chat messages are published and then delivered to various user interfaces, e.g. via websockets or other proxies like Phoenix Liveview websites.

## Provider Linking

It is assumed that this actor must have _two_ capability provider links:

* `backend` - This capability provider is configured with a connection to the back-end message broker (NATS)
* `frontend` - This capability provider is configured with a connection to the front-end message broker (also NATS).

The use of two different connections means that the back-end and front-end message broker systems do not share a security domain and that the only entity capable of communicating on both of these message brokers at the same time is this one actor.

To illustrate the support for multiple different front-end consumers, the `MessageRequest` JSON message structure is _different_ than the message that is sent to the backend `messages` actor, making explicit use of the Anti-Corruption Layer pattern to isolate the different domains and prevent "bleeding" of logic and data across these boundaries.
