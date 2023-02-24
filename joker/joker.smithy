metadata package = [ { namespace: "myorg.joker", crate: "joke" } ]

namespace myorg.joker

use org.wasmcloud.model#wasmbus
use org.wasmcloud.model#U32

@wasmbus( actorReceive: true )
service Joker {
  version: "0.1",
  operations: [ JokeMsgHandler ]
}

operation JokeMsgHandler {
  input: Blob,
  output: JokeMsg
}

structure JokeMsg {
  error: String,
  category: String,
  setup: String,
  delivery: String,
  flags: Flags,
  id: U32,
  safe: Boolean,
  lang: String
}

structure Flags {
  nsfw: Boolean,
  religious: Boolean,
  political: Boolean,
  racist: Boolean,
  sexist: Boolean,
  explicit: Boolean
}
