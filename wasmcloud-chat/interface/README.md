# Interface

Types and schemas shared among the actors and capability providers for wasmcloud chat.

**Note** that for simplicity's sake, all users are automatically a member of the room corresponding to their unique ID (private/DMs).

## Actor-to-Actor Calls

The following is a list of actor-to-actor interactions. The **target** of the operation is the actor's _call alias_.

| Origin | Target | Operation | Description |
| :--: | :--: | :--: | :-- |
| _Channel Actor_ | `messages` | `ProcessMessage` | Sent from channel to linked actor upon receipt of inbound message. If successful, results in publication of a chat message event (`MessagePublished`). Ack does **not** indicate delivery at destination. |
| _Channel Actor_ | `rooms` | `CreateRoom`<br/>`QueryRooms`<br/>`DeleteRoom`<br/>`QueryMembers` | Room operations performed by administrators. Note that `QueryRooms` can be used to get all rooms or a list of rooms to which a specific user belongs. |

## NATS Subjects

The following is a list of relevant NATS subjects used by the chat system

| Subject | Subscribers | Description |
| :-- | :--: | :-- |
| `wcc.events` | All Channel Actors | Global events that occur within the wasmcloud chat (WCC) environment |
| `wcc.events.rooms.*` | Channel Actors | Subscribed to when a user logs into a channel that is a member of a given room |

## Events

The following is a list of the events that can be published, either globally or within a room

| Event | Scope | Description |
| :-- | :--: | :-- |
| `UserLoggedIn` | Global | Indicates that a user has logged in _from a given channel_. Presence actor subscribes to this event |
| `UserLoggedOut` | Global | Indicates that a user terminated a connection _from a given channel_ (or was considered offline after timeout, etc). Presence actor subscribes to this event |
| `MessagePublished` | Room | Indicates that a message has been published to a room. Channel actors use this event to deliver content to users |
| `MessageDelivered` | Room | Published by channel actors to indicate that a published message has been delivered to a room |
| `UserJoined` | Room | Indicates a user joined the room |
| `UserLeft` | Room | Indicates a user left the room. This corresponds to permanently surrendering interest in the room, _not_ logging off |
