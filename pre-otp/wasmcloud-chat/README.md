# wasmcloud Chat

wasmcloud Chat is a reference application that serves to illustrate the process of going from sketch to scale with a relatively complex application. The chat application provides a multi-channel back-end that can be used to support virtually any form of messaging. The application allows users to join rooms for chats (e.g. "slack channels"), send and receive direct messages, and there is a robust presence system that allows users to log into the system from multiple channels (e.g. `telnet` and `web`) and the back-end handles routing and delivery properly.

wasmcloud Chat also has a fully functioning authentication system, showing one potential pattern for providing secure access to a wasmcloud application.

For a more thorough deep dive into the architecture and the entire development journey from concept to implementation, please check out the [reference applications](https://www.wasmcloud.dev/reference/refapps/) section of our documentation.

## Actors

The following is a list of the actors that make up the wasmcloud chat application:

### Channels

* [Telnet Channel](./actors/telnet-channel/README.md) - Manages the telnet comms channel
* [REST Channel](./actors/rest-channel/README.md) - Exposes a REST API that allows for sending and _querying_ messages, as well as potential admin functions like managing rooms. This channel is not real-time
* [NATS Channel](./actors/nats-channel/README.md) - Exposes a real-time comms channel via NATS, ideal for a WebSockets bridge for the web app

### Backend Core

* [Auth](./actors/auth/README.md) - Responsible for user authentication and authorization
* [Presence](./actors/presence/README.md) - Manages real-time user presence
* [Messages](./actors/messages/README.md) - Message publication, delivery, query, and persistence
* [Rooms](./actors/rooms/README.md) - CRUD operations on chat rooms

## Web Application (UI)

The web UI application for `wasmcloud.chat` is compromised of the following folders:

* [wasmcloud Chat](./wasmcloudchat/README.md) - Elixir Phoenix web application
